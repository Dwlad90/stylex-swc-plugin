//! A logger the tests can read back, so a reporting path is covered by an
//! assertion rather than by a coverage exclusion.
//!
//! A `log` macro builds its message only when `log::max_level` admits it, so
//! every argument of every message is unexecuted code until a logger is in
//! place. Reading the message back then makes the message itself the subject of
//! a case.
//!
//! The same helper as `stylex_state`'s, with one addition: this test binary
//! already printed its messages for whoever set `RUST_LOG`, and `log` takes one
//! logger for the whole process — so this one wraps that one rather than
//! replacing it. Printing stays that logger's decision and is unchanged.
//!
//! Both the level and the messages are per thread, because the harness runs
//! tests in parallel and one global level would let one case decide what
//! another one sees. `log::max_level` stays wide open; the per-thread level is
//! what [`Log::enabled`] answers from.

use std::cell::RefCell;

use log::{Level, LevelFilter, Log, Metadata, Record};

thread_local! {
  static LEVEL: RefCell<LevelFilter> = const { RefCell::new(LevelFilter::Off) };
  static MESSAGES: RefCell<Vec<String>> = const { RefCell::new(Vec::new()) };
}

/// The logger this binary installs: everything the wrapped one would have
/// written, plus a copy of what the running thread asked to keep.
struct CapturingLogger(Box<dyn Log>);

impl CapturingLogger {
  /// Whether the running thread is keeping messages at this level.
  fn kept(&self, metadata: &Metadata) -> bool {
    LEVEL.with_borrow(|level| metadata.level() <= *level)
  }
}

impl Log for CapturingLogger {
  fn enabled(&self, metadata: &Metadata) -> bool {
    self.kept(metadata) || self.0.enabled(metadata)
  }

  fn log(&self, record: &Record) {
    if self.kept(record.metadata()) {
      MESSAGES.with_borrow_mut(|messages| messages.push(record.args().to_string()));
    }

    self.0.log(record);
  }

  fn flush(&self) {
    self.0.flush();
  }
}

/// Installs the logger, wrapping the one this binary prints with.
///
/// Called once before any case runs. `max_level` is opened all the way because
/// it is what decides whether a message is built at all, and the two `enabled`
/// answers above are what decide where a built one goes.
pub(crate) fn install(prints: impl Log + 'static) {
  if log::set_boxed_logger(Box::new(CapturingLogger(Box::new(prints)))).is_ok() {
    log::set_max_level(LevelFilter::Trace);
  }
}

/// Runs `body` with this thread keeping messages at `level`, and hands back
/// everything it wrote.
pub(crate) fn logged_at<T>(level: Level, body: impl FnOnce() -> T) -> Vec<String> {
  LEVEL.with_borrow_mut(|current| *current = level.to_level_filter());
  MESSAGES.with_borrow_mut(Vec::clear);

  body();

  LEVEL.with_borrow_mut(|current| *current = LevelFilter::Off);
  MESSAGES.with_borrow(Clone::clone)
}
