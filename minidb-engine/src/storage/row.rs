/// A row represents a single record in the database.
///
/// This struct holds the data for one row, including its key and value.
pub struct Row {
    /// The key of the row
    pub key: u32,
    /// The value stored in the row
    pub value: Vec<u8>,
}

impl Row {
    /// Creates a new row with the given key and value
    pub fn new(key: u32, value: Vec<u8>) -> Self {
        Row { key, value }
    }
}