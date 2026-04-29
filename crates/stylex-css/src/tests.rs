use ctor::ctor;

#[ctor]
fn init_logger() {
  pretty_env_logger::formatted_builder().try_init();
}
