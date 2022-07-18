/*!
* Provides types, builders, and other helpers to manipulate AWS [Amazon
* Resource Name
* (ResourceName)](https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html)
* strings.
*
* The ResourceName is a key component of all AWS service APIs and yet nearly
* all client toolkits treat it simply as a string. While this may be a
* reasonable and expedient decision, it seems there might be a need to not
* only ensure correctness of ResourceNames with validators but also
* constructors that allow making these strings correclt in the first place.
*
* # ResourceName Types
*
* This crate provides a number of levels of ResourceName manipulation, the
* first is the direct construction of an ResourceName type using the core
* `ResourceName`, `Identifier`, `AccountIdentifier`, and `ResourceIdentifier`
* types.
*
* ```rust
* use aws_arn::{ResourceName, ResourceIdentifier};
* use aws_arn::known::{Partition, Service};
* use std::str::FromStr;
*
* let arn = ResourceName {
*     partition: Some(Partition::default().into()),
*     service: Service::S3.into(),
*     region: None,
*     account_id: None,
*     resource: ResourceIdentifier::from_str("mythings/thing-1").unwrap()
* };
* ```
*
* In the example above, as we are defining a minimal ResourceName we could use one of the defined constructor
* functions.
*
* ```rust
* use aws_arn::{ResourceName, ResourceIdentifier};
* use aws_arn::known::Service;
* use std::str::FromStr;
*
* let arn = ResourceName::aws(
*     Service::S3.into(),
*     ResourceIdentifier::from_str("mythings/thing-1").unwrap()
* );
* ```
*
* Alternatively, using `FromStr,` you can parse an existing ResourceName string into an ResourceName value.
*
* ```rust
* use aws_arn::ResourceName;
* use std::str::FromStr;
*
* let arn: ResourceName = "arn:aws:s3:::mythings/thing-1"
*     .parse()
*     .expect("didn't look like an ResourceName");
* ```
*
* Another approach is to use a more readable *builder* which also allows you to ignore those fields
* in the ResourceName you don't always need and uses a more fluent style of ResourceName construction.
*
* ```rust
* use aws_arn::builder::{ArnBuilder, ResourceBuilder};
* use aws_arn::known::{Partition, Service};
* use aws_arn::{ResourceName, Identifier, IdentifierLike};
* use std::str::FromStr;
*
* let arn: ResourceName = ArnBuilder::service_id(Service::S3.into())
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
* (ResourceName)](https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html) documentation.
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
* * `serde_support` adds derived `Serialize` and `Deserialize` implementations for the `ResourceName` and
*   `Resource` types. This feature is enabled by default.
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

use lazy_static::lazy_static;
use regex::{Captures, Regex};
#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};
use std::ops::Deref;
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

/// This trait is implemented by the `ResourceName` component types. It
/// represents a string-based identifier that is generally constructed using
/// `FromStr::from_str`.
///
pub trait IdentifierLike
where
    Self: Clone + Display + FromStr + Deref<Target = str>,
{
    /// Construct a new `Identifier` from the provided string **without** checking it's validity.
    /// This can be a useful method to improve performance for statically, or well-known, values;
    /// however, in general `FromStr::from_str` should be used.
    fn new_unchecked(s: &str) -> Self
    where
        Self: Sized;

    /// Returns `true` if the provided string is a valid `Identifier` value, else `false`.
    fn is_valid(s: &str) -> bool;

    /// Construct an account identifier that represents *any*.
    fn any() -> Self {
        Self::new_unchecked(STRING_WILD_ANY)
    }

    /// Return `true` if this is simply the *any* wildcard, else `false`.
    fn is_any(&self) -> bool {
        self.deref().chars().any(|c| c == CHAR_WILD_ANY)
    }

    /// Returns `true` if this identifier contains any wildcard characeters,
    /// else `false`.
    fn has_wildcards(&self) -> bool {
        self.deref()
            .chars()
            .any(|c| c == CHAR_WILD_ONE || c == CHAR_WILD_ANY)
    }

    /// Return `true` if this identifier has no wildcards, else `false`.
    fn is_plain(&self) -> bool {
        !self.has_wildcards()
    }
}

///
/// A string value that is used to capture the partition, service, and region components
/// of an ResourceName. These are ASCII only, may not include control characters, spaces, '/', or ':'.
///
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde_support", derive(Deserialize, Serialize))]
pub struct Identifier(String);

///
/// A string value that is used to capture the account ID component
/// of an ResourceName. These are ASCII digits only and a fixed length of 12 characters.
///
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde_support", derive(Deserialize, Serialize))]
pub struct AccountIdentifier(String);

///
/// A string value that is used to capture the resource component of an ResourceName. These are ASCII only,
/// may not include control characters but unlike `Identifier` they may include spaces, '/', and ':'.
///
/// > *The content of this part of the ResourceName varies by service. A resource identifier can be the name
/// > or ID of the resource (for example, `user/Bob` or `instance/i-1234567890abcdef0`) or a
/// > resource path. For example, some resource identifiers include a parent resource
/// > (`sub-resource-type/parent-resource/sub-resource`) or a qualifier such as a version
/// > (`resource-type:resource-name:qualifier`).*
///
/// > *Some resource ResourceNames can include a path. For example, in Amazon S3, the resource identifier
/// > is an object name that can include slashes ('/') to form a path. Similarly, IAM user names
/// > and group names can include paths.*
///
/// > *In some circumstances, paths can include a wildcard character, namely an asterisk ('*').*
///
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde_support", derive(Deserialize, Serialize))]
pub struct ResourceIdentifier(String);

///
/// Amazon Resource Names (ResourceNames) uniquely identify AWS resources. We require an ResourceName when you
/// need to specify a resource unambiguously across all of AWS, such as in IAM policies,
/// Amazon Relational Database Service (Amazon RDS) tags, and API calls.
///
/// The following are the general formats for ResourceNames; the specific components and values used
/// depend on the AWS service.
///
/// ```text
/// arn:partition:service:region:account-id:resource-id
/// arn:partition:service:region:account-id:resource-type/resource-id
/// arn:partition:service:region:account-id:resource-type:resource-id
/// ```
///
/// From [ResourceName Format](https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html#arns-syntax)
///
#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde_support", derive(Deserialize, Serialize))]
pub struct ResourceName {
    /// The partition that the resource is in. For standard AWS Regions, the partition is` aws`.
    /// If you have resources in other partitions, the partition is `aws-partitionname`. For
    /// example, the partition for resources in the China partition is `aws-cn`. The module
    /// `known::partition` provides common values as constants (if the `known` feature is
    /// enabled).
    pub partition: Option<Identifier>,
    /// The service namespace that identifies the AWS. The module `known::service` provides
    //  common values as constants (if the `known` feature is enabled).
    pub service: Identifier,
    /// The Region that the resource resides in. The ResourceNames for some resources do not require
    /// a Region, so this component might be omitted. The module `known::region` provides
    /// common values as constants (if the `known` feature is enabled).
    pub region: Option<Identifier>,
    /// The ID of the AWS account that owns the resource, without the hyphens. For example,
    /// `123456789012`. The ResourceNames for some resources don't require an account number, so this
    /// component may be omitted.
    pub account_id: Option<AccountIdentifier>,
    /// The content of this part of the ResourceName varies by service. A resource identifier can
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

const CHAR_ASCII_START: char = '\u{1F}';
const CHAR_ASCII_END: char = '\u{7F}';
const CHAR_SPACE: char = ' ';
const CHAR_WILD_ONE: char = '?';
const CHAR_WILD_ANY: char = '*';

const REQUIRED_COMPONENT_COUNT: usize = 6;

const PARTITION_AWS_PREFIX: &str = "aws";
const PARTITION_AWS_OTHER_PREFIX: &str = "aws-";

lazy_static! {
    static ref REGEX_VARIABLE: Regex = Regex::new(r"\$\{([^$}]+)\}").unwrap();
}

// ------------------------------------------------------------------------------------------------

impl Display for Identifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for Identifier {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if Self::is_valid(s) {
            Ok(Self(s.to_string()))
        } else {
            Err(Error::InvalidIdentifier(s.to_string()))
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

impl IdentifierLike for Identifier {
    fn new_unchecked(s: &str) -> Self {
        Self(s.to_string())
    }

    fn is_valid(s: &str) -> bool {
        !s.is_empty()
            && s.chars().all(|c| {
                c > CHAR_ASCII_START
                    && c < CHAR_ASCII_END
                    && c != CHAR_SPACE
                    && c != PATH_SEPARATOR
                    && c != PART_SEPARATOR
            })
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for AccountIdentifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for AccountIdentifier {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if Self::is_valid(s) {
            Ok(Self(s.to_string()))
        } else {
            Err(Error::InvalidAccountId(s.to_string()))
        }
    }
}

impl From<AccountIdentifier> for String {
    fn from(v: AccountIdentifier) -> Self {
        v.0
    }
}

impl From<AccountIdentifier> for ResourceName {
    fn from(account: AccountIdentifier) -> Self {
        ResourceName::from_str(&format!("arn:aws:iam::{}:root", account)).unwrap()
    }
}

impl Deref for AccountIdentifier {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl IdentifierLike for AccountIdentifier {
    fn new_unchecked(s: &str) -> Self {
        Self(s.to_string())
    }

    fn is_valid(s: &str) -> bool {
        (s.len() == 12 && s.chars().all(|c| c.is_ascii_digit()))
            || (!s.is_empty()
                && s.len() <= 12
                && s.chars()
                    .all(|c| c.is_ascii_digit() || c == CHAR_WILD_ONE || c == CHAR_WILD_ANY)
                && s.chars().any(|c| c == CHAR_WILD_ONE || c == CHAR_WILD_ANY))
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for ResourceIdentifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for ResourceIdentifier {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if Self::is_valid(s) {
            Ok(Self(s.to_string()))
        } else {
            Err(Error::InvalidResource(s.to_string()))
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

impl IdentifierLike for ResourceIdentifier {
    fn new_unchecked(s: &str) -> Self {
        Self(s.to_string())
    }

    fn is_valid(s: &str) -> bool {
        !s.is_empty() && s.chars().all(|c| c > '\u{1F}' && c < '\u{7F}')
    }

    fn is_plain(&self) -> bool {
        !self.has_wildcards() && !self.has_variables()
    }
}

impl ResourceIdentifier {
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

    /// Return `true` if the identifier contains variables of the form
    /// `${name}`, else `false`.
    pub fn has_variables(&self) -> bool {
        REGEX_VARIABLE.is_match(self.deref())
    }

    /// Replace any variables in the string with values from the context,
    /// returning a new value if the replacements result in a legal identifier
    /// string. The
    pub fn replace_variables<V>(&self, context: &HashMap<String, V>) -> Result<Self, Error>
    where
        V: Clone + Into<String>,
    {
        let new_text = REGEX_VARIABLE.replace_all(self.deref(), |caps: &Captures<'_>| {
            if let Some(value) = context.get(&caps[1]) {
                value.clone().into()
            } else {
                format!("${{{}}}", &caps[1])
            }
        });
        Self::from_str(&new_text)
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for ResourceName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
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

impl FromStr for ResourceName {
    type Err = Error;

    ///
    /// Format:
    ///
    /// * `arn:partition:service:region:account-id: | resource part |`
    ///
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts: Vec<&str> = s.split(PART_SEPARATOR).collect();
        if parts.len() < REQUIRED_COMPONENT_COUNT {
            Err(Error::TooFewComponents)
        } else if parts[0] != ARN_PREFIX {
            Err(Error::MissingPrefix)
        } else {
            let new_arn = ResourceName {
                partition: if parts[1].is_empty() {
                    None
                } else if parts[1] == PARTITION_AWS_PREFIX
                    || parts[1].starts_with(PARTITION_AWS_OTHER_PREFIX)
                {
                    Some(Identifier::from_str(parts[1])?)
                } else {
                    return Err(Error::InvalidPartition);
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

impl ResourceName {
    /// Construct a minimal `ResourceName` value with simply a service and resource.
    pub fn new(service: Identifier, resource: ResourceIdentifier) -> Self {
        Self {
            partition: None,
            service,
            region: None,
            account_id: None,
            resource,
        }
    }

    /// Construct a minimal `ResourceName` value with simply a service and resource in the `aws` partition.
    pub fn aws(service: Identifier, resource: ResourceIdentifier) -> Self {
        Self {
            partition: Some(known::Partition::default().into()),
            service,
            region: None,
            account_id: None,
            resource,
        }
    }

    /// Return `true` if the identifier contains variables of the form
    /// `${name}`, else `false`.
    pub fn has_variables(&self) -> bool {
        self.resource.has_variables()
    }

    /// Replace any variables in the string with values from the context,
    /// returning a new value if the replacements result in a legal identifier
    /// string. The
    pub fn replace_variables<V>(&self, context: &HashMap<String, V>) -> Result<Self, Error>
    where
        V: Clone + Into<String>,
    {
        Ok(Self {
            resource: self.resource.replace_variables(context)?,
            ..self.clone()
        })
    }
}

// ------------------------------------------------------------------------------------------------
// External Doc Tests
// ------------------------------------------------------------------------------------------------

#[cfg(doctest)]
doc_comment::doctest!("../README.md");

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

#[cfg(feature = "builders")]
pub mod builder;

#[cfg(feature = "known")]
pub mod known;

#[doc(hidden)]
mod error;
pub use error::Error;
