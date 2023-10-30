use std::{fs, io::Write};

use crate::{Row, editor::Position};

#[derive(Default)]
pub struct Document {
    rows: Vec<Row>,
    pub filename: Option<String>,
}

impl Document {
    pub fn open(filename: &str) -> Result<Self, std::io::Error> {
        let mut rows = Vec::new();
        let contents = std::fs::read_to_string(filename)?;
        for line in contents.lines() {
            rows.push(Row::from(line));
        }
        Ok(Document { 
            rows,
            filename: Some(filename.to_string()),
        })
    }

    pub fn row(&self, index: usize) -> Option<&Row> {
        self.rows.get(index)
    }

    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    pub fn len(&self) -> usize {
        self.rows.len() 
    }

    pub fn insert_new_line(&mut self, at: &Position) {
        if at.y > self.len() {
            return;
        }

        if at.y == self.len() {
            self.rows.push(Row::default());
            return;
        }

        let new_line = self.rows.get_mut(at.y).unwrap().split(at.x);
        self.rows.insert(at.y + 1, new_line);
    }

    pub fn insert(&mut self, at: &Position, c: char) {
        if c == '\n' {
            self.insert_new_line(at);
            return;
        }

        if at.y == self.len() {
            let mut row = Row::default();
            row.insert(at.x, c);

            self.rows.push(row);
        } else {
            let row = self.rows.get_mut(at.y).unwrap();
            row.insert(at.x, c);
        }
    }

    pub fn delete(&mut self, at: &Position) {
        let len = self.len();
        if at.y >= len {
            return;
        } else if at.x == self.rows.get_mut(at.y).unwrap().len() && at.y < len - 1 {
            let next_row = self.rows.remove(at.y + 1);
            let row = self.rows.get_mut(at.y).unwrap();
            row.append(&next_row);
        } else {
            let row = self.rows.get_mut(at.y).unwrap();
            row.delete(at.x)
        }
    }

    pub fn save(&self) -> Result<(), std::io::Error> {
        
        if let Some(filename) = &self.filename {
            let mut file = fs::File::create(filename)?;
            
            for row in &self.rows {
                file.write_all(row.as_bytes())?;
                file.write_all(b"\n")?;
            }
        }

        Ok(())
    }
}