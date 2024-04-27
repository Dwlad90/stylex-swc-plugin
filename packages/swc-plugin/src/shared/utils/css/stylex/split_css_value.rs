pub(crate) fn split_value(str: Option<String>) -> (String, String, String, String) {
  let split_values = split_value_inner(str); // Assuming split_value returns a Vec<String>

  match &split_values[..] {
    [top] => (top.clone(), top.clone(), top.clone(), top.clone()),
    [top, right] => (top.clone(), right.clone(), top.clone(), right.clone()),
    [top, right, bottom] => (top.clone(), right.clone(), bottom.clone(), right.clone()),
    _ => (
      split_values[0].clone(),
      split_values[1].clone(),
      split_values[2].clone(),
      split_values[3].clone(),
    ),
  }
}

fn split_value_inner(str: Option<String>) -> Vec<String> {
  match str {
    None => vec![String::from("None")],
    Some(s) => {
      if s.parse::<i32>().is_ok() {
        vec![s]
      } else {
        panic!()
        // Handle non-numeric string case
      }
    }
  }
}
