pub struct CsvParser {
    delimiter: char,
    has_headers: bool,
}

impl CsvParser {
    pub fn new() -> Self {
        CsvParser {
            delimiter: ',',
            has_headers: true,
        }
    }

    pub fn parse(&self, input: &str) -> Result<Vec<Vec<String>>, CsvError> {
        // ... implementation matching documentation specs
        // ... existing code ...
    }
}