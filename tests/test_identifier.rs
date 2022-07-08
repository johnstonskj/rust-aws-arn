use aws_arn::{Identifier, IdentifierLike};
use proptest::prelude::*;
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// API Tests
// ------------------------------------------------------------------------------------------------

#[test]
fn test_identifier_reject_empty() {
    let result = Identifier::from_str("");
    assert!(result.is_err());
}

#[test]
fn test_identifier_new() {
    let result = Identifier::from_str("test-new");
    assert!(result.is_ok());
    println!("{:?}", result);
    assert_eq!(result.unwrap().to_string(), String::from("test-new"));
}

#[test]
fn test_identifier_is_valid() {
    assert!(Identifier::is_valid("a"));
    assert!(Identifier::is_valid("1"));
    assert!(Identifier::is_valid("a1"));
    assert!(Identifier::is_valid("a$1"));
    assert!(Identifier::is_valid("a-1"));
    assert!(Identifier::is_valid("_"));
    assert!(Identifier::is_valid("."));
    assert!(Identifier::is_valid("a_"));

    let id = Identifier::new_unchecked("a");
    assert!(!id.is_any());
    assert!(!id.has_wildcards());
    assert!(id.is_plain());
}

#[test]
fn test_identifier_is_valid_wildcard() {
    assert!(Identifier::is_valid("*"));
    assert!(Identifier::new_unchecked("*").is_any());
    assert!(Identifier::new_unchecked("***").is_any());

    assert!(Identifier::is_valid("a?b"));
    assert!(Identifier::new_unchecked("a?b").has_wildcards());

    assert!(Identifier::is_valid("ab*"));
    assert!(Identifier::new_unchecked("ab*").has_wildcards());
}

#[test]
fn test_identifier_is_not_valid() {
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
    fn proptest_identifier_char_doesnt_crash(s in "\\PC") {
        let _ = Identifier::from_str(&s);
    }

    #[test]
    fn proptest_identifier_valid_values(s in r"[[[:ascii:]]&&[[:^cntrl:]]&&[^ :/]]+") {
        println!("valid_values {:?}", s);
        assert!(Identifier::from_str(&s).is_ok());
    }
}
