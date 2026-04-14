#![cfg_attr(coverage_nightly, feature(coverage_attribute))]

mod file_system;
pub mod package_json;
pub mod resolvers;
pub mod utils;

#[cfg(test)]
mod tests {
  use ctor::ctor;

  #[ctor]
  fn init_logger() {
    pretty_env_logger::formatted_builder().try_init();
  }
}
