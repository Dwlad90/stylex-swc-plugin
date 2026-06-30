use super::*;

// ── VerticalKeyword::as_str center arm (line 68) ─────────────────────────

#[test]
fn vertical_keyword_as_str_center() {
  // "center" arm of VerticalKeyword::as_str was not covered by existing tests.
  assert_eq!(VerticalKeyword::Center.as_str(), "center");
}

// ── Position::is_horizontal_ident — non-Ident branch (line 134) ──────────

#[test]
fn is_horizontal_ident_returns_false_for_non_ident() {
  // Exercises the `else { false }` branch with a non-Ident token.
  let result = Position::is_horizontal_ident(&SimpleToken::Whitespace);
  assert!(!result);
}

#[test]
fn is_horizontal_ident_returns_true_for_valid_idents() {
  assert!(Position::is_horizontal_ident(&SimpleToken::Ident(
    "left".to_string()
  )));
  assert!(Position::is_horizontal_ident(&SimpleToken::Ident(
    "center".to_string()
  )));
  assert!(Position::is_horizontal_ident(&SimpleToken::Ident(
    "right".to_string()
  )));
}

#[test]
fn is_horizontal_ident_returns_false_for_unknown_ident() {
  assert!(!Position::is_horizontal_ident(&SimpleToken::Ident(
    "top".to_string()
  )));
}

// ── Position::token_to_horizontal_keyword — unreachable arms ─────────────

#[test]
fn token_to_horizontal_keyword_happy_path() {
  assert_eq!(
    Position::token_to_horizontal_keyword(SimpleToken::Ident("left".to_string())),
    HorizontalKeyword::Left
  );
  assert_eq!(
    Position::token_to_horizontal_keyword(SimpleToken::Ident("center".to_string())),
    HorizontalKeyword::Center
  );
  assert_eq!(
    Position::token_to_horizontal_keyword(SimpleToken::Ident("right".to_string())),
    HorizontalKeyword::Right
  );
}

#[test]
#[should_panic]
fn token_to_horizontal_keyword_wildcard_panics() {
  // Exercises `_ => stylex_unreachable!()` (line 146).
  Position::token_to_horizontal_keyword(SimpleToken::Ident("top".to_string()));
}

#[test]
#[should_panic]
fn token_to_horizontal_keyword_non_ident_panics() {
  // Exercises `else { stylex_unreachable!() }` (line 149).
  Position::token_to_horizontal_keyword(SimpleToken::Whitespace);
}

// ── Position::is_vertical_ident — non-Ident branch (line 163) ───────────

#[test]
fn is_vertical_ident_returns_false_for_non_ident() {
  let result = Position::is_vertical_ident(&SimpleToken::Whitespace);
  assert!(!result);
}

#[test]
fn is_vertical_ident_returns_true_for_valid_idents() {
  assert!(Position::is_vertical_ident(&SimpleToken::Ident(
    "top".to_string()
  )));
  assert!(Position::is_vertical_ident(&SimpleToken::Ident(
    "center".to_string()
  )));
  assert!(Position::is_vertical_ident(&SimpleToken::Ident(
    "bottom".to_string()
  )));
}

#[test]
fn is_vertical_ident_returns_false_for_unknown_ident() {
  assert!(!Position::is_vertical_ident(&SimpleToken::Ident(
    "left".to_string()
  )));
}

// ── Position::token_to_vertical_keyword — unreachable arms ───────────────

#[test]
fn token_to_vertical_keyword_happy_path() {
  assert_eq!(
    Position::token_to_vertical_keyword(SimpleToken::Ident("top".to_string())),
    VerticalKeyword::Top
  );
  assert_eq!(
    Position::token_to_vertical_keyword(SimpleToken::Ident("center".to_string())),
    VerticalKeyword::Center
  );
  assert_eq!(
    Position::token_to_vertical_keyword(SimpleToken::Ident("bottom".to_string())),
    VerticalKeyword::Bottom
  );
}

#[test]
#[should_panic]
fn token_to_vertical_keyword_wildcard_panics() {
  // Exercises `_ => stylex_unreachable!()` (line 175).
  Position::token_to_vertical_keyword(SimpleToken::Ident("left".to_string()));
}

#[test]
#[should_panic]
fn token_to_vertical_keyword_non_ident_panics() {
  // Exercises `else { stylex_unreachable!() }` (line 178).
  Position::token_to_vertical_keyword(SimpleToken::Whitespace);
}

// ── Vertical-then-horizontal parser (lines 221-232) ──────────────────────

#[test]
fn parses_top_left_vertical_then_horizontal() {
  // "top left" exercises the vertical_horizontal_keywords branch (lines 221-232).
  let result = Position::parser().parse_to_end("top left").unwrap();
  assert!(matches!(
    result.horizontal,
    Some(Horizontal::Keyword(HorizontalKeyword::Left))
  ));
  assert!(matches!(
    result.vertical,
    Some(Vertical::Keyword(VerticalKeyword::Top))
  ));
}

#[test]
fn parses_bottom_right_vertical_then_horizontal() {
  let result = Position::parser().parse_to_end("bottom right").unwrap();
  assert!(matches!(
    result.horizontal,
    Some(Horizontal::Keyword(HorizontalKeyword::Right))
  ));
  assert!(matches!(
    result.vertical,
    Some(Vertical::Keyword(VerticalKeyword::Bottom))
  ));
}

// ── position_parser convenience fn (lines 354-356) ───────────────────────

#[test]
fn position_parser_fn_works() {
  // Exercises the `position_parser()` free function.
  let result = position_parser().parse_to_end("left").unwrap();
  assert!(matches!(
    result.horizontal,
    Some(Horizontal::Keyword(HorizontalKeyword::Left))
  ));
}

#[test]
fn position_parser_fn_parses_top() {
  let result = position_parser().parse_to_end("top").unwrap();
  assert!(matches!(
    result.vertical,
    Some(Vertical::Keyword(VerticalKeyword::Top))
  ));
}
