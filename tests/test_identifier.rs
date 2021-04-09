use aws_arn::Identifier;
use proptest::prelude::*;
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// API Tests
// ------------------------------------------------------------------------------------------------

#[test]
fn test_reject_empty() {
    let result = Identifier::from_str("");
    assert!(result.is_err());
}

#[test]
fn test_new() {
    let result = Identifier::from_str("test-new");
    assert!(result.is_ok());
    println!("{:?}", result);
    assert_eq!(result.unwrap().to_string(), String::from("test-new"));
}

#[test]
fn test_is_valid() {
    assert!(Identifier::is_valid("a"));
    assert!(Identifier::is_valid("1"));
    assert!(Identifier::is_valid("a1"));
    assert!(Identifier::is_valid("a-1"));
    assert!(Identifier::is_valid("_"));
    assert!(Identifier::is_valid("."));
    assert!(Identifier::is_valid("a_"));
}

#[test]
fn test_is_not_valid() {
    assert!(!Identifier::is_valid(""));
    assert!(!Identifier::is_valid(" "));
    assert!(!Identifier::is_valid("\t"));
    assert!(!Identifier::is_valid("\r"));
    assert!(!Identifier::is_valid("\n"));
    assert!(!Identifier::is_valid("a a"));
    assert!(!Identifier::is_valid(":"));
    assert!(!Identifier::is_valid("/"));
}

// ------------------------------------------------------------------------------------------------
// Automated Property Tests
// ------------------------------------------------------------------------------------------------

proptest! {
    #[test]
    fn char_doesnt_crash(s in "\\PC") {
        let _ = Identifier::from_str(&s);
    }

    #[test]
    fn valid_values(s in r"[[[:ascii:]]&&[[:^cntrl:]]&&[^ :/]]+") {
        println!("valid_values {:?}", s);
        assert!(Identifier::from_str(&s).is_ok());
    }
}
