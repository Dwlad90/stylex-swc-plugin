// Represents the current state of a plugin for a file.
#[derive(Debug, PartialEq, Eq)]
pub enum ModuleCycle {
    // The plugin is being processed.
    Processing,
    // The plugin has been processed and the file is being cleaned.
    Cleaning,
    // The file has been processed and the plugin is skipped.
    Skip,
}
