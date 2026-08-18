use ctor::ctor;

#[ctor(unsafe)]
fn init_logger() {
  pretty_env_logger::formatted_builder().try_init().ok();
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
#[ctor(unsafe)]
fn disable_diagnostic_colours() {
  colored::control::set_override(false);
}
