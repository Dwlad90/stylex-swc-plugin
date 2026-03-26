pub mod ast;

#[cfg(test)]
mod tests {
  use ctor::ctor;

  #[ctor]
  fn init_logger() {
    pretty_env_logger::formatted_builder().init();
  }
}
