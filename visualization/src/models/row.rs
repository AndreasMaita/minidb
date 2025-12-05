use core::fmt;

#[derive(Debug, Clone)]
pub struct MyRow {
    pub name: String,
    pub age: u8,
}

impl fmt::Display for MyRow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[name: {}, age: {}]", self.name, self.age)
    }
}
