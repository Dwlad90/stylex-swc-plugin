//! Test entrypoint for stylex-regex.
//! Keeps logger setup centralized and groups test suites by regex domain.

use ctor::ctor;

mod regex_patterns_test;

#[ctor]
fn init_logger() {
  pretty_env_logger::formatted_builder().try_init();
}
