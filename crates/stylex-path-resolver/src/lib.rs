mod file_system;
mod package_json;
pub mod resolvers;
mod utils;

#[cfg(test)]
mod tests {
  use ctor::ctor;

  #[ctor]
  fn init_color_backtrace() {
    pretty_env_logger::formatted_builder().init();
    color_backtrace::install();
  }
}
