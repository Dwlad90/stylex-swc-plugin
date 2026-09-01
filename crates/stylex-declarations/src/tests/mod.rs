use ctor::ctor;

mod convertors_tests;
mod lookup_tests;

#[ctor(unsafe)]
fn init_logger() {
  pretty_env_logger::formatted_builder().try_init().ok();
}
