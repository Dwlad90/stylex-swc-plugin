pub mod shared;
pub mod transform;

use shared::structures::stylex_options::StyleXOptionsParams;
pub use transform::StyleXTransform;

#[cfg(test)]
mod tests {
  use ctor::ctor;

  #[ctor]
  fn init_logger() {
    pretty_env_logger::formatted_builder().init();
  }
}
