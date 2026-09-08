//! A logger the tests can read back, so a reporting path is covered by an
//! assertion rather than by a coverage exclusion.
//!
//! A `log` macro builds its message only when `log::max_level` admits it, so
//! every argument of every message is unexecuted code until a logger is in
//! place. Reading the message back then makes the message itself the subject of
//! a case.
//!
//! Both the level and the messages are per thread, because the harness runs
//! tests in parallel and one global level would let one case decide what
//! another one sees. `log::max_level` stays wide open; the per-thread level is
//! what [`Log::enabled`] answers from.
//!
//! The same helper as `stylex_diagnostics`'s. `log` takes one logger for the
//! whole process and every test binary is a process of its own, so each crate
//! that reads its messages back holds its own copy.

use std::cell::RefCell;

use log::{Level, LevelFilter, Log, Metadata, Record};

thread_local! {
  static LEVEL: RefCell<LevelFilter> = const { RefCell::new(LevelFilter::Off) };
  static MESSAGES: RefCell<Vec<String>> = const { RefCell::new(Vec::new()) };
}

struct CapturingLogger;

impl Log for CapturingLogger {
  fn enabled(&self, metadata: &Metadata) -> bool {
    LEVEL.with_borrow(|level| metadata.level() <= *level)
  }

  fn log(&self, record: &Record) {
    if !self.enabled(record.metadata()) {
      return;
    }

    MESSAGES.with_borrow_mut(|messages| messages.push(record.args().to_string()));
  }

  fn flush(&self) {}
}

/// Installs the logger, on the first case that reads a message back.
///
/// A second call answers `Err`, which is the expected reply once the logger is
/// in place.
fn install() {
  if log::set_boxed_logger(Box::new(CapturingLogger)).is_ok() {
    log::set_max_level(LevelFilter::Trace);
  }
}

/// Runs `body` with this thread logging at `level`, and hands back everything
/// it wrote.
pub(crate) fn logged_at<T>(level: Level, body: impl FnOnce() -> T) -> Vec<String> {
  install();

  LEVEL.with_borrow_mut(|current| *current = level.to_level_filter());
  MESSAGES.with_borrow_mut(Vec::clear);

  body();

  LEVEL.with_borrow_mut(|current| *current = LevelFilter::Off);
  MESSAGES.with_borrow(Clone::clone)
}
