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
//! that says a quadratic is back. What these legs measured when this was
//! written, as the leg's median divided by its components. `walked` is the same
//! file run against the commit before the keys, so the two columns differ in the
//! lookups alone:
//!
//! ```text
//!   components   walked    keyed     identical styles   walked    keyed
//!           25   54.3 µs   51.9 µs                 25   47.4 µs   45.2 µs
//!          100   60.6 µs   56.0 µs                100   50.8 µs   46.1 µs
//!          400   78.9 µs   59.1 µs                400   68.5 µs   51.0 µs
//! ```
//!
//! The `walked` columns are the curve this file exists to catch: level to 100
//! and then climbing, because the quadratic term only overtakes the linear one
//! once the collections are long enough. The `keyed` ones stay near flat. What
//! is left in them is the fixed cost every transform pays -- parsing aside, the
//! producer half that turns each style object into CSS -- which is why they are
//! not perfectly so, and why the whole transform gains 1.3x at 400 rather than
//! the multiple the lookups themselves do.
//!
//! [`assert_cost_per_component_stays_flat`] reads that curve for you, so a
//! regression fails the run instead of waiting in a criterion report for a human
//! to open it.
//!
//! Both shapes are measured because they load the keys differently: see
//! [`Styles`]. The legs stop at 400 because that is the smallest series that
//! separates the columns, and a larger one would cost every run more time to say
//! what this one says. The effect keeps growing past it: measured outside this
//! repository against generated modules of 375 to 3000 components, per-component
//! cost went 83.5, 105.2, 146.9 and 220.3 µs walked against 70.7, 72.3, 79.8 and
//! 81.9 µs keyed -- 2.7x on the whole transform at the top. Those files are not
//! built here, and that pair predates the allocator below, so it is a
//! development observation rather than a number a later reader can re-run; what
//! *is* re-runnable is the legs above.
//!
//! Every number here was taken with the benches linking the same allocator the
//! shipped `.node` does. A run under the system allocator reads slower
//! throughout and reports an allocation-reducing change as a larger win than
//! consumers get.
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

// Same allocator the `.node` links, so a malloc-bound measurement here is the
// one consumers get. Linked for its `#[global_allocator]` and nothing else.
use swc_malloc as _;

use std::{
  hint::black_box,
  path::{Path, PathBuf},
  rc::Rc,
  sync::Arc,
  time::{Duration, Instant},
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

/// How a generated module spells the styles of its components.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Styles {
  /// Every component's `create` call reads differently, so each lands in a
  /// bucket of its own. What a component file writes, and the shape the keyed
  /// lookups are claimed to be linear in.
  Distinct,
  /// Every component's `create` call reads the same, so all of them share one
  /// bucket. The shape a key cannot narrow, which real code does write and which
  /// the `structurally_identical_calls` fixtures pin the *output* of.
  ///
  /// Measured because narrowing to a bucket of N leaves the confirm to decide
  /// between N, where the walk this replaced stopped at the first entry -- every
  /// one of them matches. Confirming the whole bucket to learn what its first
  /// entry already said cost 1.7x the walk here before `earliest_confirmed`
  /// learned to stop at the first entry of a bucket nothing has moved.
  Identical,
}

impl Styles {
  /// What the component at `index` names its colour channel.
  ///
  /// The index is what separates one component's `create` call from another's,
  /// so dropping it is the whole of what collapses the module onto one bucket.
  fn channel(self, index: usize) -> String {
    match self {
      Styles::Distinct => index.to_string(),
      Styles::Identical => String::from("0"),
    }
  }

  /// The name of the leg this shape measures.
  fn leg(self) -> &'static str {
    match self {
      Styles::Distinct => "components",
      Styles::Identical => "identicalStyles",
    }
  }
}

/// A module of `components` components, each declaring its own styles and
/// reading them from [`PROPS_PER_COMPONENT`] sites.
///
/// Each component still declares and reads a style variable of its own whatever
/// `styles` says, so the number of lookups is the same across the two shapes and
/// only the number of buckets they answer from differs.
fn generated_source(components: usize, styles: Styles) -> String {
  let mut source = String::from("import * as stylex from '@stylexjs/stylex';\n\n");

  for index in 0..components {
    let channel = styles.channel(index);

    source.push_str(&format!(
      "const styles{index} = stylex.create({{\n\
       \x20 base: {{ color: 'rgb({channel}, 0, 0)', paddingTop: {channel} }},\n\
       \x20 alt: {{ backgroundColor: 'rgb(0, {channel}, 0)', marginBottom: {channel} }},\n\
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
  styles: Styles,
  path: PathBuf,
  program: Program,
}

fn build_module(components: usize, styles: Styles) -> Module {
  let source = generated_source(components, styles);

  // Named after the size and the shape, because the code frame's source map is
  // process-global and keyed by file name: two modules sharing one name would
  // share one registered source. Nothing here writes the file -- production
  // transforms resolve no positions, and the memoized module is what the path
  // names.
  let path = PathBuf::from(format!("/generated/{}{components}.tsx", styles.leg()));
  let program = parse(&FileName::Real(path.clone()), &source);

  Module {
    components,
    styles,
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

/// How many times each leg is transformed to price it.
///
/// The cheapest run of the several is what is compared, because a run can only
/// be made *slower* by something outside the transform -- a scheduler, a
/// competing process, a page fault -- so the minimum is the closest a wall clock
/// gets to the work itself.
const PRICING_RUNS: usize = 7;

/// How much the per-component cost may rise across the series before the rise is
/// called a regression.
///
/// Set from what both sides measure through [`cost_per_component`], not from the
/// criterion medians in the header. Over three runs of each, the commit before
/// the keys rose 1.43x to 1.57x, and this one rises 1.05x to 1.17x. The bound
/// sits between them with about a tenth of headroom on each side, which the
/// cheapest of [`PRICING_RUNS`] runs is stable enough to keep: the widest spread
/// seen on either side of it was 0.06x.
///
/// Below the whole cost of the walk, because most of what a leg spends is the
/// producer half that both share. What is being bounded is the rise across the
/// series, not the distance between the two implementations.
const MAX_PER_COMPONENT_RISE: f64 = 1.30;

/// What one component of `module` costs to transform, in seconds.
fn cost_per_component(module: &Module) -> f64 {
  let mut cheapest = Duration::MAX;

  for _ in 0..PRICING_RUNS {
    let program = module.program.clone();
    let started = Instant::now();

    black_box(transform(&module.path, program));

    cheapest = cheapest.min(started.elapsed());
  }

  cheapest.as_secs_f64() / module.components as f64
}

/// Panics unless the transform costs about the same per component at every size.
///
/// The claim the whole file exists for. A linear transform spends a flat amount
/// per component whatever the module holds, so the largest leg may not cost
/// [`MAX_PER_COMPONENT_RISE`] times the smallest one per component; a quadratic
/// one climbs, because the lookups it makes per call grow with the collections
/// they walk.
///
/// Each shape against itself, because the two start from different fixed costs:
/// what is being asserted is that a shape stays level as it grows, not that the
/// shapes cost alike. Only the smallest and largest leg of each shape is priced,
/// so this adds four legs of [`PRICING_RUNS`] transforms -- about a third of a
/// second -- to the run.
///
/// Outside the timed region, so criterion measures the transform rather than
/// this.
fn assert_cost_per_component_stays_flat(modules: &[Module]) {
  for styles in [Styles::Distinct, Styles::Identical] {
    let mut legs = modules.iter().filter(|module| module.styles == styles);

    let (Some(smallest), Some(largest)) = (legs.next(), legs.next_back()) else {
      continue;
    };

    let rise = cost_per_component(largest) / cost_per_component(smallest);

    assert!(
      rise < MAX_PER_COMPONENT_RISE,
      "{} cost rose {rise:.2}x per component from {} components to {} -- a linear \
       transform keeps that flat, so a lookup is walking again",
      styles.leg(),
      smallest.components,
      largest.components
    );
  }
}

fn consumer_phase_benchmarks(c: &mut Criterion) {
  let globals = Globals::default();

  GLOBALS.set(&globals, || {
    let modules: Vec<Module> = [Styles::Distinct, Styles::Identical]
      .into_iter()
      .flat_map(|styles| {
        COMPONENT_COUNTS
          .into_iter()
          .map(move |components| build_module(components, styles))
      })
      .collect();

    // A full extra transform of every leg, the largest included, before
    // anything is measured. Deliberate: what it buys is the guarantee that the
    // numbers below price the transform rather than a refusal, and it costs
    // startup time only.
    for module in &modules {
      assert_resolves_every_props_site(module);
    }

    assert_cost_per_component_stays_flat(&modules);

    let mut group = c.benchmark_group("TransformConsumers");

    for module in &modules {
      // Batched because `apply` consumes the program: the clone is setup, not
      // work under measurement.
      group.bench_function(
        format!("{}/{}", module.styles.leg(), module.components),
        |b| {
          b.iter_batched(
            || module.program.clone(),
            |program| black_box(transform(black_box(&module.path), program)),
            BatchSize::SmallInput,
          )
        },
      );
    }

    group.finish();
  });
}

criterion_group!(benches, consumer_phase_benchmarks);
criterion_main!(benches);
