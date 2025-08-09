use swc_core::ecma::ast::{Expr, KeyValueProp, ObjectLit, Prop, PropName, PropOrSpread, Str};

#[derive(Debug, Clone)]
struct MediaQuery {
  // Fields for MediaQuery
}

impl MediaQuery {
  fn parse_to_end(_query: &str) -> Self {
    dbg!(&_query);
    unimplemented!("MediaQuery parsing is not implemented yet");
  }

  fn to_string(&self) -> String {
    unimplemented!("MediaQuery to_string is not implemented yet");
  }
}

pub fn last_media_query_wins_transform(styles: &[KeyValueProp]) -> Vec<KeyValueProp> {
  dfs_process_queries(styles, 0)
}

pub fn dfs_process_queries(obj: &[KeyValueProp], depth: i8) -> Vec<KeyValueProp> {
  let mut result = obj
    .iter()
    .map(|kv| {
      let key = kv.key.clone();
      let value = match kv.value.as_ref() {
        Expr::Object(obj_expr) => {
          let nested_props = dfs_process_queries(
            &obj_expr
              .props
              .iter()
              .filter_map(|p| match p {
                PropOrSpread::Spread(_spread_element) => {
                  unimplemented!("Spread elements are not supported in media queries")
                }
                PropOrSpread::Prop(prop) => {
                  if let Prop::KeyValue(kv) = prop.as_ref() {
                    Some(kv.clone())
                  } else {
                    None
                  }
                }
              })
              .collect::<Vec<_>>(),
            depth + 1,
          );
          Expr::Object(ObjectLit {
            span: obj_expr.span,
            props: nested_props
              .into_iter()
              .map(|kv| PropOrSpread::Prop(Box::new(Prop::from(kv))))
              .collect::<Vec<PropOrSpread>>(),
          })
        }
        _ => *kv.value.clone(),
      };
      KeyValueProp {
        key,
        value: Box::new(value),
      }
    })
    .collect::<Vec<KeyValueProp>>();
  dbg!(&result);

  // Only process if depth >= 1 and any key starts with "@media "
  if depth >= 1 {
    // Collect all keys that start with "@media "
    let media_keys: Vec<usize> = result
      .iter()
      .enumerate()
      .filter_map(|(i, kv)| {
        if let PropName::Str(ref s) = kv.key {
          if s.value.starts_with("@media ") {
            Some(i)
          } else {
            None
          }
        } else {
          None
        }
      })
      .collect();

    if !media_keys.is_empty() {
      // Build negations and accumulated_negations
      let mut negations = Vec::new();
      let mut accumulated_negations: Vec<Vec<MediaQuery>> = Vec::new();

      for idx in (1..media_keys.len()).rev() {
        // Skip last iteration
        let key_idx = media_keys[idx];
        let key_str = if let PropName::Str(ref s) = result[key_idx].key {
          s.value.to_string()
        } else {
          continue;
        };
        let media_query = MediaQuery::parse_to_end(&key_str);
        negations.push(media_query);
        accumulated_negations.push(negations.clone());
      }
      accumulated_negations.reverse();
      accumulated_negations.push(Vec::new());

      // Rewrite media keys
      for (i, key_idx) in media_keys.iter().enumerate() {
        let current_key_str = if let PropName::Str(ref s) = result[*key_idx].key {
          s.value.to_string()
        } else {
          continue;
        };
        let current_value = result[*key_idx].value.clone();

        let base_media_query = MediaQuery::parse_to_end(&current_key_str);
        let mut reversed_negations = accumulated_negations[i].clone();
        reversed_negations.reverse();

        let combined_query =
          combine_media_query_with_negations(base_media_query, &reversed_negations);
        let new_media_key = combined_query.to_string();

        // Update key in result
        result[*key_idx].key = PropName::Str(Str {
          value: new_media_key.into(),
          span: if let PropName::Str(ref s) = result[*key_idx].key {
            s.span
          } else {
            Default::default()
          },
          raw: None,
        });
        result[*key_idx].value = current_value;
      }
    }
  }

  result
}

fn combine_media_query_with_negations(current: MediaQuery, negations: &[MediaQuery]) -> MediaQuery {
  unimplemented!("Combining media queries with negations is not implemented yet");
  // if negations.is_empty() {
  //   return current;
  // }

  // // Assuming MediaQuery has a field `queries: MediaQueryAst`
  // // and MediaQueryAst is an enum with variants: Or { rules: Vec<MediaQueryAst> }, And { rules: Vec<MediaQueryAst> }, Not { rule: Box<MediaQueryAst> }, etc.

  // let combined_ast = match &current.queries {
  //   MediaQueryAst::Or { rules } => MediaQueryAst::Or {
  //     rules: rules
  //       .iter()
  //       .map(|rule| {
  //         let mut and_rules = vec![rule.clone()];
  //         and_rules.extend(negations.iter().map(|mq| MediaQueryAst::Not {
  //           rule: Box::new(mq.queries.clone()),
  //         }));
  //         MediaQueryAst::And { rules: and_rules }
  //       })
  //       .collect(),
  //   },
  //   _ => {
  //     let mut and_rules = vec![current.queries.clone()];
  //     and_rules.extend(negations.iter().map(|mq| MediaQueryAst::Not {
  //       rule: Box::new(mq.queries.clone()),
  //     }));
  //     MediaQueryAst::And { rules: and_rules }
  //   }
  // };

  // MediaQuery {
  //   queries: combined_ast,
  //   // ...other fields as needed...
  // }
}
