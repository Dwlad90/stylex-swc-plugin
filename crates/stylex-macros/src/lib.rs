pub mod collection_macros;
pub mod conversion_macros;
pub mod panic_macros;
pub mod stylex_error;

#[cfg(test)]
mod tests {
  use ctor::ctor;

  #[ctor]
  fn init_logger() {
    pretty_env_logger::formatted_builder().try_init();
  }
}
