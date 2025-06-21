use complex_project::*;

#[test]
fn test_complex_struct() {
    let complex = ComplexStruct::new("test".to_string(), 42u32);
    assert_eq!(complex.get_data(), "test");
}

#[test]
fn test_complex_enum() {
    let empty = ComplexEnum::<i32>::Empty;
    let single = ComplexEnum::Single(42);
    let multiple = ComplexEnum::Multiple(1, "test".to_string(), 99);
    
    assert_eq!(empty, ComplexEnum::Empty);
    assert_eq!(single, ComplexEnum::Single(42));
}

#[test]
fn test_complex_trait() {
    let complex = ComplexStruct::new("data".to_string(), true);
    let input = "input";
    let result = complex.process(&input);
    assert!(result.contains("Processing"));
}

#[test]
fn test_complex_function() {
    let result: String = complex_function(42, |x| format!("Number: {}", x));
    assert_eq!(result, "Number: 42");
}

#[test]
fn test_lifetime_function() {
    let first = "Hello";
    let second = "World";
    let result = lifetime_function(first, second);
    assert_eq!(result, "Hello - World");
}