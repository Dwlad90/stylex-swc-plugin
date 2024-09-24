// Represents the current state of a plugin for a file.
#[derive(Debug, PartialEq, Eq, Clone, Hash, Copy)]
pub(crate) enum TransformationCycle {
  // The plugin is being processed
  TransformEnter,
  // The plugin has been processed
  TransformExit,
  // The plugin has been processed and the file is being cleaned
  PreCleaning,
  // The file is being cleaned
  Cleaning,
  // Recounting variable links
  Recounting,
  // The file has been processed and the plugin is skipped
  Initializing,
  // Fill the state with expressions data before transformation
  StateFilling,
  // Skip the plugin if import does not exist
  Skip,
  // Inject styles metadata to the file
  InjectStyles,
}
