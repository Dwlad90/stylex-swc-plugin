use super::structures::{
    application_order::ApplicationOrder, property_specificity::PropertySpecificity,
    shorthands_of_shorthands::ShorthandsOfShorthands,
};

// Represents the current state of a plugin for a file.
#[derive(Debug, PartialEq, Eq)]
pub(crate) enum ModuleCycle {
    // The plugin is being processed.
    Processing,
    // The plugin has been processed and the file is being cleaned.
    Cleaning,
    // The file has been processed and the plugin is skipped.
    Initializing,
    Skip,
    InjectStyles,
    InjectClassName,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum InjectedStylesDeclarationType {
    NamedDeclarationExport,
    NamedPropertyOrDefaultExport,
}

// pub(crate) enum EXPANSIONS {
//     ApplicationOrder(ApplicationOrder),
//     PropertySpecificity(PropertySpecificity),
// ShorthandsOfShorthands(ShorthandsOfShorthands),
//     // "property-specificity" => ApplicationOrder,
//     // "shorthands-of-shorthands" => ApplicationOrder,
// }
