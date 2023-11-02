use std::cmp;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Default)]
pub struct Row {
    string: String,
    len: usize,
}

impl From<&str> for Row {
    // todo 该方法在堆中复制一份数据，存在性能消耗，检查是否可以使用引用
    fn from(slice: &str) -> Self {
        let mut row = Row { 
            string: String::from(slice),
            len: 0,
        };
        row.len = row.string.graphemes(true).count();
        row
    }
}

impl Row {
    pub fn render(&self, start: usize, end: usize) -> String {
        let end = cmp::min(end, self.string.len());
        let start = cmp::min(start, end);

        // (&self.string[start..end]).to_string()
        self.string.graphemes(true).skip(start).take(end - start).collect()
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn insert(&mut self, at: usize, c: char) {
        if at >= self.len() {
            self.string.push(c);
            self.len += 1;
            return;
        }

        let mut result = String::new();
        let mut len: usize = 0;
        for (index, grapheme) in self.string.graphemes(true).enumerate() {
            if index == at {
                result.push(c);
                len += 1;
            }

            result.push_str(grapheme);
            len += 1;
        }

        self.string = result;
        self.len = len;
    }

    pub fn delete(&mut self, at: usize) {
        if at >= self.len() {
            return;
        }

        let mut result = String::new();
        let mut len: usize = 0;
        for (index, grapheme) in self.string.graphemes(true).enumerate() {
            if index == at {
                continue;
            }

            result.push_str(grapheme);
            len += 1;
        }

        self.string = result;
        self.len = len;
    }

    pub fn append(&mut self, next_row: &Row) {
        self.string = format!("{}{}", self.string, next_row.string);
        self.len = self.string.graphemes(true).count();
    }

    pub fn split(&mut self, x: usize) -> Self {
        let mut result = String::new();
        let mut result_len: usize = 0;

        let mut splitted_row = String::new();
        let mut splitted_len: usize = 0;

        for (index, grapheme) in self.string.graphemes(true).enumerate() {
            if index < x {
                result.push_str(grapheme);
                result_len += 1;
            } else {
                splitted_row.push_str(grapheme);
                splitted_len += 1;
            }
        }

        self.string = result;
        self.len = result_len;

        Self {
            string: splitted_row,
            len: splitted_len,
        }
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.string.as_bytes()
    }
}