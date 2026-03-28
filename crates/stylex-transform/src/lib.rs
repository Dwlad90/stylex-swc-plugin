pub mod shared;
pub mod transform;

/// Re-export for simpler usage in other crates
pub use transform::StyleXTransform;

#[cfg(test)]
mod tests {
  use ctor::ctor;

  #[ctor]
  fn init_logger() {
    pretty_env_logger::formatted_builder().try_init();
  }
}
