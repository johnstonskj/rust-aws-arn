/*!
* Provides types, builders, and other helpers to manipulate AWS
* [Amazon Resource Name (ARN)](https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html)
* strings.
*
* The ARN is a key component of all AWS service APIs and yet nearly all client toolkits treat it
* simply as a string. While this may be a reasonable and expedient decision, it seems there might
* be a need to not only ensure correctness of ARNs with validators but also constructors that allow
* making these strings correclt in the first place.
*
* # ARN Types
*
* This crate provides a number of levels of ARN manipulation, the first is the direct construction
* of an ARN type using the core `ARN`, `Identifier`, and `ResourceIdentifier` types.
*
* ```rust
* use aws_arn::{ARN, ResourceIdentifier};
* use aws_arn::known::{Partition, Service};
* use std::str::FromStr;
*
* let arn = ARN {
*     partition: Some(Partition::default().into()),
*     service: Service::S3.into(),
*     region: None,
*     account_id: None,
*     resource: ResourceIdentifier::from_str("mythings/thing-1").unwrap()
* };
* ```
*
* In the example above, as we are defining a minimal ARN we could use one of the defined constructor
* functions.
*
* ```rust
* use aws_arn::{ARN, ResourceIdentifier};
* use aws_arn::known::Service;
* use std::str::FromStr;
*
* let arn = ARN::aws(
*     Service::S3.into(),
*     ResourceIdentifier::from_str("mythings/thing-1").unwrap()
* );
* ```
*
* Alternatively, using `FromStr,` you can parse an existing ARN string into an ARN value.
*
* ```rust
* use aws_arn::ARN;
* use std::str::FromStr;
*
* let arn: ARN = "arn:aws:s3:::mythings/thing-1".parse().expect("didn't look like an ARN");
* ```
*
* Another approach is to use a more readable *builder* which also allows you to ignore those fields
* in the ARN you don't always need and uses a more fluent style of ARN construction.
*
* ```rust
* use aws_arn::builder::{ArnBuilder, ResourceBuilder};
* use aws_arn::known::{Partition, Service};
* use aws_arn::{ARN, Identifier};
* use std::str::FromStr;
*
* let arn: ARN = ArnBuilder::service_id(Service::S3.into())
*     .resource(ResourceBuilder::named(Identifier::from_str("mythings").unwrap())
*         .resource_name(Identifier::new_unchecked("my-layer"))
*         .build_resource_path())
*     .in_partition_id(Partition::Aws.into())
*     .into();
* ```
*
* Finally, it is possible to use resource-type specific functions that allow an even more direct and
* simple construction (module `aws_arn::builder::{service}` - *service builder functions*, although
* at this time there are few supported services.
*
* ```rust
* use aws_arn::builder::s3;
* use aws_arn::Identifier;
* use std::str::FromStr;
*
* let arn = s3::object(
*     Identifier::from_str("mythings").unwrap(),
*     Identifier::from_str("thing-1").unwrap(),
* );
* ```
*
* For more, see the AWS documentation for [Amazon Resource Name
* (ARN)](https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html) documentation.
*
* # Optional Features
*
* This crate has attempted to be as lean as possible, with a really minimal set of dependencies,
* we have include the following capabilities as optional features.
*
* * `builders` adds the builder module. This feature is enabled by default, it also requires the
*   `known` feature.
* * `known` adds a module containing enums for partitions, regions, and services.
*   This feature is enabled by default.
* * `serde_support` adds derived `Serialize` and `Deserialize` implementations for the `ARN` and
*   `Resource` types. This feature is enabled by default.
*
*
*/

// ------------------------------------------------------------------------------------------------
// Preamble
// ------------------------------------------------------------------------------------------------

#![warn(
    // ---------- Stylistic
    future_incompatible,
    nonstandard_style,
    rust_2018_idioms,
    trivial_casts,
    trivial_numeric_casts,
    // ---------- Public
    missing_debug_implementations,
    missing_docs,
    unreachable_pub,
    // ---------- Unsafe
    unsafe_code,
    // ---------- Unused
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results,
)]

#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize};

use std::fmt::{Debug, Display, Error, Formatter};
use std::ops::Deref;
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// A string value that is used to capture the partition, service, and region components
/// of an ARN. These are ASCII only, may not include control characters, spaces, '/', or ':'.
///
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde_support", derive(Deserialize, Serialize))]
pub struct Identifier(String);

///
/// A string value that is used to capture the account ID component
/// of an ARN. These are ASCII digits only and a fixed length of 12 characters.
///
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde_support", derive(Deserialize, Serialize))]
pub struct AccountIdentifier(String);

///
/// A string value that is used to capture the resource component of an ARN. These are ASCII only,
/// may not include control characters but unlike `Identifier` they may include spaces, '/', and ':'.
///
/// > *The content of this part of the ARN varies by service. A resource identifier can be the name
/// > or ID of the resource (for example, `user/Bob` or `instance/i-1234567890abcdef0`) or a
/// > resource path. For example, some resource identifiers include a parent resource
/// > (`sub-resource-type/parent-resource/sub-resource`) or a qualifier such as a version
/// > (`resource-type:resource-name:qualifier`).*
///
/// > *Some resource ARNs can include a path. For example, in Amazon S3, the resource identifier
/// > is an object name that can include slashes ('/') to form a path. Similarly, IAM user names
/// > and group names can include paths.*
///
/// > *In some circumstances, paths can include a wildcard character, namely an asterisk ('*').*
///
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde_support", derive(Deserialize, Serialize))]
pub struct ResourceIdentifier(String);

///
/// Amazon Resource Names (ARNs) uniquely identify AWS resources. We require an ARN when you
/// need to specify a resource unambiguously across all of AWS, such as in IAM policies,
/// Amazon Relational Database Service (Amazon RDS) tags, and API calls.
///
/// The following are the general formats for ARNs; the specific components and values used
/// depend on the AWS service.
///
/// ```text
/// arn:partition:service:region:account-id:resource-id
/// arn:partition:service:region:account-id:resource-type/resource-id
/// arn:partition:service:region:account-id:resource-type:resource-id
/// ```
///
/// From [ARN Format](https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html#arns-syntax)
///
#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde_support", derive(Deserialize, Serialize))]
pub struct ARN {
    /// The partition that the resource is in. For standard AWS Regions, the partition is` aws`.
    /// If you have resources in other partitions, the partition is `aws-partitionname`. For
    /// example, the partition for resources in the China partition is `aws-cn`. The module
    /// `known::partition` provides common values as constants (if the `known` feature is
    /// enabled).
    pub partition: Option<Identifier>,
    /// The service namespace that identifies the AWS. The module `known::service` provides
    //  common values as constants (if the `known` feature is enabled).
    pub service: Identifier,
    /// The Region that the resource resides in. The ARNs for some resources do not require
    /// a Region, so this component might be omitted. The module `known::region` provides
    /// common values as constants (if the `known` feature is enabled).
    pub region: Option<Identifier>,
    /// The ID of the AWS account that owns the resource, without the hyphens. For example,
    /// `123456789012`. The ARNs for some resources don't require an account number, so this
    /// component may be omitted.
    pub account_id: Option<AccountIdentifier>,
    /// The content of this part of the ARN varies by service. A resource identifier can
    /// be the name or ID of the resource (for example, `user/Bob` or
    /// `instance/i-1234567890abcdef0`) or a resource path. For example, some resource
    /// identifiers include a parent resource
    /// (`sub-resource-type/parent-resource/sub-resource`) or a qualifier such as a
    /// version (`resource-type:resource-name:qualifier`).
    pub resource: ResourceIdentifier,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

const ARN_PREFIX: &str = "arn";

const PART_SEPARATOR: char = ':';

const PATH_SEPARATOR: char = '/';

const STRING_WILD_ANY: &str = "*";

const CHAR_WILD_ONE: char = '?';

const CHAR_WILD_ANY: char = '*';

// ------------------------------------------------------------------------------------------------

impl Default for Identifier {
    fn default() -> Self {
        Self(String::default())
    }
}

impl Display for Identifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for Identifier {
    type Err = ArnError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if Self::is_valid(s) {
            Ok(Self(s.to_string()))
        } else {
            Err(ArnError::InvalidIdentifier(s.to_string()))
        }
    }
}

impl From<Identifier> for String {
    fn from(v: Identifier) -> Self {
        v.0
    }
}

impl Deref for Identifier {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Identifier {
    /// Construct a new `Identifier` from the provided string **without** checking it's validity.
    /// This can be a useful method to improve performance for statically, or well-known, values;
    /// however, in general `FromStr::from_str` should be used.
    pub fn new_unchecked(s: &str) -> Self {
        Self(s.to_string())
    }

    /// Returns `true` if the provided string is a valid `Identifier` value, else `false`.
    pub fn is_valid(s: &str) -> bool {
        !s.is_empty()
            && s.chars().all(|c| {
                c > '\u{1F}'
                    && c < '\u{7F}'
                    && c != ' '
                    && c != PATH_SEPARATOR
                    && c != PART_SEPARATOR
            })
    }

    /// Construct an identifier that represents *any*.
    pub fn any() -> Self {
        Self(STRING_WILD_ANY.to_string())
    }

    /// Return `true` if this is simply the *any* wildcard, else `false`.
    pub fn is_any(&self) -> bool {
        self.0 == STRING_WILD_ANY
    }

    /// Returns `true` if this identifier contains any wildcard characeters,
    /// else `false`.
    pub fn has_wildcards(&self) -> bool {
        self.0
            .chars()
            .any(|c| c == CHAR_WILD_ONE || c == CHAR_WILD_ANY)
    }
}

// ------------------------------------------------------------------------------------------------

impl Default for AccountIdentifier {
    fn default() -> Self {
        Self(String::default())
    }
}

impl Display for AccountIdentifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for AccountIdentifier {
    type Err = ArnError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if Self::is_valid(s) {
            Ok(Self(s.to_string()))
        } else {
            Err(ArnError::InvalidAccountId(s.to_string()))
        }
    }
}

impl From<AccountIdentifier> for String {
    fn from(v: AccountIdentifier) -> Self {
        v.0
    }
}

impl From<AccountIdentifier> for ARN {
    fn from(account: AccountIdentifier) -> Self {
        ARN::from_str(&format!("arn:aws:iam::{}:root", account)).unwrap()
    }
}

impl Deref for AccountIdentifier {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AccountIdentifier {
    /// Construct a new `AccountIdentifier` from the provided string **without** checking it's validity.
    /// This can be a useful method to improve performance for statically, or well-known, values;
    /// however, in general `FromStr::from_str` should be used.
    pub fn new_unchecked(s: &str) -> Self {
        Self(s.to_string())
    }

    /// Returns `true` if the provided string is a valid `Identifier` value, else `false`.
    pub fn is_valid(s: &str) -> bool {
        (s.len() == 12 && s.chars().all(|c| c.is_ascii_digit()))
            || (s.len() <= 12
                && s.chars()
                    .all(|c| c.is_ascii_digit() || c == CHAR_WILD_ONE || c == CHAR_WILD_ANY))
    }

    /// Construct an account identifier that represents *any*.
    pub fn any() -> Self {
        Self(STRING_WILD_ANY.to_string())
    }

    /// Return `true` if this is simply the *any* wildcard, else `false`.
    pub fn is_any(&self) -> bool {
        self.0 == STRING_WILD_ANY
    }

    /// Returns `true` if this identifier contains any wildcard characeters,
    /// else `false`.
    pub fn has_wildcards(&self) -> bool {
        self.0
            .chars()
            .any(|c| c == CHAR_WILD_ONE || c == CHAR_WILD_ANY)
    }
}

// ------------------------------------------------------------------------------------------------

impl Default for ResourceIdentifier {
    fn default() -> Self {
        Self(String::default())
    }
}

impl Display for ResourceIdentifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for ResourceIdentifier {
    type Err = ArnError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if Self::is_valid(s) {
            Ok(Self(s.to_string()))
        } else {
            Err(ArnError::InvalidResource(s.to_string()))
        }
    }
}

impl From<ResourceIdentifier> for String {
    fn from(v: ResourceIdentifier) -> Self {
        v.0
    }
}

impl From<Identifier> for ResourceIdentifier {
    fn from(v: Identifier) -> Self {
        ResourceIdentifier::new_unchecked(&v.0)
    }
}

impl Deref for ResourceIdentifier {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ResourceIdentifier {
    /// Construct a new `ResourceIdentifier` from the provided string **without** checking it's
    /// validity. This can be a useful method to improve performance for statically, or well-known,
    /// values; however, in general `FromStr::from_str` should be used.
    pub fn new_unchecked(s: &str) -> Self {
        Self(s.to_string())
    }

    /// Returns `true` if the provided string is a valid `ResourceIdentifier` value, else `false`.
    pub fn is_valid(s: &str) -> bool {
        !s.is_empty() && s.chars().all(|c| c > '\u{1F}' && c < '\u{7F}')
    }

    /// Construct a resource identifier that represents *any*.
    pub fn any() -> Self {
        Self(STRING_WILD_ANY.to_string())
    }

    /// Return `true` if this is simply the *any* wildcard, else `false`.
    pub fn is_any(&self) -> bool {
        self.0 == STRING_WILD_ANY
    }

    /// Returns `true` if this identifier contains any wildcard characeters,
    /// else `false`.
    pub fn has_wildcards(&self) -> bool {
        self.0
            .chars()
            .any(|c| c == CHAR_WILD_ONE || c == CHAR_WILD_ANY)
    }

    /// Construct a resource identifier, as a path, using the `Identifier` path components.
    pub fn from_id_path(path: &[Identifier]) -> Self {
        Self::new_unchecked(
            &path
                .iter()
                .map(Identifier::to_string)
                .collect::<Vec<String>>()
                .join(&PATH_SEPARATOR.to_string()),
        )
    }

    /// Construct a resource identifier, as a qualified ID, using the `Identifier` path components.
    pub fn from_qualified_id(qualified: &[Identifier]) -> Self {
        Self::new_unchecked(
            &qualified
                .iter()
                .map(Identifier::to_string)
                .collect::<Vec<String>>()
                .join(&PART_SEPARATOR.to_string()),
        )
    }

    /// Construct a resource identifier, as a path, using the `ResourceIdentifier` path components.
    pub fn from_path(path: &[ResourceIdentifier]) -> Self {
        Self::new_unchecked(
            &path
                .iter()
                .map(ResourceIdentifier::to_string)
                .collect::<Vec<String>>()
                .join(&PATH_SEPARATOR.to_string()),
        )
    }

    /// Construct a resource identifier, as a qualified ID, using the `ResourceIdentifier` path components.
    pub fn from_qualified(qualified: &[ResourceIdentifier]) -> Self {
        Self::new_unchecked(
            &qualified
                .iter()
                .map(ResourceIdentifier::to_string)
                .collect::<Vec<String>>()
                .join(&PART_SEPARATOR.to_string()),
        )
    }

    /// Return `true` if this identifier contains path separator characters, else `false`.
    pub fn contains_path(&self) -> bool {
        self.0.contains(PATH_SEPARATOR)
    }

    /// Return the list of path components when split using the path separator character.
    pub fn path_split(&self) -> Vec<ResourceIdentifier> {
        self.0
            .split(PATH_SEPARATOR)
            .map(ResourceIdentifier::new_unchecked)
            .collect()
    }

    /// Return `true` if this identifier contains qualifier separator characters, else `false`.
    pub fn contains_qualified(&self) -> bool {
        self.0.contains(PART_SEPARATOR)
    }

    /// Return the list of path components when split using the qualifier separator character.
    pub fn qualifier_split(&self) -> Vec<ResourceIdentifier> {
        self.0
            .split(PART_SEPARATOR)
            .map(ResourceIdentifier::new_unchecked)
            .collect()
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for ARN {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(
            f,
            "{}",
            vec![
                ARN_PREFIX.to_string(),
                self.partition
                    .as_ref()
                    .unwrap_or(&known::Partition::default().into())
                    .to_string(),
                self.service.to_string(),
                self.region
                    .as_ref()
                    .unwrap_or(&Identifier::default())
                    .to_string(),
                self.account_id
                    .as_ref()
                    .unwrap_or(&AccountIdentifier::default())
                    .to_string(),
                self.resource.to_string()
            ]
            .join(&PART_SEPARATOR.to_string())
        )
    }
}

impl FromStr for ARN {
    type Err = ArnError;

    ///
    /// Format:
    ///
    /// * `arn:partition:service:region:account-id: | resource part |`
    ///
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts: Vec<&str> = s.split(PART_SEPARATOR).collect();
        if parts.len() < 6 {
            Err(ArnError::TooFewComponents)
        } else if parts[0] != ARN_PREFIX {
            Err(ArnError::MissingPrefix)
        } else {
            let new_arn = ARN {
                partition: if parts[1].is_empty() {
                    None
                } else if parts[1] == "aws" || parts[1].starts_with("aws-") {
                    Some(Identifier::from_str(parts[1])?)
                } else {
                    return Err(ArnError::InvalidPartition);
                },
                service: Identifier::from_str(parts[2])?,
                region: if parts[3].is_empty() {
                    None
                } else {
                    Some(Identifier::from_str(parts[3])?)
                },
                account_id: if parts[4].is_empty() {
                    None
                } else {
                    Some(AccountIdentifier::from_str(parts[4])?)
                },
                resource: {
                    let resource_parts: Vec<&str> = parts.drain(5..).collect();
                    ResourceIdentifier::from_str(&resource_parts.join(&PART_SEPARATOR.to_string()))?
                },
            };

            Ok(new_arn)
        }
    }
}

impl ARN {
    /// Construct a minimal `ARN` value with simply a service and resource.
    pub fn new(service: Identifier, resource: ResourceIdentifier) -> Self {
        Self {
            partition: None,
            service,
            region: None,
            account_id: None,
            resource,
        }
    }

    /// Construct a minimal `ARN` value with simply a service and resource in the `aws` partition.
    pub fn aws(service: Identifier, resource: ResourceIdentifier) -> Self {
        Self {
            partition: Some(known::Partition::default().into()),
            service,
            region: None,
            account_id: None,
            resource,
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

#[cfg(feature = "builders")]
pub mod builder;

#[cfg(feature = "known")]
pub mod known;

#[doc(hidden)]
mod error;
pub use crate::error::ArnError;
