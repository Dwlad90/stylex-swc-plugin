/*!
CSS basic shape parser.
Mirrors: packages/style-value-parser/src/css-types/basic-shape.js
*/

use crate::{
    token_parser::TokenParser,
    token_types::SimpleToken,
    css_types::{length_percentage_parser, LengthPercentage, position::Position}
};
use std::fmt::{self, Display};

/// A CSS basic shape
#[derive(Debug, Clone, PartialEq)]
pub enum BasicShape {
    Inset {
        top: LengthPercentage,
        right: LengthPercentage,
        bottom: LengthPercentage,
        left: LengthPercentage,
        round: Option<LengthPercentage>
    },
    Circle {
        radius: CircleRadius,
        position: Option<Position>
    },
    Ellipse {
        radius_x: CircleRadius,
        radius_y: CircleRadius,
        position: Option<Position>
    },
    Polygon {
        fill_rule: Option<String>,
        points: Vec<(LengthPercentage, LengthPercentage)>
    },
    Path {
        fill_rule: Option<String>,
        path: String
    },
}

/// Circle radius can be a length/percentage or keyword
#[derive(Debug, Clone, PartialEq)]
pub enum CircleRadius {
    LengthPercentage(LengthPercentage),
    ClosestSide,
    FarthestSide,
}

impl CircleRadius {
    pub fn parser() -> TokenParser<CircleRadius> {
        TokenParser::one_of(vec![
            length_percentage_parser().map(
                |lp| CircleRadius::LengthPercentage(lp),
                Some("length_percentage")
            ),
            TokenParser::<SimpleToken>::token(SimpleToken::Ident("closest-side".into()), Some("Ident"))
                .map(|_| CircleRadius::ClosestSide, Some("closest-side")),
            TokenParser::<SimpleToken>::token(SimpleToken::Ident("farthest-side".into()), Some("Ident"))
                .map(|_| CircleRadius::FarthestSide, Some("farthest-side")),
        ])
    }
}

impl Display for CircleRadius {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CircleRadius::LengthPercentage(lp) => write!(f, "{}", lp),
            CircleRadius::ClosestSide => write!(f, "closest-side"),
            CircleRadius::FarthestSide => write!(f, "farthest-side"),
        }
    }
}

impl BasicShape {
    pub fn parser() -> TokenParser<BasicShape> {
        TokenParser::one_of(vec![
            Self::inset_parser(),
            Self::circle_parser(),
            Self::ellipse_parser(),
            Self::polygon_parser(),
            Self::path_parser(),
        ])
    }

    fn inset_parser() -> TokenParser<BasicShape> {
        // Parse inset(top [right [bottom [left]]] [round border-radius])
        // Following JS: sequence with optionals that default
        let fn_name = TokenParser::<String>::fn_name("inset");
        let close = TokenParser::<SimpleToken>::token(SimpleToken::RightParen, Some("RightParen"));
        let whitespace = TokenParser::<SimpleToken>::token(SimpleToken::Whitespace, Some("Whitespace"));

        // Parse inset values with proper whitespace separation
        let inset_values = {
            let first = length_percentage_parser();
            let second = whitespace.clone().flat_map(|_| length_percentage_parser(), Some("second")).optional();
            let third = whitespace.clone().flat_map(|_| length_percentage_parser(), Some("third")).optional();
            let fourth = whitespace.clone().flat_map(|_| length_percentage_parser(), Some("fourth")).optional();

            first.flat_map(move |t| {
                let t_clone = t.clone();
                second.clone().map(move |r_opt| (t_clone.clone(), r_opt), Some("with_second"))
            }, Some("first_step"))
            .flat_map(move |(t, r_opt)| {
                let t_clone = t.clone();
                let r_clone = r_opt.clone();
                third.clone().map(move |b_opt| (t_clone.clone(), r_clone.clone(), b_opt), Some("with_third"))
            }, Some("second_step"))
            .flat_map(move |(t, r_opt, b_opt)| {
                let t_clone = t.clone();
                let r_clone = r_opt.clone();
                let b_clone = b_opt.clone();
                fourth.clone().map(move |l_opt| {
                    // Apply CSS shorthand expansion logic
                    let r_val = r_clone.clone().unwrap_or_else(|| t_clone.clone());
                    let b_val = b_clone.clone().unwrap_or_else(|| t_clone.clone());
                    let l_val = l_opt.unwrap_or_else(|| r_val.clone());
                    (t_clone.clone(), r_val, b_val, l_val)
                }, Some("apply_defaults"))
            }, Some("third_step"))
        };

        // Parse optional round clause
        let round_clause = {
            let round_kw = TokenParser::<SimpleToken>::token(SimpleToken::Ident("round".into()), Some("Ident"));
            let whitespace_for_round = whitespace.clone();
            whitespace.clone()
                .flat_map(move |_| round_kw.clone(), Some("round_keyword"))
                .flat_map(move |_| {
                    whitespace_for_round.clone().flat_map(|_| length_percentage_parser(), Some("round_value"))
                }, Some("round_val"))
                .optional()
        };

        fn_name
            .flat_map(move |_| inset_values.clone(), Some("inset_values"))
            .flat_map(move |(t, r, b, l)| {
                let t_clone = t.clone();
                let r_clone = r.clone();
                let b_clone = b.clone();
                let l_clone = l.clone();
                round_clause.clone().map(move |round| (t_clone.clone(), r_clone.clone(), b_clone.clone(), l_clone.clone(), round), Some("with_round"))
            }, Some("with_round"))
            .flat_map(move |(t, r, b, l, round)| {
                close.clone().map(move |_| BasicShape::Inset {
                    top: t.clone(),
                    right: r.clone(),
                    bottom: b.clone(),
                    left: l.clone(),
                    round: round.clone()
                }, Some("to_inset"))
            }, Some("close"))
    }

    fn circle_parser() -> TokenParser<BasicShape> {
        let fn_name = TokenParser::<String>::fn_name("circle");
        let close = TokenParser::<SimpleToken>::token(SimpleToken::RightParen, Some("RightParen"));
        let whitespace = TokenParser::<SimpleToken>::token(SimpleToken::Whitespace, Some("Whitespace"));

        let radius = CircleRadius::parser();
        let position = {
            let at_kw = TokenParser::<SimpleToken>::token(SimpleToken::Ident("at".into()), Some("Ident"));
            let whitespace_for_pos = whitespace.clone();
            whitespace.clone()
                .flat_map(move |_| at_kw.clone(), Some("at_keyword"))
                .flat_map(move |_| {
                    whitespace_for_pos.clone().flat_map(|_| Position::parser(), Some("position"))
                }, Some("pos"))
                .optional()
        };

        fn_name
            .flat_map(move |_| radius.clone(), Some("radius"))
            .flat_map(move |r| {
                let r_clone = r.clone();
                position.clone().map(move |p| (r_clone.clone(), p), Some("opt_pos"))
            }, Some("with_pos"))
            .flat_map(move |(r, p)| {
                close.clone().map(move |_| BasicShape::Circle { radius: r.clone(), position: p.clone() }, Some("to_circle"))
            }, Some("close"))
    }

    fn ellipse_parser() -> TokenParser<BasicShape> {
        let fn_name = TokenParser::<String>::fn_name("ellipse");
        let close = TokenParser::<SimpleToken>::token(SimpleToken::RightParen, Some("RightParen"));
        let whitespace = TokenParser::<SimpleToken>::token(SimpleToken::Whitespace, Some("Whitespace"));

        let radius_x = CircleRadius::parser();
        let radius_y = whitespace.clone().flat_map(|_| CircleRadius::parser(), Some("radius_y"));
        let position = {
            let at_kw = TokenParser::<SimpleToken>::token(SimpleToken::Ident("at".into()), Some("Ident"));
            let whitespace_for_pos = whitespace.clone();
            whitespace.clone()
                .flat_map(move |_| at_kw.clone(), Some("at_keyword"))
                .flat_map(move |_| {
                    whitespace_for_pos.clone().flat_map(|_| Position::parser(), Some("position"))
                }, Some("pos"))
                .optional()
        };

        fn_name
            .flat_map(move |_| radius_x.clone(), Some("radius_x"))
            .flat_map(move |rx| {
                let rx_clone = rx.clone();
                radius_y.clone().map(move |ry| (rx_clone.clone(), ry), Some("with_radius_y"))
            }, Some("radius_step"))
            .flat_map(move |(rx, ry)| {
                let rx_clone = rx.clone();
                let ry_clone = ry.clone();
                position.clone().map(move |p| (rx_clone.clone(), ry_clone.clone(), p), Some("opt_pos"))
            }, Some("with_pos"))
            .flat_map(move |(rx, ry, p)| {
                close.clone().map(move |_| BasicShape::Ellipse {
                    radius_x: rx.clone(),
                    radius_y: ry.clone(),
                    position: p.clone()
                }, Some("to_ellipse"))
            }, Some("close"))
    }

    fn polygon_parser() -> TokenParser<BasicShape> {
        let fn_name = TokenParser::<String>::fn_name("polygon");
        let close = TokenParser::<SimpleToken>::token(SimpleToken::RightParen, Some("RightParen"));

        // For now, implement a simplified but working version
        // TODO: Add support for multiple points and fill-rule
        let x_coord = length_percentage_parser();
        let y_coord = length_percentage_parser();

        let point = x_coord.flat_map(move |x| {
            let x_clone = x.clone();
            y_coord.clone().map(move |y| (x_clone.clone(), y), Some("point"))
        }, Some("xy"));

        fn_name
            .flat_map(move |_| point.clone(), Some("point"))
            .flat_map(move |pt| {
                close.clone().map(move |_| BasicShape::Polygon {
                    fill_rule: None,
                    points: vec![pt.clone()]
                }, Some("to_polygon"))
            }, Some("close"))
    }

    fn path_parser() -> TokenParser<BasicShape> {
        let fn_name = TokenParser::<String>::fn_name("path");
        let close = TokenParser::<SimpleToken>::token(SimpleToken::RightParen, Some("RightParen"));

        // For now, implement a simplified version that just parses the string path
        // TODO: Add support for fill-rule
        let string = TokenParser::<SimpleToken>::token(SimpleToken::String(String::new()), Some("String"))
            .map(|t| if let SimpleToken::String(s) = t { s } else { String::new() }, Some(".value"));

        fn_name
            .flat_map(move |_| string.clone(), Some("string"))
            .flat_map(move |path| {
                close.clone().map(move |_| BasicShape::Path {
                    fill_rule: None,
                    path: path.clone()
                }, Some("to_path"))
            }, Some("close"))
    }
}

impl Display for BasicShape {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BasicShape::Inset { top, right, bottom, left, round } => {
                let round_str = match round {
                    Some(r) => format!(" round {}", r),
                    None => String::new(),
                };

                // Optimize output by checking for equal values (mirrors JS logic exactly)
                // JavaScript logic: top === right && right === bottom && bottom === left && left === round
                if top == right && right == bottom && bottom == left &&
                   (round.is_none() || round.as_ref() == Some(left)) {
                    write!(f, "inset({}{})", top, round_str)
                } else if top == bottom && left == right {
                    write!(f, "inset({} {}{})", top, right, round_str)
                } else if top == bottom {
                    write!(f, "inset({} {} {}{})", top, right, bottom, round_str)
                } else {
                    write!(f, "inset({} {} {} {}{})", top, right, bottom, left, round_str)
                }
            },
            BasicShape::Circle { radius, position } => {
                let position_str = match position {
                    Some(p) => format!(" at {}", p),
                    None => String::new(),
                };
                write!(f, "circle({}{})", radius, position_str)
            },
            BasicShape::Ellipse { radius_x, radius_y, position } => {
                let position_str = match position {
                    Some(p) => format!(" at {}", p),
                    None => String::new(),
                };
                write!(f, "ellipse({} {}{})", radius_x, radius_y, position_str)
            },
            BasicShape::Polygon { fill_rule, points } => {
                let fill_rule_str = match fill_rule {
                    Some(rule) => format!("{}, ", rule),
                    None => String::new(),
                };
                let points_str = points
                    .iter()
                    .map(|(x, y)| format!("{} {}", x, y))
                    .collect::<Vec<_>>()
                    .join(", ");
                write!(f, "polygon({}{})", fill_rule_str, points_str)
            },
            BasicShape::Path { fill_rule, path } => {
                let fill_rule_str = match fill_rule {
                    Some(rule) => format!("{}, ", rule),
                    None => String::new(),
                };
                write!(f, "path({}\"{}\")", fill_rule_str, path)
            },
        }
    }
}
