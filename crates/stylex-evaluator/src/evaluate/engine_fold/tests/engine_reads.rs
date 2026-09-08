//! How a case asks the engine what a value it built behaves like.
//!
//! A value this module hands the engine is asked the way the printed source asks
//! it — by running JavaScript against it — rather than through an accessor of the
//! compiler's own. Two suites do that, over values built by different halves of
//! the bridge, so the reading itself is written once here.

use boa_engine::{Context, JsValue, Source};

/// What `read` — an arrow over `values` — answers, as a string.
///
/// Every step that can fail says which of them failed, because a case that read
/// nothing and a case that read the wrong thing are different mistakes.
#[track_caller]
pub(super) fn answered_by(context: &mut Context, read: &str, values: &[JsValue]) -> String {
  let compiled = match context.eval(Source::from_bytes(read)) {
    Ok(compiled) => compiled,
    Err(error) => panic!("`{}` did not compile: {}", read, error),
  };

  let Some(reader) = compiled.as_callable() else {
    panic!("`{}` is not a function", read);
  };

  let answered = match reader.call(&JsValue::undefined(), values, context) {
    Ok(answered) => answered,
    Err(error) => panic!("`{}` threw: {}", read, error),
  };

  match answered.to_string(context) {
    Ok(text) => text.to_std_string_escaped(),
    Err(error) => panic!("`{}` answered something unreadable: {}", read, error),
  }
}
