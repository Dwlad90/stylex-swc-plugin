# `StyleX AST`

## Overview

SWC AST manipulation utilities for the StyleX compiler. Contains factory
functions for creating AST nodes and pure convertor functions.

## Contents

### Factories (`ast/factories`)

- `object_expression_factory`, `array_expression_factory` -- Collection literals
- `prop_or_spread_expression_factory`, `prop_or_spread_string_factory` -- Object
  properties
- `lit_str_factory`, `lit_number_factory`, `lit_boolean_factory`,
  `lit_null_factory` -- Literal factories
- `ident_factory`, `ident_name_factory`, `binding_ident_factory` -- Identifier
  factories
- `spread_element_factory`, `call_expr_member_factory`, `arrow_expr_factory` --
  Expression factories
- `jsx_attr_factory`, `jsx_attr_or_spread_*` -- JSX attribute factories
- `var_declarator_factory` -- Variable declaration factory

### Pure Convertors (`ast/convertors`)

- `number_to_expression`, `string_to_expression`, `bool_to_expression`,
  `null_to_expression`, `ident_to_expression` -- Value-to-AST conversion
- `lit_to_num`, `lit_to_string`, `lit_str_to_string`, `lit_str_to_atom` --
  Literal extraction
- `atom_to_string`, `wtf8_atom_to_atom`, `atom_to_str` -- Atom conversion
- `simple_tpl_to_string`, `convert_simple_tpl_to_str_expr`,
  `convert_concat_to_tpl_expr` -- Template literal conversion
- `string_to_prop_name`, `transform_shorthand_to_key_values` -- Property helpers
