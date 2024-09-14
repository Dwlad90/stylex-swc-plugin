#[derive(Debug, PartialEq, Clone)]
pub(crate) enum ImportPathResolution {
  False,
  Tuple(ImportPathResolutionType, String),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub(crate) enum ImportPathResolutionType {
  ThemeNameRef,
  // FilePath,
}
