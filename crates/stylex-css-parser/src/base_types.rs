// #[derive(Debug, Clone)]
// pub struct SubString {
//   pub string: String,
//   pub start_index: usize,
//   pub end_index: usize,
// }

// impl SubString {
//   pub fn new(str: &str) -> Self {
//     SubString {
//       string: str.to_string(),
//       start_index: 0,
//       end_index: str.len().saturating_sub(1),
//     }
//   }

//   pub fn starts_with(&self, str: &str) -> bool {
//     // Check if there's enough room for the string
//     if self.start_index + str.len() > self.string.len() {
//       return false;
//     }

//     // Use a loop to avoid creating a new string
//     let self_chars: Vec<char> = self.string.chars().collect();
//     let str_chars: Vec<char> = str.chars().collect();

//     for i in 0..str_chars.len() {
//       let self_index = self.start_index + i;
//       if self_index > self.end_index || self_chars[self_index] != str_chars[i] {
//         return false;
//       }
//     }

//     true
//   }

//   pub fn first(&self) -> Option<char> {
//     if self.is_empty() {
//       None
//     } else {
//       self.string.chars().nth(self.start_index)
//     }
//   }

//   pub fn get(&self, relative_index: usize) -> Option<char> {
//     let absolute_index = self.start_index + relative_index;
//     if absolute_index <= self.end_index {
//       self.string.chars().nth(absolute_index)
//     } else {
//       None
//     }
//   }

//   pub fn to_string(&self) -> String {
//     if self.is_empty() {
//       String::new()
//     } else {
//       self.string[self.start_index..=self.end_index].to_string()
//     }
//   }

//   pub fn is_empty(&self) -> bool {
//     self.start_index > self.end_index
//   }
// }

#[derive(Debug)]
pub struct SubString<'a> {
  pub(crate) string: &'a str,
  pub(crate) start_index: usize,
  pub(crate) end_index: usize,
}

impl<'a> SubString<'a> {
  pub fn new(s: &'a str) -> Self {
    Self {
      string: s,
      start_index: 0,
      end_index: s.len(),
    }
  }

  pub fn is_empty(&self) -> bool {
    self.start_index >= self.end_index
  }

  pub fn first(&self) -> Option<char> {
    if self.is_empty() {
      None
    } else {
      self.string[self.start_index..].chars().next()
    }
  }

  pub fn starts_with(&self, prefix: &str) -> bool {
    if self.is_empty() {
      return false;
    }

    self.string[self.start_index..].starts_with(prefix)
  }
}

impl std::fmt::Display for SubString<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    if self.is_empty() {
      Ok(())
    } else {
      write!(f, "{}", &self.string[self.start_index..self.end_index])
    }
  }
}
