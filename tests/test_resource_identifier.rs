use aws_arn::{IdentifierLike, ResourceIdentifier};
use proptest::prelude::*;
use std::ops::Deref;
use std::{collections::HashMap, iter::FromIterator, str::FromStr};

// ------------------------------------------------------------------------------------------------
// API Tests
// ------------------------------------------------------------------------------------------------

#[test]
fn test_resource_identifier_reject_empty() {
    let result = ResourceIdentifier::from_str("");
    assert!(result.is_err());
}

#[test]
fn test_resource_identifier_new() {
    let result = ResourceIdentifier::from_str("test-new");
    assert!(result.is_ok());
    println!("{:?}", result);
    assert_eq!(result.unwrap().to_string(), String::from("test-new"));
}

#[test]
fn test_resource_identifier_is_valid() {
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

    let id = ResourceIdentifier::new_unchecked("a");
    assert!(!id.is_any());
    assert!(!id.has_wildcards());
    assert!(id.is_plain());
}

#[test]
fn test_resource_identifier_is_valid_wildcard() {
    assert!(ResourceIdentifier::is_valid("*"));
    assert!(ResourceIdentifier::new_unchecked("*").is_any());
    assert!(ResourceIdentifier::new_unchecked("***").is_any());

    assert!(ResourceIdentifier::is_valid("a?b"));
    assert!(ResourceIdentifier::new_unchecked("a?b").has_wildcards());

    assert!(ResourceIdentifier::is_valid("ab*"));
    assert!(ResourceIdentifier::new_unchecked("ab*").has_wildcards());
}

#[test]
fn test_resource_identifier_is_valid_variable() {
    assert!(ResourceIdentifier::is_valid("${var}"));

    assert!(ResourceIdentifier::new_unchecked("${var}").has_variables());

    assert!(!ResourceIdentifier::new_unchecked("${var}").is_plain());
}

#[test]
fn test_resource_identifier_valid_replacement() {
    let id = ResourceIdentifier::new_unchecked("${greeting} ${name}!");
    let replacements: HashMap<String, String> =
        HashMap::from_iter(vec![("name".to_string(), "Simon".to_string())].into_iter());
    let new_id = id.replace_variables(&replacements).unwrap();
    assert_eq!(new_id.deref(), "${greeting} Simon!");
}

#[test]
fn test_resource_identifier_invalid_replacement() {
    let id = ResourceIdentifier::new_unchecked("${greeting} ${name}!");
    let replacements: HashMap<String, String> =
        HashMap::from_iter(vec![("name".to_string(), "bad\nвал".to_string())].into_iter());
    let new_id = id.replace_variables(&replacements);
    assert!(new_id.is_err());
}

#[test]
fn test_resource_identifier_is_not_valid() {
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
    fn proptest_resource_identifier_char_doesnt_crash(s in "\\PC") {
        let _ = ResourceIdentifier::from_str(&s);
    }

    #[test]
    fn proptest_resource_identifier_valid_values(s in r"[[[:ascii:]]&&[[:^cntrl:]]]+") {
        println!("valid_values {:?}", s);
        assert!(ResourceIdentifier::from_str(&s).is_ok());
    }
}
