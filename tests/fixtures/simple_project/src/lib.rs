//! A simple test library
//! 
//! This library contains basic Rust constructs for testing.

/// A simple struct
#[derive(Debug, Clone)]
pub struct SimpleStruct {
    /// The name field
    pub name: String,
    /// The value field
    pub value: i32,
}

impl SimpleStruct {
    /// Creates a new SimpleStruct
    pub fn new(name: String, value: i32) -> Self {
        Self { name, value }
    }
    
    /// Gets the name
    pub fn get_name(&self) -> &str {
        &self.name
    }
}

/// A simple enum
#[derive(Debug, PartialEq)]
pub enum SimpleEnum {
    /// First variant
    First,
    /// Second variant with data
    Second(i32),
}

/// A simple trait
pub trait SimpleTrait {
    /// A required method
    fn simple_method(&self) -> String;
}

impl SimpleTrait for SimpleStruct {
    fn simple_method(&self) -> String {
        format!("SimpleStruct: {}", self.name)
    }
}

/// A simple function
pub fn simple_function(input: &str) -> String {
    format!("Hello, {}!", input)
}