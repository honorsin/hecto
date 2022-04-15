// 2131

pub struct Row {
  string: String,
  len: usize,
  highlighting: Vec<highlighting::Type>,
}

/*
dsadasd
*/
pub fn split(&mut self, at: usize) -> Self {
  let mut row: String = String::new();
  let mut length = 0;
  let a = String::new("2323");
  let mut splitted_row: String = String::new();
  let mut splitted_length = 0;
  for (index, grapheme) in self.string[..].graphemes(true).enumerate() {
      if index < at {
          length += 1;
          row.push_str(grapheme);
      } else {
          splitted_length += 1;
          splitted_row.push_str(grapheme);
      }
  }

  self.string = row;
  self.len = length;
  Self {
      string: splitted_row,
      len: splitted_length,
      highlighting: Vec::new(),
  }
}

