/*!
CSS easing function parser.
Mirrors: packages/style-value-parser/src/css-types/easing-function.js
*/

use crate::{token_parser::TokenParser, token_types::SimpleToken};

/// A CSS easing function
#[derive(Debug, Clone, PartialEq)]
pub enum EasingFunction {
    Linear(Vec<f64>),
    CubicBezier(f64, f64, f64, f64),
    Steps(u32, StepsStart),
    Keyword(CubicBezierKeyword),
    StepsKeyword(StepsKeywordType), // Added: mirrors StepsKeyword class from JS
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StepsStart {
    Start,
    End,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CubicBezierKeyword {
    Ease,
    EaseIn,
    EaseOut,
    EaseInOut,
}

/// CSS steps keywords: 'step-start' | 'step-end'
/// Mirrors: StepsKeyword class from easing-function.js
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StepsKeywordType {
    StepStart,  // 'step-start'
    StepEnd,    // 'step-end'
}

impl EasingFunction {
    pub fn parse() -> TokenParser<EasingFunction> {
        TokenParser::one_of(vec![
            Self::linear_parser(),
            Self::cubic_bezier_keyword_parser(),
            Self::cubic_bezier_parser(),
            Self::steps_parser(),
            Self::steps_keyword_parser(), // Added: mirrors StepsKeyword.parser from JS
        ])
    }

    fn number() -> TokenParser<f64> {
        TokenParser::<SimpleToken>::token(SimpleToken::Number(0.0), Some("Number"))
            .map(|t| if let SimpleToken::Number(v) = t { v } else { 0.0 }, Some("to_f64"))
    }

    fn ident_string() -> TokenParser<String> {
        TokenParser::<SimpleToken>::token(SimpleToken::Ident(String::new()), Some("Ident"))
            .map(|t| if let SimpleToken::Ident(s) = t { s } else { String::new() }, Some(".value"))
    }

    fn expect_close_paren<T>(out: T) -> TokenParser<T>
    where
        T: Clone + std::fmt::Debug + 'static,
    {
        TokenParser::<SimpleToken>::token(SimpleToken::RightParen, Some("RightParen"))
            .map(move |_| out.clone(), Some(")"))
    }

    fn linear_parser() -> TokenParser<EasingFunction> {
        // linear(<number>[, <number> ...])
        let fn_name = TokenParser::<String>::fn_name("linear");
        let first = Self::number();
        let comma = TokenParser::<SimpleToken>::token(SimpleToken::Comma, Some("Comma"));
        let tail = TokenParser::zero_or_more(Self::number().prefix(comma));

        fn_name
            .flat_map(move |_| first.clone(), Some("first"))
            .flat_map(move |head| {
                let t = tail.clone();
                t.map(move |mut rest| {
                    let mut values = Vec::with_capacity(1 + rest.len());
                    values.push(head);
                    values.append(&mut rest);
                    values
                }, Some("collect"))
            }, Some("tail"))
            .flat_map(|values| Self::expect_close_paren(values), Some("close"))
            .map(|values| EasingFunction::Linear(values), Some("to_linear"))
    }

    fn cubic_bezier_keyword_parser() -> TokenParser<EasingFunction> {
        // ease, ease-in, ease-out, ease-in-out
        Self::ident_string()
            .where_fn(
                |s| matches!(s.as_str(), "ease" | "ease-in" | "ease-out" | "ease-in-out"),
                Some("bezier_keyword"),
            )
            .map(|s| {
                let kw = match s.as_str() {
                    "ease" => CubicBezierKeyword::Ease,
                    "ease-in" => CubicBezierKeyword::EaseIn,
                    "ease-out" => CubicBezierKeyword::EaseOut,
                    _ => CubicBezierKeyword::EaseInOut,
                };
                EasingFunction::Keyword(kw)
            }, Some("to_keyword"))
    }

    fn cubic_bezier_parser() -> TokenParser<EasingFunction> {
        // cubic-bezier(n,n,n,n)
        let fn_name = TokenParser::<String>::fn_name("cubic-bezier");
        let comma = TokenParser::<SimpleToken>::token(SimpleToken::Comma, Some("Comma"));
        let numbers = TokenParser::<f64>::sequence(vec![
            Self::number(),
            Self::number().prefix(comma.clone()),
            Self::number().prefix(comma.clone()),
            Self::number().prefix(comma.clone()),
        ]);

        fn_name
            .flat_map(move |_| numbers.clone(), Some("args"))
            .flat_map(|vals| Self::expect_close_paren(vals), Some("close"))
            .map(|vals| EasingFunction::CubicBezier(vals[0], vals[1], vals[2], vals[3]), Some("to_bezier"))
    }

    fn steps_parser() -> TokenParser<EasingFunction> {
        // steps(<number>, start|end)
        let fn_name = TokenParser::<String>::fn_name("steps");
        let comma = TokenParser::<SimpleToken>::token(SimpleToken::Comma, Some("Comma"));
        let number = Self::number().map(|v| v.max(0.0).round() as u32, Some("to_u32"));
        let start_end = Self::ident_string().where_fn(
            |s| matches!(s.as_str(), "start" | "end"),
            Some("start_end"),
        );

        fn_name
            .flat_map(move |_| number.clone(), Some("num"))
            .flat_map(move |n| {
                let se = start_end.clone();
                comma.clone().flat_map(move |_| se.clone().map(move |s| (n, s), Some("pair")), Some("after_comma"))
            }, Some("arg2"))
            .flat_map(|(n, s)| Self::expect_close_paren((n, s)), Some("close"))
            .map(|(n, s)| {
                let pos = if s == "start" { StepsStart::Start } else { StepsStart::End };
                EasingFunction::Steps(n, pos)
            }, Some("to_steps"))
    }

    /// Parser for steps keywords: 'step-start' | 'step-end'
    /// Mirrors: StepsKeyword.parser from easing-function.js
    fn steps_keyword_parser() -> TokenParser<EasingFunction> {
        TokenParser::one_of(vec![
            TokenParser::<SimpleToken>::string("step-start")
                .map(|_| EasingFunction::StepsKeyword(StepsKeywordType::StepStart), Some("step_start")),
            TokenParser::<SimpleToken>::string("step-end")
                .map(|_| EasingFunction::StepsKeyword(StepsKeywordType::StepEnd), Some("step_end")),
        ])
    }
}
