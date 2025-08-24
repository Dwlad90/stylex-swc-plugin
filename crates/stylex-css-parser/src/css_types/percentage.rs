/*!
CSS Percentage type parsing.

Handles percentage values like "50%".
Provides percentage parsing functionality for CSS percentage values.
*/

pub use crate::css_types::common_types::Percentage;


pub use Percentage as CssPercentage;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_percentage_reexport() {
        let pct = Percentage::new(50.0);
        assert_eq!(pct.value, 50.0);
        assert_eq!(pct.to_string(), "50%");
    }

    #[test]
    fn test_css_percentage_alias() {
        let pct = CssPercentage::new(75.0);
        assert_eq!(pct.value, 75.0);
    }
}
