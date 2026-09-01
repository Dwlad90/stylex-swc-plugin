use ctor::ctor;

mod convertors_tests;
mod evaluate_tests;
mod growable_stack_test;
pub(crate) mod scaffolding;
mod stylex_first_that_works_test;

#[ctor(unsafe)]
fn init_logger() {
  pretty_env_logger::formatted_builder().try_init().ok();
}
