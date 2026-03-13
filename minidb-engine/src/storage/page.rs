/// A slotted page, which is a fixed-size buffer that stores records.
///
/// The page size is 4096 bytes (as defined by the constant `PAGE_SIZE`).
/// Each record in the page has a header that contains metadata about the record,
/// such as its size and position in the page.
pub struct SlottedPage {
    /// The raw data buffer for the page
    pub data: [u8; 4096],
}

impl SlottedPage {
    /// Creates a new empty slotted page
    pub fn new() -> Self {
        SlottedPage {
            data: [0; 4096],
        }
    }
}