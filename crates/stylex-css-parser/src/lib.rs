pub mod parser;
pub mod resolvers;
pub mod token_list;
pub mod token_parser;
pub mod tokens;
pub mod css_types;

#[cfg(test)]
mod tests {
  use ctor::ctor;

  #[ctor]
  fn init_color_backtrace() {
    pretty_env_logger::formatted_builder().init();
    color_backtrace::install();
  }
}
