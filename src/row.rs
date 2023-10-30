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
        row.update_len(); 
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

    fn update_len(&mut self) {
        self.len = self.string.graphemes(true).count();
    }

    pub fn insert(&mut self, at: usize, c: char) {
        if at >= self.len() {
            self.string.push(c);
        } else {
            let mut result: String = self.string.graphemes(true).take(at).collect();
            let after: String = self.string.graphemes(true).skip(at).collect();
            
            result.push(c);
            result.push_str(&after);
            self.string = result;

        }
        self.update_len();
    }

    pub fn delete(&mut self, at: usize) {
        if at >= self.len() {
            return;
        } else {
            let mut result: String = self.string.graphemes(true).take(at).collect();
            let after: String = self.string.graphemes(true).skip(at + 1).collect();

            result.push_str(&after);
            self.string = result;

            self.update_len();
        }
    }

    pub fn append(&mut self, next_row: &Row) {
        self.string = format!("{}{}", self.string, next_row.string);
        self.update_len();
    }

    pub fn split(&mut self, x: usize) -> Self {
        let before = self.string.graphemes(true).take(x).collect();
        let after: String = self.string.graphemes(true).skip(x).collect();

        self.string = before;
        self.update_len();

        Self::from(&after[..])
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.string.as_bytes()
    }
}