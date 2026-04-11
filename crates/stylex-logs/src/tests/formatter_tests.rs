// Tests for log formatting color mapping and width tracking helpers.
// Source: crates/stylex-logs/src/formatter.rs

use super::*;

#[test]
fn max_target_width_tracks_longest_target() {
  MAX_MODULE_WIDTH.store(0, Ordering::Relaxed);

  assert_eq!(max_target_width("a"), 1);
  assert_eq!(max_target_width("ab"), 2);
  assert_eq!(max_target_width("x"), 2);
}

#[test]
fn ansi_constants_are_present() {
  assert_eq!(ANSI_RED, "\x1B[31m");
  assert_eq!(ANSI_GREEN, "\x1B[32m");
  assert_eq!(ANSI_RESET, "\x1B[0m");
  assert_eq!(ANSI_ORANGE, "\x1B[38;5;208m");
}

#[test]
fn level_color_maps_each_level() {
  assert_eq!(level_color(log::Level::Error), Color::Red);
  assert_eq!(level_color(log::Level::Warn), Color::Yellow);
  assert_eq!(level_color(log::Level::Info), Color::Green);
  assert_eq!(level_color(log::Level::Debug), Color::Blue);
  assert_eq!(level_color(log::Level::Trace), Color::White);
}

#[test]
fn format_log_line_contains_target_and_message() {
  let line = format_log_line(log::Level::Info, "stylex::module", "hello", 4);
  assert!(line.contains(STYLEX_PREFIX));
  assert!(line.contains("stylex::module"));
  assert!(line.contains("hello"));
}
