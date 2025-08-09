/*!
Token types and tokenization utilities for CSS parsing.

This module provides CSS tokenization capabilities using the cssparser crate,
mirroring the functionality of the JavaScript TokenList and TokenIterator.
*/

use crate::CssResult;

/// Simple token representation
#[derive(Debug, Clone, PartialEq)]
pub enum SimpleToken {
    Ident(String),
    AtKeyword(String),
    Hash(String),
    String(String),
    Number(f64),
    Dimension { value: f64, unit: String },
    Percentage(f64),
    Url(String),
    Function(String),
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,
    Comma,
    Semicolon,
    Colon,
    Whitespace,
    Comment(String),
    Unknown(String),
}

/// A simple tokenizer that works with basic CSS tokens
/// This is a simplified version that will be enhanced later to use cssparser properly
pub struct SimpleTokenizer {
    input: String,
    position: usize,
}

impl SimpleTokenizer {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.to_string(),
            position: 0,
        }
    }

    pub fn next_token(&mut self) -> Option<SimpleToken> {
        if self.position >= self.input.len() {
            return None;
        }

        // Skip whitespace
        while self.position < self.input.len() && self.input.chars().nth(self.position)?.is_whitespace() {
            self.position += 1;
        }

        if self.position >= self.input.len() {
            return None;
        }

        let start = self.position;
        let ch = self.input.chars().nth(self.position)?;

        match ch {
            '(' => {
                self.position += 1;
                Some(SimpleToken::LeftParen)
            }
            ')' => {
                self.position += 1;
                Some(SimpleToken::RightParen)
            }
            '[' => {
                self.position += 1;
                Some(SimpleToken::LeftBracket)
            }
            ']' => {
                self.position += 1;
                Some(SimpleToken::RightBracket)
            }
            '{' => {
                self.position += 1;
                Some(SimpleToken::LeftBrace)
            }
            '}' => {
                self.position += 1;
                Some(SimpleToken::RightBrace)
            }
            ',' => {
                self.position += 1;
                Some(SimpleToken::Comma)
            }
            ';' => {
                self.position += 1;
                Some(SimpleToken::Semicolon)
            }
            ':' => {
                self.position += 1;
                Some(SimpleToken::Colon)
            }
            _ => {
                // For now, just consume everything else as an identifier
                while self.position < self.input.len() {
                    let ch = self.input.chars().nth(self.position);
                    if ch.is_none() || matches!(ch, Some('(' | ')' | '[' | ']' | '{' | '}' | ',' | ';' | ':')) {
                        break;
                    }
                    self.position += 1;
                }
                let value = self.input[start..self.position].trim().to_string();
                if value.is_empty() {
                    None
                } else {
                    Some(SimpleToken::Ident(value))
                }
            }
        }
    }

    pub fn is_at_end(&self) -> bool {
        self.position >= self.input.len()
    }
}

/// A list of CSS tokens with parsing state, mirroring the JavaScript TokenList class
pub struct TokenList {
    tokenizer: SimpleTokenizer,
    consumed_tokens: Vec<SimpleToken>,
    pub current_index: usize,
    is_at_end: bool,
}

impl TokenList {
    /// Create a new TokenList from a CSS string
    /// Mirrors: constructor(input: TokenIterator | string)
    pub fn new(input: &str) -> Self {
        Self {
            tokenizer: SimpleTokenizer::new(input),
            consumed_tokens: Vec::new(),
            current_index: 0,
            is_at_end: false,
        }
    }

    /// Consume the next token
    /// Mirrors: consumeNextToken(): CSSToken | null
    pub fn consume_next_token(&mut self) -> CssResult<Option<SimpleToken>> {
        if self.current_index < self.consumed_tokens.len() {
            // Return already consumed token
            let token = self.consumed_tokens[self.current_index].clone();
            self.current_index += 1;
            return Ok(Some(token));
        }

        if self.is_at_end {
            return Ok(None);
        }

        if self.tokenizer.is_at_end() {
            self.is_at_end = true;
            return Ok(None);
        }

        if let Some(token) = self.tokenizer.next_token() {
            self.consumed_tokens.push(token.clone());
            self.current_index += 1;
            if self.tokenizer.is_at_end() {
                self.is_at_end = true;
            }
            Ok(Some(token))
        } else {
            self.is_at_end = true;
            Ok(None)
        }
    }

    /// Peek at the next token without consuming it
    /// Mirrors: peek(): CSSToken | null
    pub fn peek(&mut self) -> CssResult<Option<SimpleToken>> {
        if self.current_index < self.consumed_tokens.len() {
            return Ok(Some(self.consumed_tokens[self.current_index].clone()));
        }

        if self.is_at_end || self.tokenizer.is_at_end() {
            return Ok(None);
        }

        if let Some(token) = self.tokenizer.next_token() {
            self.consumed_tokens.push(token.clone());
            Ok(Some(token))
        } else {
            self.is_at_end = true;
            Ok(None)
        }
    }

    /// Get the first token (alias for peek)
    /// Mirrors: get first(): CSSToken | null { return this.peek(); }
    pub fn first(&mut self) -> CssResult<Option<SimpleToken>> {
        self.peek()
    }

    /// Set the current parsing index
    /// Mirrors: setCurrentIndex(newIndex: number): void
    pub fn set_current_index(&mut self, new_index: usize) {
        if new_index < self.consumed_tokens.len() {
            // If we already have these tokens consumed, just update the index
            self.current_index = new_index;
            return;
        }

        // Try to consume tokens until we reach the target index
        while !self.is_at_end
            && !self.tokenizer.is_at_end()
            && self.consumed_tokens.len() <= new_index {

            if let Some(token) = self.tokenizer.next_token() {
                self.consumed_tokens.push(token);
                if self.tokenizer.is_at_end() {
                    self.is_at_end = true;
                }
            } else {
                self.is_at_end = true;
                break;
            }
        }

        // Clamp to the end if we couldn't reach the target
        self.current_index = new_index.min(self.consumed_tokens.len());
    }

    /// Rewind the parser by a number of positions
    /// Mirrors: rewind(positions: number = 1): void
    pub fn rewind(&mut self, positions: usize) {
        self.current_index = self.current_index.saturating_sub(positions);
    }

    /// Check if the token list is empty
    /// Mirrors: get isEmpty(): boolean
    pub fn is_empty(&self) -> bool {
        self.is_at_end ||
        (self.current_index >= self.consumed_tokens.len() && self.tokenizer.is_at_end())
    }

    /// Get all tokens by consuming everything
    /// Mirrors: getAllTokens(): $ReadOnlyArray<CSSToken>
    pub fn get_all_tokens(&mut self) -> Vec<SimpleToken> {
        // Consume all remaining tokens
        while !self.is_empty() {
            if self.consume_next_token().unwrap_or(None).is_none() {
                break;
            }
        }
        self.consumed_tokens.clone()
    }

    /// Get a slice of tokens from start to end
    /// Mirrors: slice(start: number, end: number = this.currentIndex): Array<CSSToken>
    pub fn slice(&mut self, start: usize, end: Option<usize>) -> Vec<SimpleToken> {
        let initial_index = self.current_index;
        let end = end.unwrap_or(self.current_index);

        if start > end {
            return Vec::new();
        }

        self.set_current_index(start);
        let mut result = Vec::new();

        // Consume tokens until we have enough to satisfy the slice request
        while self.current_index < end {
            match self.consume_next_token() {
                Ok(Some(token)) => result.push(token),
                _ => break,
            }
        }

        self.set_current_index(initial_index);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_tokenizer() {
        let mut tokenizer = SimpleTokenizer::new("color: red;");

        assert_eq!(tokenizer.next_token(), Some(SimpleToken::Ident("color".to_string())));
        assert_eq!(tokenizer.next_token(), Some(SimpleToken::Colon));
        assert_eq!(tokenizer.next_token(), Some(SimpleToken::Ident("red".to_string())));
        assert_eq!(tokenizer.next_token(), Some(SimpleToken::Semicolon));
        assert_eq!(tokenizer.next_token(), None);
        assert!(tokenizer.is_at_end());
    }

    #[test]
    fn test_token_list_basic() {
        let mut token_list = TokenList::new("color: red;");

        // Test peek (should not advance)
        let first = token_list.peek().unwrap();
        assert_eq!(first, Some(SimpleToken::Ident("color".to_string())));

        // Peek again should return the same
        let first_again = token_list.peek().unwrap();
        assert_eq!(first_again, Some(SimpleToken::Ident("color".to_string())));

        // Test consume (should advance)
        let consumed = token_list.consume_next_token().unwrap();
        assert_eq!(consumed, Some(SimpleToken::Ident("color".to_string())));

        // Next peek should be different
        let second = token_list.peek().unwrap();
        assert_eq!(second, Some(SimpleToken::Colon));
    }

    #[test]
    fn test_token_list_rewind() {
        let mut token_list = TokenList::new("a : b");

        // Consume two tokens
        token_list.consume_next_token().unwrap();
        token_list.consume_next_token().unwrap();

        // Current should be 'b'
        let current = token_list.peek().unwrap();
        assert_eq!(current, Some(SimpleToken::Ident("b".to_string())));

        // Rewind and check
        token_list.rewind(1);
        let rewound = token_list.peek().unwrap();
        assert_eq!(rewound, Some(SimpleToken::Colon));
    }

    #[test]
    fn test_token_list_slice() {
        let mut token_list = TokenList::new("a : b ; c");

        let slice = token_list.slice(1, Some(4));
        assert_eq!(slice.len(), 3);
        assert_eq!(slice[0], SimpleToken::Colon);
        assert_eq!(slice[1], SimpleToken::Ident("b".to_string()));
        assert_eq!(slice[2], SimpleToken::Semicolon);
    }
}
