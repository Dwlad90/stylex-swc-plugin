use ctor::ctor;

/// Prepares the whole test binary before any test runs.
///
/// One constructor rather than one per concern: each `#[ctor]` is a second
/// `unsafe` entry point running before `main`, and everything here is the same
/// job — putting the process into the state every test in this crate assumes.
#[ctor(unsafe)]
fn prepare_test_binary() {
  pretty_env_logger::formatted_builder().try_init().ok();
  disable_diagnostic_colours();
}

/// Renders every `StyleXError` without styling for the whole test binary.
///
/// `StyleXError`'s `Display` colours the `[StyleX]` prefix and the message, and
/// whether it does is decided at runtime by whether stderr is a terminal and by
/// `NO_COLOR` / `CLICOLOR_FORCE`. Any test that asserts on a rendered
/// diagnostic therefore passes with output piped to a file and fails in a
/// terminal — a real defect this suite has already had once.
///
/// Fixed once here rather than by stripping escape codes at each assertion: a
/// helper only protects the tests that remember to call it, and the next
/// diagnostic assertion written is the one that will not.
fn disable_diagnostic_colours() {
  stylex_macros::stylex_error::disable_colour_output();
}
