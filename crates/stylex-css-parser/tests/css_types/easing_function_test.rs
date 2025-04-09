use stylex_css_parser::{base_types::SubString, css_types::easing_function::parse_easing_function};

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn parses_valid_css_easing_function_types_strings_correctly() {
    // Test linear easing function
    let mut input1 = SubString::new("linear(1, 2)");
    let result1 = parse_easing_function().run(&mut input1);
    assert!(result1.is_ok());
    assert_eq!(result1.unwrap().to_string(), "linear(1, 2)");

    let mut input2 = SubString::new("linear(1.5, 2  ,   3)");
    let result2 = parse_easing_function().run(&mut input2);
    assert!(result2.is_ok());
    assert_eq!(result2.unwrap().to_string(), "linear(1.5, 2, 3)");

    let mut input3 = SubString::new("linear(  .5  , 2, 3,4  )");
    let result3 = parse_easing_function().run(&mut input3);
    assert!(result3.is_ok());
    assert_eq!(result3.unwrap().to_string(), "linear(0.5, 2, 3, 4)");

    // Test cubic bezier keywords
    let mut input4 = SubString::new("ease-in");
    let result4 = parse_easing_function().run(&mut input4);
    assert!(result4.is_ok());
    assert_eq!(result4.unwrap().to_string(), "ease-in");

    let mut input5 = SubString::new("ease-out");
    let result5 = parse_easing_function().run(&mut input5);
    assert!(result5.is_ok());
    assert_eq!(result5.unwrap().to_string(), "ease-out");

    let mut input6 = SubString::new("ease-in-out");
    let result6 = parse_easing_function().run(&mut input6);
    assert!(result6.is_ok());
    assert_eq!(result6.unwrap().to_string(), "ease-in-out");

    let mut input7 = SubString::new("ease");
    let result7 = parse_easing_function().run(&mut input7);
    assert!(result7.is_ok());
    assert_eq!(result7.unwrap().to_string(), "ease");

    // Test steps keywords
    let mut input8 = SubString::new("step-start");
    let result8 = parse_easing_function().run(&mut input8);
    assert!(result8.is_ok());
    assert_eq!(result8.unwrap().to_string(), "step-start");

    let mut input9 = SubString::new("step-end");
    let result9 = parse_easing_function().run(&mut input9);
    assert!(result9.is_ok());
    assert_eq!(result9.unwrap().to_string(), "step-end");

    // Test steps easing function
    let mut input10 = SubString::new("steps(1, start)");
    let result10 = parse_easing_function().run(&mut input10);
    assert!(result10.is_ok());
    assert_eq!(result10.unwrap().to_string(), "steps(1, start)");

    let mut input11 = SubString::new("steps(1   ,     start)");
    let result11 = parse_easing_function().run(&mut input11);
    assert!(result11.is_ok());
    assert_eq!(result11.unwrap().to_string(), "steps(1, start)");

    let mut input12 = SubString::new("steps(1,end)");
    let result12 = parse_easing_function().run(&mut input12);
    assert!(result12.is_ok());
    assert_eq!(result12.unwrap().to_string(), "steps(1, end)");

    // Test cubic bezier easing function
    let mut input13 = SubString::new("cubic-bezier(1,1,1,1)");
    let result13 = parse_easing_function().run(&mut input13);
    assert!(result13.is_ok());
    assert_eq!(result13.unwrap().to_string(), "cubic-bezier(1, 1, 1, 1)");

    let mut input14 = SubString::new("cubic-bezier( 1.5 ,    1 ,    .1 , 1 )");
    let result14 = parse_easing_function().run(&mut input14);
    assert!(result14.is_ok());
    assert_eq!(
      result14.unwrap().to_string(),
      "cubic-bezier(1.5, 1, 0.1, 1)"
    );
  }

  #[test]
  fn fails_to_parse_invalid_css_easing_function_types_strings() {
    let mut input1 = SubString::new("linear(1 2 3)");
    let result1 = parse_easing_function().run(&mut input1);
    assert!(result1.is_err());

    let mut input2 = SubString::new("cubic-bezier(1, 2, 3)");
    let result2 = parse_easing_function().run(&mut input2);
    assert!(result2.is_err());

    let mut input3 = SubString::new("cubic-bezier(1, 2, 3, 4, 5)");
    let result3 = parse_easing_function().run(&mut input3);
    assert!(result3.is_err());

    let mut input4 = SubString::new("cubic-bezier(1 .25 1 .25)");
    let result4 = parse_easing_function().run(&mut input4);
    assert!(result4.is_err());

    let mut input5 = SubString::new("out-ease");
    let result5 = parse_easing_function().run(&mut input5);
    assert!(result5.is_err());

    let mut input6 = SubString::new("linear()");
    let result6 = parse_easing_function().run(&mut input6);
    assert!(result6.is_err());

    let mut input7 = SubString::new("steps(1, 2)");
    let result7 = parse_easing_function().run(&mut input7);
    assert!(result7.is_err());
  }
}
