use swc_core::css::{
    ast::{ComponentValue, DimensionToken, ListOfComponentValues, Stylesheet, Token},
    visit::{Fold, FoldWith},
};

#[cfg(test)]
use swc_core::css::codegen::{
    writer::basic::{BasicCssWriter, BasicCssWriterConfig},
    CodeGenerator, CodegenConfig, Emit,
};

#[cfg(test)]
use crate::shared::utils::css::swc_parse_css;

struct CssFolder;

impl Fold for CssFolder {
    fn fold_list_of_component_values(
        &mut self,
        mut list: ListOfComponentValues,
    ) -> ListOfComponentValues {
        list.children = whitespace_normalizer(&list.children);

        list.fold_children_with(self)
    }
    fn fold_token(&mut self, token: Token) -> Token {
        match token {
            Token::Dimension(mut dimension) => {
                let mut dimension = timing_normalizer(&mut dimension).clone();
                let mut dimension = zero_demention_normalizer(&mut dimension).clone();
                let dimension = leading_zero_normalizer(&mut dimension);

                return Token::Dimension(dimension.clone());
            }
            _ => token,
        }
    }
}

fn whitespace_normalizer(components: &Vec<ComponentValue>) -> Vec<ComponentValue> {
    components
        .clone()
        .into_iter()
        .filter(|child| match child {
            ComponentValue::PreservedToken(preserved_token) => match &preserved_token.token {
                swc_core::css::ast::Token::WhiteSpace { value: _ } => false,
                _ => true,
            },
            _ => true,
        })
        .collect()
}

fn timing_normalizer(dimension: &mut Box<DimensionToken>) -> &Box<DimensionToken> {
    if !dimension.unit.eq("ms") || dimension.value < 10.0 {
        return dimension;
    }
    dimension.value = dimension.value / 1000.0;
    dimension.unit = "s".into();

    dimension
}

fn zero_demention_normalizer(dimension: &mut Box<DimensionToken>) -> &Box<DimensionToken> {
    if dimension.value != 0.0 {
        return dimension;
    }

    let angles = vec!["deg", "grad", "turn", "rad"];
    let timings = vec!["ms", "s"];

    dimension.value = 0.0;

    if angles.contains(&dimension.unit.to_string().as_str()) {
        dimension.unit = "deg".into();
    } else if timings.contains(&dimension.unit.to_string().as_str()) {
        dimension.unit = "s".into();
    }

    dimension
}

fn leading_zero_normalizer(dimension: &mut Box<DimensionToken>) -> &Box<DimensionToken> {
    if dimension.value < 1.0 && dimension.value >= 0.0 {
        dimension.raw_value = dimension.raw_value.replace("0.", ".").into();
    }

    dimension
}

pub(crate) fn base_normalizer(ast: Stylesheet) -> Stylesheet {
    let mut folder = CssFolder;

    ast.fold_children_with(&mut folder)
}

#[test]
fn should_normalize() {
    assert_eq!(
        stringify(&base_normalizer(
            swc_parse_css("opacity, margin-top").0.unwrap()
        )),
        "opacity,margin-top"
    );
}

/// Stringifies the [`Stylesheet`]
#[cfg(test)]
pub fn stringify(node: &Stylesheet) -> String {
    let mut buf = String::new();
    let writer = BasicCssWriter::new(&mut buf, None, BasicCssWriterConfig::default());
    let mut codegen = CodeGenerator::new(writer, CodegenConfig { minify: true });

    let _ = codegen.emit(&node);

    buf
}
