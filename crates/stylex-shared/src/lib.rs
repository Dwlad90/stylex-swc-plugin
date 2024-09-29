pub mod shared;
pub mod transform;

use shared::structures::stylex_options::StyleXOptionsParams;
pub use transform::StyleXTransform;

#[cfg(test)]
mod tests {
  use ctor::ctor;

  #[ctor]
  fn init_color_backtrace() {
    pretty_env_logger::formatted_builder().init();
    color_backtrace::install();
  }
}
