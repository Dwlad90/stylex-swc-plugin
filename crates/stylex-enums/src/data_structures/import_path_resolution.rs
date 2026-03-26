#[derive(Debug, PartialEq, Clone)]
pub enum ImportPathResolution {
  False,
  Tuple(ImportPathResolutionType, String),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ImportPathResolutionType {
  ThemeNameRef,
  // FilePath,
}
