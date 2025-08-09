/**
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */
use std::fmt::{self, Display};

use crate::css_types::custom_ident::CustomIdentifier;
use crate::parser::Parser;

#[derive(Debug, Clone, PartialEq)]
pub struct DashedIdentifier {
  pub value: String,
}

impl DashedIdentifier {
  pub fn new(value: String) -> Self {
    Self { value }
  }

  pub fn parse<'a>() -> Parser<'a, DashedIdentifier> {
    Parser::<'a, String>::sequence::<String, CustomIdentifier, String, String>(
      Some(Parser::<'a, String>::string("--")),
      Some(CustomIdentifier::parse()),
      None,
      None,
    )
    .to_parser()
    .map(|tuple| {
      tuple
        .map(|(dash, ident, _, _)| {
          let dash = dash.unwrap();
          let ident = ident.unwrap();
          DashedIdentifier::new(dash + &ident.value)
        })
        .unwrap()
    })
  }
}

impl Display for DashedIdentifier {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.value)
  }
}
