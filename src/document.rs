use crate::Row;

#[derive(Default)]
pub struct Document {
    rows: Vec<Row>
}

impl Document {
    pub fn open(filename: &str) -> Result<Self, std::io::Error> {
        let mut rows = Vec::new();
        let contents = std::fs::read_to_string(filename)?;
        for line in contents.lines() {
            rows.push(Row::from(line));
        }
        Ok(Document { rows })
    }

    pub fn row(&self, index: usize) -> Option<&Row> {
        self.rows.get(index)
    }

    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }
}