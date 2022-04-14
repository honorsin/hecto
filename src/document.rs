use crate::Position;
use crate::Row;
use crate::SearchDirection;
use std::fs;
use std::io::{Error, Write};
#[derive(Default)]
pub struct Document {
    pub rows: Vec<Row>,
    pub file_name: Option<String>,
    dirty: bool,
}

impl Document {
    pub fn open(filename: &str) -> Result<Self, std::io::Error> {
        let mut rows = Vec::new();
        let contents = fs::read_to_string(filename)?;

        for value in contents.lines() {
            let mut row = Row::from(value);
            row.highlight();
            rows.push(row);
        }

        Ok(Self {
            rows,
            file_name: Some(String::from(filename)),
            dirty: false,
        })
    }

    pub fn save(&mut self) -> Result<(), Error> {
        if let Some(file_name) = &self.file_name {
            let mut file = fs::File::create(file_name)?;
            for row in &self.rows {
                file.write_all(row.as_bytes())?;
                file.write_all(b"\n")?;
            }
            self.dirty = false;
        }
        Ok(())
    }

    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    pub fn row(&self, index: usize) -> Option<&Row> {
        self.rows.get(index)
    }

    pub fn len(&self) -> usize {
        self.rows.len()
    }

    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    fn insert_newline(&mut self, at: &Position) {
        if at.y > self.len() {
            return;
        }
        if at.y == self.len() {
            self.rows.push(Row::default());
            return;
        }

        let current_row = &mut self.rows[at.y];
        let mut new_row = current_row.split(at.x);
        current_row.highlight();
        new_row.highlight();

        #[allow(clippy::integer_arithmetic)]
        self.rows.insert(at.y + 1, new_row);
    }

    pub fn insert(&mut self, at: &Position, c: char) {
        if at.y > self.rows.len() {
            return;
        }
        self.dirty = true;
        if c == '\n' {
            self.insert_newline(at);
            return;
        }

        if at.y == self.rows.len() {
            let mut row = Row::default();
            row.insert(0, c);
            row.highlight();
            self.rows.push(row);
        } else {
            let row = self.rows.get_mut(at.y).unwrap();
            row.insert(at.x, c);
        }
    }

    pub fn delete(&mut self, at: &Position) {
        if at.y < self.rows.len() {
            if at.x == self.rows.get_mut(at.y).unwrap().len() && at.y + 1 < self.len() {
                let next_row = self.rows.remove(at.y + 1);
                let row = self.rows.get_mut(at.y).unwrap();
                row.append(&next_row);
                row.highlight();
            } else {
                let row = self.rows.get_mut(at.y).unwrap();
                row.delete(at.x);
                row.highlight();
            }
        }
    }

    #[allow(clippy::indexing_slicing)]
    pub fn find(&self, query: &str, at: &Position, direction: SearchDirection) ->Option<Position> {
        if at.y >= self.rows.len() {
            return None;
        }
        let mut position = Position {x: at.x, y: at.y};
       
        let start = if direction == SearchDirection::Forward {
            at.y
        } else {
           0
        };

        let end = if direction == SearchDirection::Forward {
            self.rows.len()
        } else {
            at.y.saturating_add(1)
        };

        for _ in start..end {
            if let Some(row) = self.rows.get(position.y) {
                if let Some(x) = row.find(&query, position.x, direction) {
                    position.x = x;
                    return Some(position);
                }

                if direction == SearchDirection::Forward {
                    position.y = position.y.saturating_add(1);
                    position.x = 0;
                } else {
                    position.y = position.y.saturating_sub(1);
                    position.x = self.rows.get(position.y).unwrap().len();
                }
            } else {
                return None;
            }
        }
        None
    }
}
