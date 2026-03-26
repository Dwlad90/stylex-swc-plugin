use std::cmp::Ordering;

pub(crate) fn sort_pseudos(pseudos: &[String]) -> Vec<String> {
  if pseudos.len() < 2 {
    return pseudos.to_owned();
  }

  let mut acc: Vec<Vec<String>> = Vec::new();

  for pseudo in pseudos {
    if pseudo.starts_with("::") {
      acc.push(vec![pseudo.to_string()]);
    } else if let Some(last_element) = acc.last_mut() {
      if last_element.len() == 1 && !last_element[0].starts_with("::") {
        last_element.push(pseudo.to_string());
      } else {
        acc.push(vec![pseudo.to_string()]);
      }
    } else {
      acc.push(vec![pseudo.to_string()]);
    }
  }

  acc
    .into_iter()
    .flat_map(|pseudo| {
      if pseudo.len() > 1 {
        let mut sorted_pseudo = pseudo.clone();
        sorted_pseudo.sort();
        sorted_pseudo
      } else {
        pseudo
      }
    })
    .collect()
}

pub(crate) fn sort_at_rules(at_rules: &[String]) -> Vec<String> {
  let mut unsorted_at_rules = at_rules.to_vec();

  unsorted_at_rules.sort_by(|a, b| string_comparator(a, b));

  unsorted_at_rules
}

// a comparator function that sorts strings alphabetically
// but where `default` always comes first
fn string_comparator(a: &str, b: &str) -> std::cmp::Ordering {
  if a == "default" {
    return Ordering::Less;
  }
  if b == "default" {
    return Ordering::Greater;
  }
  a.cmp(b)
}
