use aws_arn::ResourceIdentifier;
use proptest::prelude::*;
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// API Tests
// ------------------------------------------------------------------------------------------------

#[test]
fn test_reject_empty() {
    let result = ResourceIdentifier::from_str("");
    assert!(result.is_err());
}

#[test]
fn test_new() {
    let result = ResourceIdentifier::from_str("test-new");
    assert!(result.is_ok());
    println!("{:?}", result);
    assert_eq!(result.unwrap().to_string(), String::from("test-new"));
}

#[test]
fn test_is_valid() {
    assert!(ResourceIdentifier::is_valid("a"));
    assert!(ResourceIdentifier::is_valid("1"));
    assert!(ResourceIdentifier::is_valid("a1"));
    assert!(ResourceIdentifier::is_valid("a-1"));
    assert!(ResourceIdentifier::is_valid("_"));
    assert!(ResourceIdentifier::is_valid("."));
    assert!(ResourceIdentifier::is_valid("a_"));
    assert!(ResourceIdentifier::is_valid(" "));
    assert!(ResourceIdentifier::is_valid(":"));
    assert!(ResourceIdentifier::is_valid("/"));
}

#[test]
fn test_is_not_valid() {
    assert!(!ResourceIdentifier::is_valid(""));
    assert!(!ResourceIdentifier::is_valid("\t"));
    assert!(!ResourceIdentifier::is_valid("\r"));
    assert!(!ResourceIdentifier::is_valid("\n"));
}

// ------------------------------------------------------------------------------------------------
// Automated Property Tests
// ------------------------------------------------------------------------------------------------

proptest! {
    #[test]
    fn char_doesnt_crash(s in "\\PC") {
        let _ = ResourceIdentifier::from_str(&s);
    }

    #[test]
    fn valid_values(s in r"[[[:ascii:]]&&[[:^cntrl:]]]+") {
        println!("valid_values {:?}", s);
        assert!(ResourceIdentifier::from_str(&s).is_ok());
    }
}
