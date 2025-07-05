use phf::phf_map;

pub(crate) static LOGICAL_TO_RTL: phf::Map<&'static str, &'static str> = phf_map! {
    "margin-start" => "margin-right",
    "margin-end" => "margin-left",
    "padding-start" => "padding-right",
    "padding-end" => "padding-left",
    "border-start" => "border-right",
    "border-end" => "border-left",
    "border-start-width" => "border-right-width",
    "border-end-width" => "border-left-width",
    "border-start-color" => "border-right-color",
    "border-end-color" => "border-left-color",
    "border-start-style" => "border-right-style",
    "border-end-style" => "border-left-style",
    "border-top-start-radius" => "border-top-right-radius",
    "border-bottom-start-radius" => "border-bottom-right-radius",
    "border-top-end-radius" => "border-top-left-radius",
    "border-bottom-end-radius" => "border-bottom-left-radius",
    "start" => "right",
    "end" => "left",
};
