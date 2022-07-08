use aws_arn::{AccountIdentifier, IdentifierLike};
use proptest::prelude::*;
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// API Tests
// ------------------------------------------------------------------------------------------------

#[test]
fn test_account_identifier_reject_empty() {
    let result = AccountIdentifier::from_str("");
    assert!(result.is_err());
}

#[test]
fn test_account_identifier_new() {
    let result = AccountIdentifier::from_str("123456789*");
    assert!(result.is_ok());
    println!("{:?}", result);
    assert_eq!(result.unwrap().to_string(), String::from("123456789*"));
}

#[test]
fn test_account_identifier_is_valid() {
    assert!(AccountIdentifier::is_valid("123456789012"));

    let id = AccountIdentifier::new_unchecked("123456789012");
    assert!(!id.is_any());
    assert!(!id.has_wildcards());
    assert!(id.is_plain());
}

#[test]
fn test_account_identifier_is_valid_wildcard() {
    assert!(AccountIdentifier::is_valid("*"));
    assert!(AccountIdentifier::new_unchecked("*").is_any());
    assert!(AccountIdentifier::new_unchecked("***").is_any());

    assert!(AccountIdentifier::is_valid("2??6?*?8"));
    assert!(AccountIdentifier::new_unchecked("2??6?*?8").has_wildcards());

    assert!(AccountIdentifier::is_valid("12345*"));
    assert!(AccountIdentifier::new_unchecked("12345*").has_wildcards());
}

#[test]
fn test_account_identifier_from_str_wildcard() {
    assert!(AccountIdentifier::from_str("*").is_ok());
    assert!(AccountIdentifier::from_str("2??6?*?8").is_ok());
}

#[test]
fn test_account_identifier_is_not_valid() {
    assert!(!AccountIdentifier::is_valid(""));
    assert!(!AccountIdentifier::is_valid("123456789"));
    assert!(!AccountIdentifier::is_valid(" "));
    assert!(!AccountIdentifier::is_valid("\t"));
    assert!(!AccountIdentifier::is_valid("\r"));
    assert!(!AccountIdentifier::is_valid("\n"));
    assert!(!AccountIdentifier::is_valid("a"));
    assert!(!AccountIdentifier::is_valid("a a"));
    assert!(!AccountIdentifier::is_valid(":"));
    assert!(!AccountIdentifier::is_valid("/"));
}

// ------------------------------------------------------------------------------------------------
// Automated Property Tests
// ------------------------------------------------------------------------------------------------

proptest! {
    #[test]
    fn proptest_account_identifier_char_doesnt_crash(s in "\\PC") {
        let _ = AccountIdentifier::from_str(&s);
    }

//    #[test]
//    fn proptest_account_identifier_valid_values(s in r"[0-9]{12}|(?:[0-9][?*]|)") {
//        println!("valid_values {:?}", s);
//        assert!(AccountIdentifier::from_str(&s).is_ok());
//    }
}
