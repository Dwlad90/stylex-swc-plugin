// Represents the current state of a plugin for a file.
#[derive(Debug, PartialEq, Eq, Clone, Hash, Copy)]
pub(crate) enum ModuleCycle {
  // The plugin is being processed.
  TransformEnter,
  TransformExit,
  // The plugin has been processed and the file is being cleaned.
  PreCleaning,
  Cleaning,
  // The file has been processed and the plugin is skipped.
  Initializing,
  Skip,
  InjectStyles,
}
