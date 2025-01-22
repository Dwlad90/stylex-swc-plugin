use env_logger::{Builder, WriteStyle};
use log::LevelFilter;
use std::io::Write;
use std::sync::Once;

static INIT: Once = Once::new();

pub fn initialize() {
  INIT.call_once(|| {
    Builder::new()
      .format(|buf, record| {
        writeln!(
          buf,
          "[{}] {} [{}:{}] - {}",
          record.level(),
          chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
          record.file().unwrap_or("unknown"),
          record.line().unwrap_or(0),
          record.args()
        )
      })
      .filter_level(LevelFilter::Off)
      .parse_env("STYLEX_DEBUG") // Allow override via environment variable
      .write_style(WriteStyle::Always)
      .init();
  });
}
