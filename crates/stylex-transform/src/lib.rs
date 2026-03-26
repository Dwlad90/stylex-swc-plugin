pub mod shared;
pub mod transform;

// Re-export for use by $crate:: in exported macros
#[doc(hidden)]
pub use stylex_ast as __stylex_ast;

use stylex_structures::stylex_options::StyleXOptionsParams;
pub use transform::StyleXTransform;

#[cfg(test)]
mod tests {
  use ctor::ctor;

  #[ctor]
  fn init_logger() {
    pretty_env_logger::formatted_builder().init();
  }
}
