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
}