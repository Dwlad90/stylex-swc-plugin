pub mod css;
pub mod order;
pub mod utils;
pub mod values;

#[cfg(test)]
mod tests {
  use ctor::ctor;

  #[ctor]
  fn init_logger() {
    pretty_env_logger::formatted_builder().try_init();
  }
}
