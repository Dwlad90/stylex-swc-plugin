use stylex_css_parser::css_types::flex::Flex;

#[cfg(test)]
mod tests {
    use super::*;

    mod flex_parse {
        use super::*;

        // parses valid `fr` values
        #[test]
        fn parses_valid_fr_values() {
            assert_eq!(Flex::parse().parse_to_end("1fr").unwrap(), Flex::new(1.0));
            assert_eq!(Flex::parse().parse_to_end("2.5fr").unwrap(), Flex::new(2.5));
            assert_eq!(Flex::parse().parse_to_end("0fr").unwrap(), Flex::new(0.0));
        }

        // rejects invalid `fr` values
        #[test]
        fn rejects_invalid_fr_values() {
            assert!(Flex::parse().parse_to_end("-1fr").is_err());
            assert!(Flex::parse().parse_to_end("1 fr").is_err());
            assert!(Flex::parse().parse_to_end("1px").is_err());
        }
    }
}
