/*!
CSS basic shape parser.
Mirrors: packages/style-value-parser/src/css-types/basic-shape.js
*/

use crate::{
    token_parser::TokenParser,
    token_types::SimpleToken,
    css_types::{length_percentage_parser, LengthPercentage, position::Position}
};

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

        // Parse first value (required)
        let first_val = length_percentage_parser();

        // Parse remaining optional values, defaulting as in JS
        let second_val = length_percentage_parser().optional();
        let third_val = length_percentage_parser().optional();
        let fourth_val = length_percentage_parser().optional();

        // Parse optional round clause
        let round_kw = TokenParser::<SimpleToken>::token(SimpleToken::Ident("round".into()), Some("Ident"));
        let round_val = round_kw
            .flat_map(move |_| length_percentage_parser(), Some("round_val"))
            .optional();

        fn_name
            .flat_map(move |_| first_val.clone(), Some("first"))
            .flat_map(move |t| {
                let t_clone = t.clone();
                second_val.clone().map(move |r_opt| (t_clone.clone(), r_opt), Some("second"))
            }, Some("with_second"))
            .flat_map(move |(t, r_opt)| {
                let t_clone = t.clone();
                let r_clone = r_opt.clone();
                third_val.clone().map(move |b_opt| (t_clone.clone(), r_clone.clone(), b_opt), Some("third"))
            }, Some("with_third"))
            .flat_map(move |(t, r_opt, b_opt)| {
                let t_clone = t.clone();
                let r_clone = r_opt.clone();
                let b_clone = b_opt.clone();
                fourth_val.clone().map(move |l_opt| (t_clone.clone(), r_clone.clone(), b_clone.clone(), l_opt), Some("fourth"))
            }, Some("with_fourth"))
            .flat_map(move |(t, r_opt, b_opt, l_opt)| {
                let t_clone = t.clone();
                let r_clone = r_opt.clone();
                let b_clone = b_opt.clone();
                let l_clone = l_opt.clone();
                round_val.clone().map(move |round| (t_clone.clone(), r_clone.clone(), b_clone.clone(), l_clone.clone(), round), Some("round"))
            }, Some("with_round"))
                        .flat_map(move |(t, r_opt, b_opt, l_opt, round)| {
                let t_clone = t.clone();
                let r_val = r_opt.unwrap_or_else(|| t.clone());
                let b_val = b_opt.unwrap_or_else(|| t_clone.clone());
                let l_val = l_opt.unwrap_or_else(|| r_val.clone());
                let round_clone = round.clone();
                close.clone().map(move |_| BasicShape::Inset {
                    top: t_clone.clone(),
                    right: r_val.clone(),
                    bottom: b_val.clone(),
                    left: l_val.clone(),
                    round: round_clone.clone()
                }, Some("to_inset"))
            }, Some("close"))
    }

    fn circle_parser() -> TokenParser<BasicShape> {
        let fn_name = TokenParser::<String>::fn_name("circle");
        let close = TokenParser::<SimpleToken>::token(SimpleToken::RightParen, Some("RightParen"));

        let radius = CircleRadius::parser();
        let at_kw = TokenParser::<SimpleToken>::token(SimpleToken::Ident("at".into()), Some("Ident"));
        let position = at_kw
            .flat_map(move |_| Position::parser(), Some("pos"))
            .optional();

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

        let radius_x = CircleRadius::parser();
        let radius_y = CircleRadius::parser();
        let at_kw = TokenParser::<SimpleToken>::token(SimpleToken::Ident("at".into()), Some("Ident"));
        let position = at_kw
            .flat_map(move |_| Position::parser(), Some("pos"))
            .optional();

        fn_name
            .flat_map(move |_| radius_x.clone(), Some("radius_x"))
            .flat_map(move |rx| {
                let rx_clone = rx.clone();
                radius_y.clone().map(move |ry| (rx_clone.clone(), ry), Some("radius_y"))
            }, Some("with_radius_y"))
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

        let fill_rule = TokenParser::<SimpleToken>::token(SimpleToken::Ident(String::new()), Some("Ident"))
            .map(|t| if let SimpleToken::Ident(s) = t { s } else { String::new() }, Some(".value"))
            .where_fn(|s| s == "nonzero" || s == "evenodd", Some("fill_rule"))
            .optional();

        // For now, simplify to parse just one point - we'll expand this later
        let x_coord = length_percentage_parser();
        let y_coord = length_percentage_parser();
        let point = x_coord
            .flat_map(move |x| {
                let x_clone = x.clone();
                y_coord.clone().map(move |y| (x_clone.clone(), y), Some("point"))
            }, Some("xy"));

        fn_name
            .flat_map(move |_| fill_rule.clone(), Some("fill_rule"))
            .flat_map(move |rule| {
                let rule_clone = rule.clone();
                point.clone().map(move |pt| (rule_clone.clone(), vec![pt]), Some("points"))
            }, Some("with_points"))
            .flat_map(move |(rule, points)| {
                close.clone().map(move |_| BasicShape::Polygon {
                    fill_rule: rule.clone(),
                    points: points.clone()
                }, Some("to_polygon"))
            }, Some("close"))
    }

    fn path_parser() -> TokenParser<BasicShape> {
        let fn_name = TokenParser::<String>::fn_name("path");
        let close = TokenParser::<SimpleToken>::token(SimpleToken::RightParen, Some("RightParen"));

        let fill_rule = TokenParser::<SimpleToken>::token(SimpleToken::Ident(String::new()), Some("Ident"))
            .map(|t| if let SimpleToken::Ident(s) = t { s } else { String::new() }, Some(".value"))
            .where_fn(|s| s == "nonzero" || s == "evenodd", Some("fill_rule"))
            .optional();
        let string = TokenParser::<SimpleToken>::token(SimpleToken::String(String::new()), Some("String"))
            .map(|t| if let SimpleToken::String(s) = t { s } else { String::new() }, Some(".value"));

        fn_name
            .flat_map(move |_| fill_rule.clone(), Some("fill_rule"))
            .flat_map(move |rule| {
                let rule_clone = rule.clone();
                string.clone().map(move |s| (rule_clone.clone(), s), Some("str"))
            }, Some("with_string"))
            .flat_map(move |(rule, path)| {
                close.clone().map(move |_| BasicShape::Path {
                    fill_rule: rule.clone(),
                    path: path.clone()
                }, Some("to_path"))
            }, Some("close"))
    }
}
