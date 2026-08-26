//! What the consumer phase costs against the number of `stylex.*` calls a
//! module makes, and -- the point of a series rather than one size -- whether
//! that cost is linear in it.
//!
//! Every call the transform rewrites asks the state manager which recorded entry
//! holds it: which declarator it initialises, which style variable it is bound
//! to, which top-level expression it is. Answered by walking the collection and
//! comparing whole `CallExpr` subtrees with `eq_ignore_span`, each of those runs
//! once per call against every entry -- `O(calls x entries)`, which on a module
//! of one component per `stylex.create` is genuinely quadratic in the file. They
//! are now answered from keys, and a return to the old shape shows up here as a
//! rising cost per component where a flat curve keeps it level.
//!
//! **Read the per-component cost, not the total.** Doubling the components
//! doubles the work a linear transform does, so a total that doubles is the
//! result being asserted; it is the total growing by *more* than the doubling
//! that says a quadratic is back. What these three legs measured when this was
//! written, as the leg's median divided by its components:
//!
//! ```text
//!   components   walked    keyed
//!           25   76.8 µs   65.9 µs
//!          100   71.7 µs   65.7 µs
//!          400   90.3 µs   67.7 µs
//! ```
//!
//! The left column is the curve this file exists to catch: level to 100 and then
//! climbing, because the quadratic term only overtakes the linear one once the
//! collections are long enough. The right one stays flat. What is left in it is
//! the fixed cost every transform pays, which is why it is not perfectly so.
//!
//! The legs stop at 400 because that is the smallest series that separates the
//! two columns, and a larger one would cost every run more time to say what this
//! one says. The effect keeps growing past it: measured outside this repository
//! against generated modules of 375 to 3000 components, per-component cost went
//! 83.5, 105.2, 146.9 and 220.3 µs walked against 70.7, 72.3, 79.8 and 81.9 µs
//! keyed -- 2.7x on the whole transform at the top. Those files are not built
//! here, so that pair is a development observation rather than a number a later
//! reader can re-run; what *is* re-runnable is the three legs above.
//!
//! The module is generated rather than cut from a real file because the shape
//! being measured is *many* calls: the repository's large fixture is one long
//! array of `stylex.create` calls that declares almost nothing and reads almost
//! nothing, so it exercises the producer half and asks these questions barely at
//! all. Every component here declares its own styles and reads them from nine
//! `stylex.props` sites, which is the ratio a component file has.
//!
//! What is timed is one `Program::apply` of the StyleX pass. Parsing is setup
//! and the clone the pass consumes is batched out, so the number is the
//! transform rather than a whole compile.
//!
//! **Every benchmark here runs inside `GLOBALS.set`**, for the reason
//! `transform_debug_bench` states at length and `guidelines/PERFORMANCE.md`
//! repeats: `into_pass` calls `Mark::new()`, which panics outside a `GLOBALS`
//! scope, and the transform's panic boundary swallows it -- so a bench without
//! one times a panic and its unwind and reports the swallowed path's regressions
//! as improvements.

use std::{
  hint::black_box,
  path::{Path, PathBuf},
  rc::Rc,
  sync::Arc,
};

use criterion::{BatchSize, Criterion, criterion_group, criterion_main};
use stylex_structures::stylex_options::ModuleResolution;
use stylex_transform::StyleXTransform;
use swc_core::{
  common::{
    FileName, GLOBALS, Globals, SourceMap, comments::SingleThreadedComments, input::StringInput,
    sync::Lrc,
  },
  ecma::{
    ast::{EsVersion, JSXAttr, JSXAttrName, JSXAttrValue, Program},
    parser::{Parser, Syntax, TsSyntax, lexer::Lexer},
    visit::{Visit, VisitWith},
  },
};

/// The component counts measured.
///
/// Four times the size across the series, which is what separates a rising
/// per-component cost from a level one. The header says why it stops at 400.
const COMPONENT_COUNTS: [usize; 3] = [25, 100, 400];

/// How many `stylex.props` sites each component reads its styles from.
///
/// The ratio that decides what this file measures: the lookups being timed are
/// per *call*, and a component that declares one `create` and reads it once
/// would spend most of its time in the producer half instead. Nine is what a
/// moderately styled component element tree spells.
const PROPS_PER_COMPONENT: usize = 9;

/// The attribute a resolved `stylex.props` spread becomes, counted to prove the
/// transform did the work rather than refusing it.
const RESOLVED_ATTR: &str = "className";

/// A module of `components` components, each declaring its own styles and
/// reading them from [`PROPS_PER_COMPONENT`] sites.
///
/// Every component's styles are distinct, because identical `create` calls
/// collapse onto one entry in the state manager's call map and would make the
/// collections this benchmark is about far smaller than the file suggests.
fn generated_source(components: usize) -> String {
  let mut source = String::from("import * as stylex from '@stylexjs/stylex';\n\n");

  for index in 0..components {
    source.push_str(&format!(
      "const styles{index} = stylex.create({{\n\
       \x20 base: {{ color: 'rgb({index}, 0, 0)', paddingTop: {index} }},\n\
       \x20 alt: {{ backgroundColor: 'rgb(0, {index}, 0)', marginBottom: {index} }},\n\
       }});\n"
    ));

    source.push_str(&format!(
      "export function Component{index}() {{\n  return (\n    <div>\n"
    ));

    for _ in 0..PROPS_PER_COMPONENT {
      source.push_str(&format!(
        "      <span {{...stylex.props(styles{index}.base, styles{index}.alt)}} />\n"
      ));
    }

    source.push_str("    </div>\n  );\n}\n");
  }

  source
}

/// One size of the generated module: its path, and the parsed program.
struct Module {
  components: usize,
  path: PathBuf,
  program: Program,
}

fn build_module(components: usize) -> Module {
  let source = generated_source(components);

  // Named after the size, because the code frame's source map is process-global
  // and keyed by file name: two sizes sharing one name would share one
  // registered source. Nothing here writes the file -- production transforms
  // resolve no positions, and the memoized module is what the path names.
  let path = PathBuf::from(format!("/generated/components{components}.tsx"));
  let program = parse(&FileName::Real(path.clone()), &source);

  Module {
    components,
    path,
    program,
  }
}

fn parse(file_name: &FileName, source: &str) -> Program {
  let cm: Lrc<SourceMap> = Default::default();
  let fm = cm.new_source_file(Arc::new(file_name.clone()), source.to_owned());
  let lexer = Lexer::new(
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    EsVersion::EsNext,
    StringInput::from(&*fm),
    None,
  );
  let mut parser = Parser::new_from(lexer);

  match parser.parse_program() {
    Ok(program) => program,
    Err(error) => panic!("Failed to parse {}: {:#?}", file_name, error),
  }
}

/// The transform under measurement, configured as a production build configures
/// it. `dev` is off: it turns on the position lookup, which is several times the
/// whole production transform and is what `transform_debug_bench` measures.
fn transform(path: &Path, program: Program) -> Program {
  let comments = Rc::new(SingleThreadedComments::default());

  let pass = StyleXTransform::test(comments)
    .with_filename(FileName::Real(path.to_path_buf()))
    .with_dev(false)
    .with_treeshake_compensation(true)
    .with_unstable_module_resolution(ModuleResolution::haste(None))
    .with_enable_minified_keys(false)
    .with_runtime_injection()
    .into_pass();

  program.apply(pass)
}

/// Counts the `className` attributes a resolved `stylex.props` spread leaves
/// behind.
///
/// Keyed on a string-valued attribute, because a spread the transform could not
/// resolve stays a spread of a runtime call and leaves none: this counts
/// resolutions, not elements.
#[derive(Default)]
struct ResolvedPropsCounter {
  resolved: usize,
}

impl Visit for ResolvedPropsCounter {
  fn visit_jsx_attr(&mut self, attr: &JSXAttr) {
    if matches!(&attr.name, JSXAttrName::Ident(name) if name.sym.as_ref() == RESOLVED_ATTR)
      && matches!(&attr.value, Some(JSXAttrValue::Str(_)))
    {
      self.resolved += 1;
    }

    attr.visit_children_with(self);
  }
}

/// Panics unless the transform resolved every `stylex.props` site in the module.
///
/// A refusal, a deopt and a swallowed panic are all fast, and a curve that
/// flattens because the work stopped happening is indistinguishable from a win.
/// The props sites are what asks the lookups being timed, so they are what has
/// to have happened.
fn assert_resolves_every_props_site(module: &Module) {
  let mut counter = ResolvedPropsCounter::default();

  transform(&module.path, module.program.clone()).visit_with(&mut counter);

  let expected = module.components * PROPS_PER_COMPONENT;

  assert_eq!(
    counter.resolved, expected,
    "a {}-component module resolved {} of its {expected} `stylex.props` sites -- the benchmark \
     below would be timing a refusal rather than the transform",
    module.components, counter.resolved
  );
}

fn consumer_phase_benchmarks(c: &mut Criterion) {
  let globals = Globals::default();

  GLOBALS.set(&globals, || {
    let modules: Vec<Module> = COMPONENT_COUNTS.into_iter().map(build_module).collect();

    for module in &modules {
      assert_resolves_every_props_site(module);
    }

    let mut group = c.benchmark_group("TransformConsumers");

    for module in &modules {
      // Batched because `apply` consumes the program: the clone is setup, not
      // work under measurement.
      group.bench_function(format!("components/{}", module.components), |b| {
        b.iter_batched(
          || module.program.clone(),
          |program| black_box(transform(black_box(&module.path), program)),
          BatchSize::SmallInput,
        )
      });
    }

    group.finish();
  });
}

criterion_group!(benches, consumer_phase_benchmarks);
criterion_main!(benches);
