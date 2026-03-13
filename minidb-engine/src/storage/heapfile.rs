/// A heap file is a collection of pages that stores records.
///
/// This implementation provides basic functionality for managing pages in a heap file,
/// including creating new pages and accessing existing pages by index.
pub struct HeapFile {
    /// The number of pages in the heap file
    pub num_pages: usize,
}

impl HeapFile {
    /// Creates a new empty heap file
    pub fn new() -> Self {
        HeapFile { num_pages: 0 }
    }
}