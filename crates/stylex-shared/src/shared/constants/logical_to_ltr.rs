use phf::phf_map;

pub(crate) static LOGICAL_TO_LTR: phf::Map<&'static str, &'static str> = phf_map! {
        "margin-start" => "margin-left",
        "margin-end" => "margin-right",
        "padding-start" => "padding-left",
        "padding-end" => "padding-right",
        "border-start" => "border-left",
        "border-end" => "border-right",
        "border-start-width" => "border-left-width",
        "border-end-width" => "border-right-width",
        "border-start-color" => "border-left-color",
        "border-end-color" => "border-right-color",
        "border-start-style" => "border-left-style",
        "border-end-style" => "border-right-style",
        "border-top-start-radius" => "border-top-left-radius",
        "border-bottom-start-radius" => "border-bottom-left-radius",
        "border-top-end-radius" => "border-top-right-radius",
        "border-bottom-end-radius" => "border-bottom-right-radius",
        "start" => "left",
        "end" => "right",
};
