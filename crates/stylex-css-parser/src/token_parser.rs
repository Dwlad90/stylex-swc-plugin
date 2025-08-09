/*!
Token parser combinators for CSS parsing.

This module provides a monadic parser combinator library for building CSS parsers,
closely mirroring the JavaScript TokenParser class and its associated functionality.
*/

use crate::{CssResult, CssParseError, token_types::{TokenList, SimpleToken}};
use std::fmt::Debug;
use std::rc::Rc;

/// A parser function that takes a TokenList and returns a result
pub type ParseFn<T> = Rc<dyn Fn(&mut TokenList) -> CssResult<Option<T>>>;

/// A parser combinator for CSS tokens, mirroring the JavaScript TokenParser class
#[derive(Clone)]
pub struct TokenParser<T: Clone + Debug> {
    parse_fn: ParseFn<T>,
    label: String,
}

impl<T: Clone + Debug + 'static> TokenParser<T> {
    /// Create a new TokenParser
    /// Mirrors: constructor(parser: Function, label: string)
    pub fn new<F>(parser_fn: F, label: &str) -> Self
    where
        F: Fn(&mut TokenList) -> CssResult<Option<T>> + 'static,
    {
        Self {
            parse_fn: Rc::new(parser_fn),
            label: label.to_string(),
        }
    }

    /// Parse a CSS string using this parser
    /// Mirrors: parse(css: string): T | Error
    pub fn parse(&self, css: &str) -> CssResult<Option<T>> {
        let mut tokens = TokenList::new(css);
        (self.parse_fn)(&mut tokens)
    }

    /// Parse a CSS string and ensure all input is consumed
    /// Mirrors: parseToEnd(css: string): T
    pub fn parse_to_end(&self, css: &str) -> CssResult<T> {
        let mut tokens = TokenList::new(css);
        let initial_index = tokens.current_index;

        let output = (self.parse_fn)(&mut tokens)?;

        if let Some(value) = output {
            // Check if we've consumed all input
            if let Ok(Some(_remaining_token)) = tokens.peek() {
                let consumed_tokens = tokens.slice(initial_index, None);
                return Err(CssParseError::ParseError {
                    message: format!(
                        "Expected end of input, but found remaining tokens\nConsumed tokens: {:?}",
                        consumed_tokens
                    ),
                });
            }
            Ok(value)
        } else {
            Err(CssParseError::ParseError {
                message: format!("Expected {} to return a value, but got None", self.label),
            })
        }
    }

    /// Map the output of this parser using a function
    /// Mirrors: map<NewT>(f: (value: T) => NewT, label?: string): TokenParser<NewT>
    pub fn map<U, F>(&self, f: F, label: Option<&str>) -> TokenParser<U>
    where
        U: Clone + Debug + 'static,
        F: Fn(T) -> U + 'static,
    {
        let parse_fn = self.parse_fn.clone();
        let new_label = format!("{}.map({})", self.label, label.unwrap_or(""));

        TokenParser::new(
            move |tokens| {
                let current_index = tokens.current_index;
                match (parse_fn)(tokens) {
                    Ok(Some(value)) => Ok(Some(f(value))),
                    Ok(None) => Ok(None),
                    Err(e) => {
                        tokens.set_current_index(current_index);
                        Err(e)
                    }
                }
            },
            &new_label,
        )
    }

    /// Flat map operation for chaining parsers
    /// Mirrors: flatMap<U>(f: (value: T) => TokenParser<U>, label?: string): TokenParser<U>
    pub fn flat_map<U, F>(&self, f: F, label: Option<&str>) -> TokenParser<U>
    where
        U: Clone + Debug + 'static,
        F: Fn(T) -> TokenParser<U> + 'static,
    {
        let parse_fn = self.parse_fn.clone();
        let new_label = format!("{}.flatMap({})", self.label, label.unwrap_or(""));

        TokenParser::new(
            move |tokens| {
                let current_index = tokens.current_index;

                let output1 = match (parse_fn)(tokens) {
                    Ok(Some(value)) => value,
                    Ok(None) => return Ok(None),
                    Err(e) => {
                        tokens.set_current_index(current_index);
                        return Err(e);
                    }
                };

                let second_parser = f(output1);
                match (second_parser.parse_fn)(tokens) {
                    Ok(result) => Ok(result),
                    Err(e) => {
                        tokens.set_current_index(current_index);
                        Err(e)
                    }
                }
            },
            &new_label,
        )
    }

    /// Try this parser, or fall back to another parser
    /// Mirrors: or<U>(parser2: TokenParser<U>): TokenParser<T | U>
    pub fn or<U>(&self, other: TokenParser<U>) -> TokenParser<Result<T, U>>
    where
        U: Clone + Debug + 'static,
    {
        let parse_fn1 = self.parse_fn.clone();
        let parse_fn2 = other.parse_fn.clone();
        let new_label = if other.label == "optional" {
            format!("Optional<{}>", self.label)
        } else {
            format!("OneOf<{}, {}>", self.label, other.label)
        };

        TokenParser::new(
            move |tokens| {
                let current_index = tokens.current_index;

                match (parse_fn1)(tokens) {
                    Ok(Some(value)) => Ok(Some(Ok(value))),
                    Ok(None) => Ok(None),
                    Err(_) => {
                        tokens.set_current_index(current_index);
                        match (parse_fn2)(tokens) {
                            Ok(Some(value)) => Ok(Some(Err(value))),
                            Ok(None) => Ok(None),
                            Err(e) => {
                                tokens.set_current_index(current_index);
                                Err(e)
                            }
                        }
                    }
                }
            },
            &new_label,
        )
    }

    /// Make this parser optional
    /// Mirrors: get optional(): TokenParser<void | T>
    pub fn optional(self) -> TokenParser<Option<T>> {
        let parse_fn = self.parse_fn;
        let new_label = format!("Optional<{}>", self.label);

        TokenParser::new(
            move |tokens| {
                let current_index = tokens.current_index;

                match (parse_fn)(tokens) {
                    Ok(value) => Ok(Some(value)),
                    Err(_) => {
                        tokens.set_current_index(current_index);
                        Ok(Some(None))
                    }
                }
            },
            &new_label,
        )
    }

    /// Apply a predicate to filter the results
    /// Mirrors: where<Refined>(predicate: (value: T) => boolean, label?: string): TokenParser<Refined>
    pub fn where_fn<F>(&self, predicate: F, label: Option<&str>) -> TokenParser<T>
    where
        F: Fn(&T) -> bool + 'static,
    {
        let description = label.unwrap_or("");
        self.flat_map(
            move |output| {
                if predicate(&output) {
                    TokenParser::always(output)
                } else {
                    TokenParser::never()
                }
            },
            Some(description),
        )
    }

    /// Parse with prefix and suffix parsers
    /// Mirrors: surroundedBy(prefix: TokenParser<mixed>, suffix?: TokenParser<mixed>): TokenParser<T>
    pub fn surrounded_by<P, S>(&self, prefix: TokenParser<P>, suffix: Option<TokenParser<S>>) -> TokenParser<T>
    where
        P: Clone + Debug + 'static,
        S: Clone + Debug + 'static,
    {
        let main_parser = self.clone();

        match suffix {
            Some(suffix_parser) => {
                prefix.flat_map(
                    move |_| {
                        let main = main_parser.clone();
                        let suffix = suffix_parser.clone();
                        main.flat_map(
                            move |value| {
                                let result_value = value.clone();
                                suffix.map(move |_| result_value.clone(), None)
                            },
                            Some("surrounded_middle")
                        )
                    },
                    Some("surrounded_prefix")
                )
            }
            None => {
                let prefix_clone = prefix.clone();
                prefix.flat_map(
                    move |_| {
                        let main = main_parser.clone();
                        let prefix_clone2 = prefix_clone.clone();
                        main.flat_map(
                            move |value| {
                                let result_value = value.clone();
                                prefix_clone2.map(move |_| result_value.clone(), None)
                            },
                            Some("surrounded_middle_same")
                        )
                    },
                    Some("surrounded_prefix_same")
                )
            }
        }
    }

    /// Skip a parser after this one
    /// Mirrors: skip(skipParser: TokenParser<mixed>): TokenParser<T>
    pub fn skip<U>(&self, skip_parser: TokenParser<U>) -> TokenParser<T>
    where
        U: Clone + Debug + 'static,
    {
        self.flat_map(
            move |output| {
                let output_clone = output.clone();
                skip_parser.map(move |_| output_clone.clone(), None)
            },
            Some("skip"),
        )
    }

    /// Add a prefix parser
    /// Mirrors: prefix(prefixParser: TokenParser<mixed>): TokenParser<T>
    pub fn prefix<P>(&self, prefix_parser: TokenParser<P>) -> TokenParser<T>
    where
        P: Clone + Debug + 'static,
    {
        prefix_parser.flat_map(
            {
                let self_clone = self.clone();
                move |_| self_clone.clone()
            },
            Some("prefix")
        )
    }

    /// Add a suffix parser
    /// Mirrors: suffix(suffixParser: TokenParser<mixed>): TokenParser<T>
    pub fn suffix<S>(&self, suffix_parser: TokenParser<S>) -> TokenParser<T>
    where
        S: Clone + Debug + 'static,
    {
        self.flat_map(
            move |output| {
                let output_clone = output.clone();
                suffix_parser.map(move |_| output_clone.clone(), None)
            },
            Some("suffix"),
        )
    }

    /// Get the label for this parser
    pub fn label(&self) -> &str {
        &self.label
    }
}

/// Static constructor methods for TokenParser
impl<T: Clone + Debug + 'static> TokenParser<T> {
    /// Parser that always succeeds with the given value
    /// Mirrors: static always<T>(output: T): TokenParser<T>
    pub fn always(value: T) -> TokenParser<T> {
        let label = format!("Always<{:?}>", value);
        TokenParser::new(
            move |_| Ok(Some(value.clone())),
            &label,
        )
    }

    /// Parser that always fails
    /// Mirrors: static never<T>(): TokenParser<T>
    pub fn never() -> TokenParser<T> {
        TokenParser::new(
            |_| Err(CssParseError::ParseError { message: "Never".to_string() }),
            "Never",
        )
    }

    /// Try multiple parsers in order
    /// Mirrors: static oneOf<T>(...parsers): TokenParser<T>
    pub fn one_of(parsers: Vec<TokenParser<T>>) -> TokenParser<T> {
        TokenParser::new(
            move |tokens| {
                let mut errors = Vec::new();
                let index = tokens.current_index;

                for parser in &parsers {
                    match (parser.parse_fn)(tokens) {
                        Ok(Some(output)) => return Ok(Some(output)),
                        Ok(None) => return Ok(None),
                        Err(e) => {
                            tokens.set_current_index(index);
                            errors.push(e);
                        }
                    }
                }

                Err(CssParseError::ParseError {
                    message: format!(
                        "No parser matched\n{}",
                        errors
                            .iter()
                            .map(|err| format!("- {}", err))
                            .collect::<Vec<_>>()
                            .join("\n")
                    ),
                })
            },
            "oneOf",
        )
    }

    /// Parse a sequence of parsers
    /// Mirrors: static sequence<T>(...parsers): TokenParserSequence<T>
    pub fn sequence<U: Clone + Debug + 'static>(parsers: Vec<TokenParser<U>>) -> TokenParser<Vec<U>> {
        TokenParser::new(
            move |tokens| {
                let current_index = tokens.current_index;
                let mut results = Vec::new();

                for parser in &parsers {
                    match (parser.parse_fn)(tokens) {
                        Ok(Some(value)) => results.push(value),
                        Ok(None) => {
                            // If any parser returns None, the whole sequence fails
                            tokens.set_current_index(current_index);
                            return Ok(None);
                        }
                        Err(e) => {
                            tokens.set_current_index(current_index);
                            return Err(e);
                        }
                    }
                }

                Ok(Some(results))
            },
            "sequence",
        )
    }

    /// Parse zero or more occurrences
    /// Mirrors: static zeroOrMore<T>(parser: TokenParser<T>): TokenZeroOrMoreParsers<T>
    pub fn zero_or_more(parser: TokenParser<T>) -> TokenParser<Vec<T>> {
        TokenParser::new(
            move |tokens| {
                let mut results = Vec::new();

                loop {
                    let current_index = tokens.current_index;
                    match (parser.parse_fn)(tokens) {
                        Ok(Some(value)) => results.push(value),
                        Ok(None) => continue,
                        Err(_) => {
                            tokens.set_current_index(current_index);
                            break;
                        }
                    }
                }

                Ok(Some(results))
            },
            &format!("ZeroOrMore<{}>", parser.label),
        )
    }

    /// Parse one or more occurrences
    /// Mirrors: static oneOrMore<T>(parser: TokenParser<T>): TokenOneOrMoreParsers<T>
    pub fn one_or_more(parser: TokenParser<T>) -> TokenParser<Vec<T>> {
        TokenParser::new(
            move |tokens| {
                let mut results = Vec::new();
                let start_index = tokens.current_index;

                // Must match at least once
                match (parser.parse_fn)(tokens) {
                    Ok(Some(value)) => results.push(value),
                    Ok(None) => {
                        tokens.set_current_index(start_index);
                        return Err(CssParseError::ParseError {
                            message: "OneOrMore requires at least one match".to_string(),
                        });
                    }
                    Err(e) => {
                        tokens.set_current_index(start_index);
                        return Err(e);
                    }
                }

                // Then try to match more
                loop {
                    let current_index = tokens.current_index;
                    match (parser.parse_fn)(tokens) {
                        Ok(Some(value)) => results.push(value),
                        Ok(None) => continue,
                        Err(_) => {
                            tokens.set_current_index(current_index);
                            break;
                        }
                    }
                }

                Ok(Some(results))
            },
            &format!("OneOrMore<{}>", parser.label),
        )
    }

    /// Parse a specific token type
    /// Mirrors: static token<TT>(tokenType: TT[0], label?: string): TokenParser<TT>
    pub fn token(expected_token: SimpleToken, label: Option<&str>) -> TokenParser<SimpleToken> {
        let label_str = label.unwrap_or(&format!("{:?}", expected_token)).to_string();

        TokenParser::new(
            move |tokens| {
                let current_index = tokens.current_index;

                match tokens.consume_next_token() {
                    Ok(Some(token)) => {
                        if std::mem::discriminant(&token) == std::mem::discriminant(&expected_token) {
                            Ok(Some(token))
                        } else {
                            tokens.set_current_index(current_index);
                            Err(CssParseError::ParseError {
                                message: format!(
                                    "Expected token type {:?}, got {:?}",
                                    expected_token, token
                                ),
                            })
                        }
                    }
                    Ok(None) => {
                        tokens.set_current_index(current_index);
                        Err(CssParseError::ParseError {
                            message: "Expected token, got end of input".to_string(),
                        })
                    }
                    Err(e) => {
                        tokens.set_current_index(current_index);
                        Err(e)
                    }
                }
            },
            &label_str,
        )
    }

    /// Parse a specific string as an identifier
    /// Mirrors: static string<S>(str: S): TokenParser<S>
    pub fn string(expected: &str) -> TokenParser<String> {
        let expected_clone = expected.to_string();
        TokenParser::<SimpleToken>::token(SimpleToken::Ident(String::new()), Some("Ident"))
            .map(
                |token| {
                    if let SimpleToken::Ident(value) = token {
                        value
                    } else {
                        unreachable!()
                    }
                },
                Some(".value"),
            )
            .where_fn(
                move |value| value == &expected_clone,
                Some(&format!("=== {}", expected)),
            )
    }
}

/// Common token parsers for basic CSS tokens
impl TokenParser<SimpleToken> {
    /// Parse an identifier token
    pub fn ident() -> TokenParser<SimpleToken> {
        TokenParser::<SimpleToken>::token(SimpleToken::Ident(String::new()), Some("Ident"))
    }

    /// Parse a colon token
    pub fn colon() -> TokenParser<SimpleToken> {
        TokenParser::<SimpleToken>::token(SimpleToken::Colon, Some("Colon"))
    }

    /// Parse a semicolon token
    pub fn semicolon() -> TokenParser<SimpleToken> {
        TokenParser::<SimpleToken>::token(SimpleToken::Semicolon, Some("Semicolon"))
    }

    /// Parse a comma token
    pub fn comma() -> TokenParser<SimpleToken> {
        TokenParser::<SimpleToken>::token(SimpleToken::Comma, Some("Comma"))
    }

    /// Parse a left parenthesis token
    pub fn left_paren() -> TokenParser<SimpleToken> {
        TokenParser::<SimpleToken>::token(SimpleToken::LeftParen, Some("LeftParen"))
    }

    /// Parse a right parenthesis token
    pub fn right_paren() -> TokenParser<SimpleToken> {
        TokenParser::<SimpleToken>::token(SimpleToken::RightParen, Some("RightParen"))
    }

    /// Parse a left bracket token
    pub fn left_bracket() -> TokenParser<SimpleToken> {
        TokenParser::<SimpleToken>::token(SimpleToken::LeftBracket, Some("LeftBracket"))
    }

    /// Parse a right bracket token
    pub fn right_bracket() -> TokenParser<SimpleToken> {
        TokenParser::<SimpleToken>::token(SimpleToken::RightBracket, Some("RightBracket"))
    }

    /// Parse a left brace token
    pub fn left_brace() -> TokenParser<SimpleToken> {
        TokenParser::<SimpleToken>::token(SimpleToken::LeftBrace, Some("LeftBrace"))
    }

    /// Parse a right brace token
    pub fn right_brace() -> TokenParser<SimpleToken> {
        TokenParser::<SimpleToken>::token(SimpleToken::RightBrace, Some("RightBrace"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_always_parser() {
        let parser = TokenParser::always(42);
        let result = parser.parse("anything").unwrap();
        assert_eq!(result, Some(42));
    }

    #[test]
    fn test_never_parser() {
        let parser: TokenParser<i32> = TokenParser::never();
        assert!(parser.parse("anything").is_err());
    }

    #[test]
    fn test_map_parser() {
        let parser = TokenParser::always(10)
            .map(|x| x * 2, Some("double"));
        let result = parser.parse("anything").unwrap();
        assert_eq!(result, Some(20));
    }

    #[test]
    fn test_flat_map_parser() {
        let parser = TokenParser::always(5)
            .flat_map(|x| TokenParser::always(x + 1), Some("add_one"));
        let result = parser.parse("anything").unwrap();
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_optional_parser() {
        let success_parser = TokenParser::always(42).optional();
        let result = success_parser.parse("anything").unwrap();
        assert_eq!(result, Some(Some(42)));

        let fail_parser: TokenParser<Option<i32>> = TokenParser::<i32>::never().optional();
        let result = fail_parser.parse("anything").unwrap();
        assert_eq!(result, Some(None));
    }

    #[test]
    fn test_where_fn_parser() {
        let parser = TokenParser::always(10)
            .where_fn(|&x| x > 5, Some("greater_than_5"));
        let result = parser.parse("anything").unwrap();
        assert_eq!(result, Some(10));

        let parser = TokenParser::always(3)
            .where_fn(|&x| x > 5, Some("greater_than_5"));
        assert!(parser.parse("anything").is_err());
    }

    #[test]
    fn test_one_of_parser() {
        let parser = TokenParser::one_of(vec![
            TokenParser::<i32>::never(),
            TokenParser::always(42),
            TokenParser::always(24),
        ]);
        let result = parser.parse("anything").unwrap();
        assert_eq!(result, Some(42)); // Should return first successful result
    }

    #[test]
    fn test_sequence_parser() {
        let parser = TokenParser::<i32>::sequence(vec![
            TokenParser::always(1),
            TokenParser::always(2),
            TokenParser::always(3),
        ]);
        let result = parser.parse("anything").unwrap();
        assert_eq!(result, Some(vec![1, 2, 3]));
    }

    #[test]
    fn test_zero_or_more_parser() {
        // This test is simplified since we don't have proper token consumption yet
        let parser = TokenParser::zero_or_more(TokenParser::<i32>::never());
        let result = parser.parse("anything").unwrap();
        assert_eq!(result, Some(vec![]));
    }

    #[test]
    fn test_one_or_more_parser() {
        // Test that it fails when no matches
        let parser = TokenParser::one_or_more(TokenParser::<i32>::never());
        assert!(parser.parse("anything").is_err());
    }

    #[test]
    fn test_or_parser() {
        let parser1 = TokenParser::always(1);
        let parser2 = TokenParser::always(2);
        let combined = parser1.or(parser2);

        let result = combined.parse("anything").unwrap();
        assert!(matches!(result, Some(Ok(1))));
    }

    #[test]
    fn test_string_parser() {
        // This is a more complex test that would require proper tokenization
        // For now, just test that the parser can be created
        let parser = TokenParser::<String>::string("test");
        assert_eq!(parser.label(), "Ident.map(.value).flatMap(=== test)");
    }

    #[test]
    fn test_token_parsers() {
        // Test that token parsers can be created
        let _ident = TokenParser::ident();
        let _colon = TokenParser::colon();
        let _semicolon = TokenParser::semicolon();
        let _comma = TokenParser::comma();

        // Just verify they have correct labels
        assert_eq!(TokenParser::ident().label(), "Ident");
        assert_eq!(TokenParser::colon().label(), "Colon");
        assert_eq!(TokenParser::comma().label(), "Comma");
    }

    #[test]
    fn test_parse_to_end() {
        let parser = TokenParser::always(42);
        // This should work since always parser doesn't consume tokens
        let result = parser.parse_to_end("").unwrap();
        assert_eq!(result, 42);
    }

    #[test]
    fn test_skip_parser() {
        let main_parser = TokenParser::always(10);
        let skip_parser = TokenParser::always(());
        let combined = main_parser.skip(skip_parser);

        let result = combined.parse("anything").unwrap();
        assert_eq!(result, Some(10));
    }

    #[test]
    fn test_prefix_parser() {
        let main_parser = TokenParser::always(10);
        let prefix_parser = TokenParser::always(());
        let combined = main_parser.prefix(prefix_parser);

        let result = combined.parse("anything").unwrap();
        assert_eq!(result, Some(10));
    }

    #[test]
    fn test_suffix_parser() {
        let main_parser = TokenParser::always(10);
        let suffix_parser = TokenParser::always(());
        let combined = main_parser.suffix(suffix_parser);

        let result = combined.parse("anything").unwrap();
        assert_eq!(result, Some(10));
    }

    #[test]
    fn test_label_preservation() {
        let parser = TokenParser::always(42);
        assert!(parser.label().contains("Always"));

        let mapped = parser.map(|x| x * 2, Some("double"));
        assert!(mapped.label().contains("map(double)"));
    }
}
