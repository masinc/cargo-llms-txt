//! A complex test library
//! 
//! This library contains various Rust constructs including generics, lifetimes, and complex traits.

use serde::{Deserialize, Serialize};

/// A complex struct with generics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplexStruct<T, U> 
where
    T: Clone + std::fmt::Display,
    U: std::fmt::Debug,
{
    /// Generic field T
    pub data: T,
    /// Generic field U
    pub metadata: U,
    /// Optional field
    pub optional: Option<String>,
}

impl<T, U> ComplexStruct<T, U> 
where
    T: Clone + std::fmt::Display,
    U: std::fmt::Debug,
{
    /// Creates a new ComplexStruct
    pub fn new(data: T, metadata: U) -> Self {
        Self {
            data,
            metadata,
            optional: None,
        }
    }
    
    /// Sets the optional field
    pub fn with_optional(mut self, optional: String) -> Self {
        self.optional = Some(optional);
        self
    }
    
    /// Gets a reference to the data with lifetime
    pub fn get_data(&self) -> &T {
        &self.data
    }
}

/// A complex enum with various variants
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ComplexEnum<T> {
    /// Empty variant
    Empty,
    /// Single value variant
    Single(T),
    /// Multiple values variant
    Multiple(T, String, i32),
    /// Struct variant
    Struct {
        /// The value field
        value: T,
        /// The name field
        name: String,
        /// The active flag
        active: bool,
    },
}

/// A trait with associated types and lifetimes
pub trait ComplexTrait<'a, T> {
    /// Associated type
    type Output;
    
    /// A method with lifetime parameters
    fn process<'b>(&'b self, input: &'a T) -> Self::Output
    where
        'a: 'b;
    
    /// A default method
    fn default_behavior(&self) -> String {
        "Default behavior".to_string()
    }
}

/// Implementation for ComplexStruct
impl<'a, T, U> ComplexTrait<'a, T> for ComplexStruct<T, U>
where
    T: Clone + std::fmt::Display + 'a,
    U: std::fmt::Debug,
{
    type Output = String;
    
    fn process<'b>(&'b self, input: &'a T) -> Self::Output
    where
        'a: 'b,
    {
        format!("Processing: {} with {}", input, self.data)
    }
}

/// A generic function with complex bounds
pub fn complex_function<T, U, F>(data: T, transform: F) -> U
where
    T: Clone + std::fmt::Display,
    U: From<String>,
    F: Fn(T) -> String,
{
    let result = transform(data);
    U::from(result)
}

/// A function with lifetime parameters
pub fn lifetime_function<'a, 'b>(first: &'a str, second: &'b str) -> String 
where
    'a: 'b,
{
    format!("{} - {}", first, second)
}

/// A macro for demonstration
#[macro_export]
macro_rules! complex_macro {
    ($name:ident, $value:expr) => {
        let $name = ComplexStruct::new($value, "metadata".to_string());
    };
}