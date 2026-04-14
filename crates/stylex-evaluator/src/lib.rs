#![cfg_attr(coverage_nightly, feature(coverage_attribute))]

pub mod common;

#[cfg(test)]
mod tests {
  use ctor::ctor;

  #[ctor]
  fn init_logger() {
    pretty_env_logger::formatted_builder().try_init();
  }
}
