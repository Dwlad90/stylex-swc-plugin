pub mod constants;
pub mod css;
pub mod structures;
pub mod utils;

#[cfg(test)]
mod tests {
  use ctor::ctor;

  #[ctor]
  fn init_logger() {
    pretty_env_logger::formatted_builder().init();
  }
}
