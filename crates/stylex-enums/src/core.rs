// Represents the current state of a plugin for a file.
#[derive(Debug, PartialEq, Eq, Clone, Hash, Copy)]
pub enum TransformationCycle {
  // The plugin is being processed
  TransformEnter,
  // The plugin has been processed
  TransformExit,
  // The plugin has been processed and the file is being cleaned
  PreCleaning,
  // The file is being cleaned
  Cleaning,
  // Discover stylex imports and fill the state with reference-count data in
  // a single AST walk (replaces the legacy `Initializing` + `StateFilling`
  // two-pass split).
  Discover,
}
