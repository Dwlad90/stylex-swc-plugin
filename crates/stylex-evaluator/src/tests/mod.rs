use ctor::ctor;

mod growable_stack_test;
mod scaffolding;

#[ctor(unsafe)]
fn init_logger() {
  pretty_env_logger::formatted_builder().try_init().ok();
}
