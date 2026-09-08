//! A logger the tests can read back, so a reporting path is run and asserted
//! rather than left out of the coverage measurement.
//!
//! A `log` macro builds its message only when `log::max_level` admits it, so
//! every argument of every message is unexecuted code until a logger opens the
//! level. Installing this one opens it for the whole binary, which is what runs
//! those arguments; reading a message back is what makes the words in it the
//! subject of a case.
//!
//! `stylex_state`'s helper wraps nothing, because nothing else in that binary
//! logs. This one wraps: this test binary already installed a printing logger
//! of its own from a `ctor`, and `log` takes one logger for the whole process.
//! What prints stays the wrapped logger's decision, taken against the level it
//! was built with, and is unchanged.
//!
//! Both the level and the messages are per thread, because one global level
//! would let one case decide what another one sees. `log::max_level` stays wide
//! open, and the per-thread level is what [`Log::log`] keeps a message by —
//! [`Log::enabled`] is not on that path at all, because a `log` macro asks
//! `max_level` and then calls `log` directly. `enabled` answers for
//! `log_enabled!`, which is the one caller that does ask it.

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
/// it is what decides whether a message is built at all. Where a built one goes
/// is decided in [`Log::log`] above: `kept` says whether this thread keeps a
/// copy, and the wrapped logger's own filter says whether it prints.
///
/// Opened for the whole binary rather than around each case, because this runs
/// from a `ctor` and there is no per-case moment to close it in. `nextest`
/// gives each case a process of its own, so a narrower level would be safe
/// there — but `cargo test` runs the cases of one binary as parallel threads,
/// and `max_level` is process-wide, so opening and closing it around one case
/// would decide what another one sees. The four other crates that read their
/// messages back hold the same shape.
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
