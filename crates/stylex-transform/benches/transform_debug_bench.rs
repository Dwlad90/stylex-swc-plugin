//! What a `dev` build's `file:line` annotation costs, and how that cost scales
//! with the size of the file it annotates.
//!
//! `enable_debug_data_prop` defaults to on and `dev` implies `debug`, so every
//! development build resolves the authored position of every style namespace and
//! writes it into `$$css` as `file:line`. Resolving one position means locating
//! the namespace key inside the module's own parsed source, which is the most
//! expensive thing a `dev` transform does -- it was about 85% of one and grew
//! roughly with the square of the file size until the per-lookup deep clone and
//! then the per-key walk behind it were removed. Nothing in this repo measured
//! that, so each improvement was unguarded and the next change to the path would
//! have reported as nothing.
//!
//! Two things are pinned here, and the second is the reason the sizes come in a
//! series rather than one at a time:
//!
//! - the absolute cost of a `dev` transform, against the same file transformed
//!   with `dev` off -- the penalty a developer waits on, and the number a change
//!   to the debug path moves;
//! - the *shape* of that cost against file size. Locating a namespace key used
//!   to be one whole-program walk per key, which is `O(namespaces x file size)`
//!   -- genuinely quadratic in a file that is one long list of styles. It is now
//!   answered from an index built in one walk per module, and a return to the
//!   old shape shows up here as a rising cost per create, where a flat curve
//!   keeps it level.
//!
//! What is timed is one `Program::apply` of the StyleX pass. Parsing the module
//! is setup and the clone the pass consumes is batched out, so the number is the
//! transform rather than a whole compile: no lex, no codegen, no CSS emission.
//! The `file:line` lookup *does* re-read and re-parse the module from disk on
//! its first miss, once per transform, because that is what the compiler does
//! too -- it is part of what the annotation costs, not harness overhead.
//!
//! **Every benchmark here runs inside `GLOBALS.set`, and so must anything else
//! that touches the transform.** `into_pass` and the code-frame path both call
//! `Mark::new()`, which panics outside a `GLOBALS` scope. Inside the transform
//! that panic is swallowed by the diagnostic panic boundary, so a bench without
//! `GLOBALS` still produces a number -- it just times a panic and its unwind
//! instead of the work, and reports a regression in the swallowed path as an
//! improvement. That mistake inflated an earlier attribution of this very path
//! by 3.6x. See `guidelines/PERFORMANCE.md`.

use std::{
  fs,
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
    ast::{EsVersion, Expr, KeyValueProp, Lit, Program, PropName},
    parser::{Parser, Syntax, TsSyntax, lexer::Lexer},
    visit::{Visit, VisitWith},
  },
};

/// The `$$css` marker the debug annotation is written into.
const COMPILED_KEY: &str = "$$css";

/// The create counts measured. The committed fixture holds the largest of them;
/// the smaller ones are cut from it, so all three are the same styles in the
/// same order and differ only in how many.
///
/// Four times the size across the series reads a rising per-create cost apart
/// from a level one, which is what a quadratic term looks like once it is large
/// enough to dominate.
///
/// It is not large enough at these sizes, and that is worth stating rather
/// than implying: while the per-key walk was still there, the per-create cost
/// across this series was 235, 212 and 251 µs -- flat, because the quadratic
/// term was only about a seventh of a 100-create transform. What guards the
/// path here is the paired `dev` and `prod` legs at one size: putting the walk
/// back takes dev/100 from 9.1 ms to 25.1 ms against an unchanged prod/100.
/// Reading the *curve* takes files four to sixteen times this one, which the
/// committed fixture deliberately does not hold; those measurements live in the
/// tracker instead.
///
/// The series stops at 100 because that is the smallest slice that showed the
/// effect it was cut to show, and 200 and 400 would only cost every run more
/// time to say what 100 already says.
const CREATE_COUNTS: [usize; 3] = [25, 50, 100];

/// The marker every element of the fixture's array begins with. Matched as a
/// substring rather than as a whole line so reindenting the fixture cannot
/// silently change what gets cut, and every cut is checked afterwards by
/// [`assert_slice_is_well_formed`] regardless.
const CREATE_MARKER: &str = "stylex.create(";

/// The committed fixture: the first 100 `stylex.create` calls of
/// `apps/rollup-large-example/lotsOfStyles.js`, which is one long array of them.
///
/// A real file rather than a generated one, because the cost being measured is
/// the cost of finding a namespace key in authored source: real styles bring
/// shorthands that expand, media queries, and `var()` references, and a
/// generated file of identical namespaces would also make every key lookup
/// ambiguous, which is a different path.
fn fixture_path() -> PathBuf {
  PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("benches/fixtures/lotsOfStyles100.js")
}

/// Where a cut-down slice is written so the code-frame path can read it.
///
/// It has to be a real file: resolving a namespace key's position re-reads the
/// module by the filename the transform was given. `CARGO_TARGET_TMPDIR` is
/// cargo's own scratch directory for dev targets, so the slices land beside the
/// build artifacts rather than in the source tree or in a shared `/tmp`.
fn slice_dir() -> PathBuf {
  PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("transform-debug-bench")
}

fn read_fixture() -> String {
  let path = fixture_path();

  match fs::read_to_string(&path) {
    Ok(source) => source,
    Err(error) => panic!("Failed to read {}: {}", path.display(), error),
  }
}

/// The number of `stylex.create` calls in `source`.
fn count_creates(source: &str) -> usize {
  source.matches(CREATE_MARKER).count()
}

/// The byte offset of the line that the `index`-th (0-based) `stylex.create`
/// call starts on, or `None` when there is no such call.
///
/// The line start rather than the call itself, so a cut there drops the whole
/// array element including its indentation instead of leaving half a line
/// behind.
fn create_line_start(source: &str, index: usize) -> Option<usize> {
  let call = source.match_indices(CREATE_MARKER).nth(index)?.0;

  Some(match source[..call].rfind('\n') {
    Some(newline) => newline + 1,
    None => 0,
  })
}

/// The first `creates` elements of the fixture's array, as a module of its own.
///
/// Cutting at a line start and closing the array is the whole transformation:
/// the fixture is one `export const … = [ stylex.create({…}), … ];`, so every
/// prefix of its elements is a valid module once the array and the statement are
/// closed again.
fn slice_source(source: &str, creates: usize) -> String {
  let available = count_creates(source);

  assert!(
    creates <= available,
    "asked for {creates} creates from a fixture that holds {available}"
  );

  if creates == available {
    return source.to_owned();
  }

  let cut = match create_line_start(source, creates) {
    Some(cut) => cut,
    // Unreachable while `creates < available`: there is an element after the
    // last one kept. Stated rather than unwrapped so a future edit to the
    // counting above fails with a sentence instead of a panic message.
    None => panic!("the fixture holds {available} creates but no {creates}-th one to cut at"),
  };

  format!("{}];\n", &source[..cut])
}

/// Panics unless `source` is a module of exactly `creates` `stylex.create`
/// calls.
///
/// The cut above is textual, so this is what stands behind it: a fixture
/// reflowed onto different lines, or an array closed in a way the cut does not
/// expect, fails here rather than silently benchmarking a smaller file or a
/// parse error. The parse is the same syntax the compiler uses.
fn assert_slice_is_well_formed(source: &str, creates: usize) {
  let found = count_creates(source);

  assert_eq!(
    found, creates,
    "a {creates}-create slice holds {found} `stylex.create` calls"
  );

  let module = parse(&FileName::Anon, source);

  assert!(
    module
      .as_module()
      .is_some_and(|module| !module.body.is_empty()),
    "a {creates}-create slice parsed to an empty module"
  );
}

/// One size of the fixture: its source, on disk under the path the transform
/// will be told about, already parsed.
struct Slice {
  creates: usize,
  path: PathBuf,
  program: Program,
}

/// Cuts `creates` elements out of `source`, writes the result to disk, and
/// parses it.
///
/// Named for the write, because there is one: the position lookup re-reads the
/// module by the filename the transform was given, so a slice that exists only
/// in memory resolves nothing.
fn materialize_slice(source: &str, creates: usize) -> Slice {
  let sliced = slice_source(source, creates);

  assert_slice_is_well_formed(&sliced, creates);

  let dir = slice_dir();

  if let Err(error) = fs::create_dir_all(&dir) {
    panic!("Failed to create {}: {}", dir.display(), error);
  }

  // Named after the size rather than after the fixture, because the code-frame
  // source map is process-global and keyed by file name: two slices sharing one
  // name would share one registered source, and every position resolved for the
  // second would be looked up in the first.
  let path = dir.join(format!("lotsOfStyles{creates}.js"));

  if let Err(error) = fs::write(&path, &sliced) {
    panic!("Failed to write {}: {}", path.display(), error);
  }

  let program = parse(&FileName::Real(path.clone()), &sliced);

  Slice {
    creates,
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

/// The transform under measurement, configured the way a development build
/// configures it.
///
/// `dev` is the switch this file exists for: it implies `debug`, and `debug`
/// with the default `enable_debug_data_prop` is what turns on the position
/// lookup. Everything else matches the fixture suite's `dev` leg so a number
/// here and a snapshot there describe the same transform.
fn transform(path: &Path, program: Program, dev: bool) -> Program {
  let comments = Rc::new(SingleThreadedComments::default());

  let pass = StyleXTransform::test(comments)
    .with_filename(FileName::Real(path.to_path_buf()))
    .with_dev(dev)
    .with_treeshake_compensation(true)
    .with_unstable_module_resolution(ModuleResolution::haste(None))
    .with_enable_minified_keys(false)
    .with_runtime_injection()
    .into_pass();

  program.apply(pass)
}

/// Counts `$$css` properties whose value is a resolved `file:line`.
///
/// A position that cannot be resolved degrades to `$$css: true` rather than
/// failing, so counting the resolved ones is the only way to tell a measured
/// lookup from a measured refusal.
///
/// Keyed on the property rather than on the string, so an unrelated literal
/// that happens to end in `:<digits>` -- a `grid-area`, a `background`
/// shorthand, a `url()` with a port -- cannot stand in for an annotation.
#[derive(Default)]
struct AnnotationCounter {
  resolved: usize,
}

impl Visit for AnnotationCounter {
  fn visit_key_value_prop(&mut self, prop: &KeyValueProp) {
    if is_compiled_key(&prop.key)
      && let Expr::Lit(Lit::Str(value)) = prop.value.as_ref()
      // `None` for a string holding an unpaired surrogate, which no annotation
      // this counts can be: they are built from a file path and a line number.
      && value.value.as_str().is_some_and(is_file_line_annotation)
    {
      self.resolved += 1;
    }

    prop.visit_children_with(self);
  }
}

/// Whether `key` is the `$$css` marker, quoted or not.
fn is_compiled_key(key: &PropName) -> bool {
  match key {
    PropName::Ident(ident) => ident.sym.as_ref() == COMPILED_KEY,
    PropName::Str(value) => value.value.as_str() == Some(COMPILED_KEY),
    _ => false,
  }
}

/// Whether `value` looks like `<something>:<line number>`.
fn is_file_line_annotation(value: &str) -> bool {
  match value.rsplit_once(':') {
    Some((file, line)) => {
      !file.is_empty() && !line.is_empty() && line.bytes().all(|byte| byte.is_ascii_digit())
    },
    None => false,
  }
}

fn count_resolved_annotations(program: &Program) -> usize {
  let mut counter = AnnotationCounter::default();
  program.visit_with(&mut counter);
  counter.resolved
}

/// Panics unless transforming `slice` with `dev` on annotated every namespace it
/// compiled with a resolved `file:line`.
///
/// Without it this file would happily report the cost of *not* finding any
/// position -- which is what it looks like when the fixture stops resolving, when
/// the panic boundary starts swallowing every lookup, or when `GLOBALS` is
/// missing. A flat, fast, meaningless curve is the failure mode being ruled out,
/// and it is indistinguishable from a win.
fn assert_annotates_every_namespace(slice: &Slice) {
  let dev = count_resolved_annotations(&transform(
    &slice.path,
    slice.program.clone(),
    /* dev */ true,
  ));

  assert!(
    dev >= slice.creates,
    "a {}-create slice resolved {dev} `file:line` annotations, fewer than one per create -- \
     the benchmark below would be timing failed lookups rather than the debug path",
    slice.creates
  );

  let prod = count_resolved_annotations(&transform(
    &slice.path,
    slice.program.clone(),
    /* dev */ false,
  ));

  assert_eq!(
    prod, 0,
    "a {}-create slice annotated {prod} positions with `dev` off, so the two legs below \
     are not measuring the presence and absence of the debug path",
    slice.creates
  );
}

fn debug_path_benchmarks(c: &mut Criterion) {
  let globals = Globals::default();

  GLOBALS.set(&globals, || {
    let source = read_fixture();
    let slices: Vec<Slice> = CREATE_COUNTS
      .into_iter()
      .map(|creates| materialize_slice(&source, creates))
      .collect();

    for slice in &slices {
      assert_annotates_every_namespace(slice);
    }

    let mut group = c.benchmark_group("TransformDebugPath");

    for slice in &slices {
      for (label, dev) in [("dev", true), ("prod", false)] {
        // Batched because `apply` consumes the program: the clone is setup, not
        // work under measurement.
        group.bench_function(format!("{label}/{}", slice.creates), |b| {
          b.iter_batched(
            || slice.program.clone(),
            |program| black_box(transform(black_box(&slice.path), program, black_box(dev))),
            BatchSize::SmallInput,
          )
        });
      }
    }

    group.finish();
  });
}

criterion_group!(transform_debug_benches, debug_path_benchmarks);
criterion_main!(transform_debug_benches);
