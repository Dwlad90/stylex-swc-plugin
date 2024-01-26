use phf::phf_set;

// Using MDN data as a source of truth to populate the above sets
// by group in alphabetical order:

pub(crate) static UNITLESS_NUMBER_PROPERTIES: phf::Set<&'static str> = phf_set! {
  "WebkitLineClamp",
  "animationIterationCount",
  "aspectRatio",
  "borderImageOutset",
  "borderImageSlice",
  "borderImageWidth",
  "counterSet",
  "columnCount",
  "flex", // Unsupportd
  "flexGrow",
  "flexPositive",
  "flexShrink",
  "flexOrder",
  "gridRow",
  "gridColumn",
  "fontWeight",
  "hyphenateLimitChars",
  "lineClamp",
  "lineHeight",
  "maskBorderOutset",
  "maskBorderSlice",
  "maskBorderWidth",
  "opacity",
  "order",
  "orphans",
  "tabSize",
  "widows",
  "zIndex",
  "fillOpacity",
  "floodOpacity",
  "rotate",
  "scale",
  "stopOpacity",
  "strokeDasharray",
  "strokeDashoffset",
  "strokeMiterlimit",
  "strokeOpacity",
  "strokeWidth",

  "mathDepth",
};
