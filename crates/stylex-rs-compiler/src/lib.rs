#![allow(deprecated)]

// Sets this addon's global allocator. Linked for its `#[global_allocator]` and
// nothing else, hence the `as _`.
//
// The transform is allocation-bound rather than compute-bound: a sampled
// profile of a large module puts roughly 45% of samples in `malloc`/`free`,
// because every folded style flows through short-lived `String`s, `Vec`s and
// cloned AST nodes. Swapping the allocator is the whole of the change and it
// measures 1.09-1.15x end-to-end on this repo's two largest fixtures.
//
// `swc_malloc` rather than `mimalloc` directly: it resolves to mimalloc on
// every target `.github/workflows/npm.yml` publishes except
// `x86_64-unknown-linux-musl`, where the system allocator is kept on purpose
// because mimalloc segfaults on ARM64 musl. The workspace manifest records why.
use swc_malloc as _;

mod enums;
mod structs;
mod utils;
use log::{info, warn};
use napi::{Env, Result};
use std::{
  env, panic,
  path::{Component, Path, PathBuf},
  sync::Arc,
};
use structs::{StyleXMetadata, StyleXOptions, StyleXTransformResult};
use stylex_logs::initializer::initialize as initialize_logger;
use stylex_macros::stylex_error::{SuppressPanicStderr, format_panic_message};
use swc_compiler_base::{PrintArgs, SourceMapsConfig, print};

use stylex_structures::{plugin_pass::PluginPass, stylex_options::StyleXOptionsParams};
use stylex_transform::StyleXTransform;
use swc_ecma_parser::{Parser, StringInput, Syntax, TsSyntax, lexer::Lexer};

use swc_core::{
  common::{FileName, GLOBALS, Globals, Mark, SourceMap, comments::SingleThreadedComments},
  ecma::{
    ast::EsVersion,
    transforms::{
      base::{fixer::fixer, hygiene::hygiene, resolver},
      typescript::{Config as TypescriptConfig, typescript},
    },
    visit::visit_mut_pass,
  },
};

use napi_derive::napi;
use utils::extract_stylex_metadata;

use crate::enums::SourceMaps;

fn source_maps_config(source_map: Option<&SourceMaps>) -> SourceMapsConfig {
  match source_map {
    Some(SourceMaps::True) => SourceMapsConfig::Bool(true),
    Some(SourceMaps::False) => SourceMapsConfig::Bool(false),
    Some(SourceMaps::Inline) => SourceMapsConfig::Str("inline".into()),
    None => SourceMapsConfig::Bool(true),
  }
}

/// The extensions whose modules are JavaScript rather than TypeScript.
///
/// Not [`stylex_path_resolver::resolvers::EXTENSIONS`], which is the list of
/// suffixes to *try* when resolving an import path and includes `.md`. This one
/// answers which language a file was authored in, and only its own caller wants
/// it.
const JAVASCRIPT_EXTENSIONS: [&str; 4] = ["js", "jsx", "mjs", "cjs"];

/// Whether `path` names a JavaScript module rather than a TypeScript one.
///
/// The extension is the only thing that can say. Every input is parsed as
/// TypeScript regardless — the parser cannot be chosen per file without already
/// knowing the answer, and TypeScript is the superset — so nothing later in the
/// pipeline can tell the two apart.
///
/// It decides one thing: whether the type-stripping pass may elide an import
/// specifier nothing in the module references as a value. That elision is
/// TypeScript's rule and only TypeScript's — a binding with no value reference
/// may name a type, and a type has no module to import at runtime. JavaScript
/// has no type-only imports, so in a `.js` module the same specifier is a value
/// import the author wrote, and removing it changes what the module means.
///
/// It cost a refusal, which is how it was found. A dynamic style's parameter
/// shadowing an imported name is not a *reference* to it, so when the parameter
/// is the specifier's only occurrence the specifier was elided before the
/// StyleX transform ran, nothing registered the name, and a module the
/// reference implementation refuses compiled to a runtime value instead.
///
/// A TypeScript input keeps the elision, because there the hazard runs the
/// other way: preserving a specifier that names a type makes the emitted module
/// import a file that may hold nothing at runtime. So the two answer
/// differently on purpose, and only for a TypeScript input does this compiler
/// still read a shadowed StyleX import as an ordinary parameter.
///
/// An extension this does not recognise — none at all, or one no toolchain
/// agrees on — is answered as TypeScript, which is the conservative half: it
/// keeps the elision, and an elision only ever removes something.
///
/// Out of scope: TypeScript *syntax* inside a file named as JavaScript. With
/// the elision off, `type A = number; export { A };` in a `.js` file drops the
/// alias and keeps the export, leaving a module that cannot link. That input is
/// already malformed — this pipeline happens to parse it — and the spelling a
/// real toolchain produces, `export type { A }`, is stripped unconditionally by
/// the specifier walk regardless of this answer.
fn is_javascript_input(path: &Path) -> bool {
  path
    .extension()
    .and_then(|extension| extension.to_str())
    .is_some_and(|extension| {
      JAVASCRIPT_EXTENSIONS
        .iter()
        .any(|javascript| javascript.eq_ignore_ascii_case(extension))
    })
}

/// Whether to embed the original source text in the emitted map's
/// `sourcesContent`. Defaults to `true`: `@babel/generator` always embeds it,
/// and `next-swc-loader` turns it on whenever it asks for a map, so leaving it
/// off makes DevTools fall back to fetching `sources[0]` over
/// `webpack-internal://` — which 404s.
fn resolve_inline_sources_content(inline_sources_content: Option<bool>) -> bool {
  inline_sources_content.unwrap_or(true)
}

/// Whether to emit column positions in `mappings`. Defaults to `true`, matching
/// `@babel/generator` and webpack's own `SourceMapDevToolPlugin` (which only
/// drops columns for the `cheap-*` devtools). Without it every mapping starts
/// at column 0 and DevTools can only highlight whole lines.
fn resolve_emit_source_map_columns(emit_source_map_columns: Option<bool>) -> bool {
  emit_source_map_columns.unwrap_or(true)
}

/// Remove source text inherited from an input map when inlining is disabled.
fn clear_source_contents(input_source_map: &mut swc_sourcemap::SourceMap) {
  for idx in 0..input_source_map.get_source_count() {
    input_source_map.set_source_contents(idx, None);
  }
}

/// Whether a source map's `sources` entry refers to `path`.
///
/// The same file is spelled several ways by upstream tooling — `page.tsx`,
/// `./app/page.tsx`, the absolute path, `file:///abs/path/page.tsx` — so the
/// comparison is on trailing path *components* after dropping any URL scheme,
/// never on the raw string. Component-wise matching is what keeps
/// `other/page.tsx` from matching `app/page.tsx`; a plain suffix compare would
/// also accept `my-page.tsx`.
///
/// An entry that still doesn't line up (a `webpack://<namespace>/…` URL, say)
/// simply doesn't match, and the caller leaves it untouched.
fn source_names_file(source: &str, path: &Path) -> bool {
  let source = source.split_once("://").map_or(source, |(_, rest)| rest);

  let mut path_components = path.components().rev();
  let mut matched = false;

  for source_component in Path::new(source).components().rev() {
    // `Components` keeps a leading `.`; it carries no meaning here.
    if matches!(source_component, Component::CurDir) {
      continue;
    }

    match path_components.next() {
      Some(path_component) if path_component == source_component => matched = true,
      _ => return false,
    }
  }

  matched
}

/// Seed the authored text into a chained input map, for the one entry that
/// unambiguously names the file being compiled.
///
/// `SourceMap::build_source_map_with_config` returns `orig` verbatim once its
/// mappings have been adjusted, discarding everything the builder would
/// otherwise have inlined. So on the chained path `inline_sources_content` has
/// no effect and `sourcesContent` is whatever earlier tooling left behind —
/// frequently `null`, which is what sends Chrome DevTools off to re-fetch
/// `sources[0]` over `webpack-internal://`. Filling the gap here, on the
/// already-parsed map and before it reaches `print`, costs no extra parse or
/// serialization.
///
/// Deliberately narrow. `src` is this loader's *input*, not the original
/// authored file, so it is only correct for the entry describing this file.
/// An entry is filled only when it carries no text already and is the sole
/// entry resolving to `path`; two claimants means neither can be filled with
/// confidence and nothing is written. Attaching this text to an earlier
/// authored file named by the chain would yield a plausible but wrong map,
/// which is worse to debug against than no text at all.
fn backfill_source_contents(
  input_source_map: &mut swc_sourcemap::SourceMap,
  path: &Path,
  src: &str,
) {
  let mut candidate = None;

  for idx in 0..input_source_map.get_source_count() {
    let names_this_file = input_source_map
      .get_source(idx)
      .is_some_and(|source| source_names_file(source, path));

    if !names_this_file {
      continue;
    }

    if candidate.is_some() {
      return;
    }

    candidate = Some(idx);
  }

  match candidate {
    Some(idx) if input_source_map.get_source_contents(idx).is_none() => {
      input_source_map.set_source_contents(idx, Some(src.to_owned().into()));
    },
    _ => {},
  }
}

#[napi]
pub fn transform(
  env: Env,
  filename: String,
  code: String,
  mut options: StyleXOptions,
) -> Result<StyleXTransformResult> {
  initialize_logger();

  info!("Transforming source file: {}", filename);

  // Parse the env object separately since it needs the napi::Env for JS function
  // references.
  let parsed_env = options
    .env
    .take()
    .map(|ref env_obj| utils::fn_parser::parse_env_object(&env, env_obj))
    .transpose()?;

  // Parse debugFilePath separately since it needs the napi::Env for JS function
  // references. The UnknownRef must be explicitly unref'd after extracting its
  // value to avoid napi "ObjectRef is not unref" leak warnings.
  let parsed_debug_file_path = options
    .debug_file_path
    .take()
    .map(|unknown_ref| {
      let parse_result = unknown_ref
        .get_value(&env)
        .and_then(|value| utils::fn_parser::parse_debug_file_path(&env, value));
      // Always unref to prevent leak, regardless of parse success/failure
      let _ = unknown_ref.unref(&env);
      parse_result
    })
    .transpose()?;

  let _suppress = SuppressPanicStderr::new();
  let result = panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
    let cm: Arc<SourceMap> = Default::default();
    let file_path = PathBuf::from(filename);
    let filename = FileName::Real(file_path.clone());

    let fm = cm.new_source_file(filename.clone().into(), code);

    let cwd = env::current_dir()?;

    let plugin_pass = PluginPass {
      cwd: Some(cwd),
      filename: filename.clone(),
    };

    let source_map = source_maps_config(options.source_map.as_ref());
    let should_chain_input_source_map =
      !matches!(options.source_map.as_ref(), Some(SourceMaps::False));
    let inline_sources_content = resolve_inline_sources_content(options.inline_sources_content);
    let emit_source_map_columns = resolve_emit_source_map_columns(options.emit_source_map_columns);

    // Parse the incoming source map (if any) once: it feeds both the debug
    // source-map annotations and the chaining of the emitted map.
    let input_source_map = options.input_source_map.take().and_then(|json| {
      match swc_sourcemap::SourceMap::from_slice(json.as_bytes()) {
        Ok(mut map) => {
          // Chaining returns the input map as-is, so `inline_sources_content`
          // has to be applied to *it* rather than to the printer's builder:
          // seed this file's text where the chain left a hole, or strip the
          // inherited text outright when the caller disabled inlining.
          if should_chain_input_source_map {
            if inline_sources_content {
              backfill_source_contents(&mut map, &file_path, &fm.src);
            } else {
              clear_source_contents(&mut map);
            }
          }

          Some(Arc::new(map))
        },
        Err(err) => {
          warn!(
            "[StyleX] Failed to parse inputSourceMap, ignoring it: {}",
            err
          );
          None
        },
      }
    });

    // Column granularity is not ours to choose once a map is being chained.
    // `adjust_mappings` keeps the *input* map's tokens — columns and all — and
    // only shifts them by one (line, col) delta per covering range in the map
    // built here. Dropping columns from that map leaves exactly one range per
    // generated line, so every token on the line is shifted by the delta
    // computed for the first one. StyleX rewrites a line non-uniformly, so
    // those columns come out wrong, and the map is no smaller for it: its size
    // is the input map's token count either way. Emit ours in full and let the
    // upstream map's granularity carry through.
    let is_chaining = should_chain_input_source_map && input_source_map.is_some();
    let emit_source_map_columns = emit_source_map_columns || is_chaining;

    let mut config: StyleXOptionsParams = options.try_into()?;

    // Set the parsed env and debugFilePath on the config
    config.env = parsed_env;
    config.debug_file_path = parsed_debug_file_path;

    // Collect comments while lexing and hand the same store to the printer.
    // Without it the emitted code loses every comment — including the ones
    // bundlers act on: `/* webpackChunkName: "…" */` on dynamic imports and
    // `/* #__PURE__ */` annotations that minifiers need to drop dead calls.
    let comments = SingleThreadedComments::default();

    let mut parser = Parser::new_from(Lexer::new(
      Syntax::Typescript(TsSyntax {
        tsx: true,
        ..Default::default()
      }),
      EsVersion::latest(),
      StringInput::from(&*fm),
      Some(&comments),
    ));

    let program = match parser.parse_program() {
      Ok(program) => program,
      Err(err) => {
        let error_message = format!("Failed to parse file `{}`: {:?}", filename, err);
        return Err(napi::Error::from_reason(error_message));
      },
    };

    let globals = Globals::default();
    GLOBALS.set(&globals, || {
      // Set the NAPI env in thread-local storage so env functions can call back to JS
      utils::fn_parser::with_napi_env(&env, || {
        let unresolved_mark = Mark::new();
        let top_level_mark = Mark::new();

        // The same store the lexer filled and the printer will read. The
        // alternative, `PluginCommentsProxy`, only forwards to a wasm plugin
        // host — outside `wasm32` every one of its methods is a no-op, so a
        // transform holding it can neither read an existing annotation nor
        // attach a new one, and does so silently.
        let mut stylex: StyleXTransform<&SingleThreadedComments> =
          StyleXTransform::new(&comments, plugin_pass, &mut config);

        // Give the transform exact access to the parsed input so span-based
        // position lookups need no re-parsing, and to the input source map so
        // debug annotations point at the original authored file.
        stylex.state.set_input_source_file(fm.clone());
        if let Some(ref input_source_map) = input_source_map {
          stylex.state.set_input_source_map(input_source_map.clone());
        }

        let program = program
          .apply(resolver(unresolved_mark, top_level_mark, true))
          .apply(typescript(
            // `verbatim_module_syntax` turns off exactly one thing:
            // inferring that an unreferenced import specifier must have been a
            // type. Every explicitly type-only form is still stripped, so this
            // is safe for the JavaScript input it is turned on for.
            //
            // Leaving it off for a TypeScript file is a deliberate divergence
            // from the reference implementation rather than an omission, and it
            // is the reason a `.ts` module can compile a shape a `.js` module
            // refuses. The measurement and the decision are
            // `docs/adr/0001-a-typescript-module-reads-an-unreferenced-import-as-a-type.md`.
            TypescriptConfig {
              verbatim_module_syntax: is_javascript_input(&file_path),
              ..Default::default()
            },
            unresolved_mark,
            top_level_mark,
          ))
          .apply(&mut visit_mut_pass(&mut stylex))
          .apply(hygiene())
          .apply(&mut fixer(None));

        let stylex_metadata = extract_stylex_metadata(env, &stylex)?;
        drop(stylex);

        // StateManager shared this map during transformation and has just been
        // dropped, so the Arc is normally unique here and unwraps for free.
        // `print` takes `orig` by value, and `build_source_map_with_config`
        // clones it again internally — this saves one of those two copies of
        // every token and source-content string, not both.
        let original_source_map = if should_chain_input_source_map {
          input_source_map.map(Arc::unwrap_or_clone)
        } else {
          None
        };

        let transformed_code = print(
          cm,
          &program,
          PrintArgs {
            source_map,
            inline_sources_content,
            emit_source_map_columns,
            comments: Some(&comments),
            // Chain the emitted map onto the input map so it resolves all the
            // way back to the original authored file.
            orig: original_source_map,
            ..Default::default()
          },
        );

        let result = match transformed_code {
          Ok(output) => output,
          Err(e) => {
            return Err(napi::Error::from_reason(format!(
              "[StyleX] Failed to print transformed code: {}",
              e
            )));
          },
        };

        let js_result = StyleXTransformResult {
          code: result.code,
          metadata: StyleXMetadata {
            stylex: stylex_metadata,
          },
          map: result.map,
        };

        Ok(js_result)
      })
    })
  }));

  match result {
    Ok(res) => res,
    Err(error) => {
      let error_msg = format_panic_message(&error);

      Err(napi::Error::from_reason(error_msg))
    },
  }
}

#[cfg(test)]
#[path = "tests/lib_tests.rs"]
mod tests;
