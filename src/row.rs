use std::cmp;

#[derive(Default)]
pub struct Row {
    string: String,
}

impl From<&str> for Row {
    // todo 该方法在堆中复制一份数据，存在性能消耗，检查是否可以使用引用
    fn from(slice: &str) -> Self {
        Row { string: String::from(slice) }
    }
}

impl Row {
    pub fn render(&self, start: usize, end: usize) -> String {
        let end = cmp::min(end, self.string.len());
        let start = cmp::min(start, end);

        (&self.string[start..end]).to_string()
    }

    pub fn len(&self) -> usize {
        self.string.len() 
    }
}