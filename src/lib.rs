/*!
Provides types, builders, and other helpers to manipulate AWS Amazon Resource Name (ARN) strings.

The ARN is a key component of all AWS service APIs and yet nearly all client toolkits treat it
simply as a string. While this may be a reasonable and expedient decision, it seems there might
be a need to not only ensure correctness of ARNs with validators but also constructors that allow
making these strings correclt in the first place.

# ARN Types

This crate provides three levels of ARN manipulation, the first is the direct construction of an
ARN type (module `aws_arn` - the core `Resource` and `ARN` types).

```rust
use aws_arn::{ARN, Resource};

let arn = ARN {
    partition: Some("aws".to_string()),
    service: "s3".to_string(),
    region: None,
    account_id: None,
    resource: Resource::Path("".to_string())};
```

One issue with the code above is that, unless you subsequently call `arn.validate()` the
resulting ARN could be garbage. Alternatively, using `FromStr,` you can parse a string into an
ARN which will call `validate()` for you.

```rust
use aws_arn::ARN;
use std::str::FromStr;

let arn: ARN = "arn:aws:s3:::mythings/thing-1".parse().expect("didn't look like an ARN");
```

The next is to use a more readable builder which also allows you to ignore those fields in the ARN
you don't always need (module `aws_arn::builder` - the `ResourceBuilder` and `ArnBuilder` types
providing a more fluent style of ARN construction).

```rust
use aws_arn::builder::{ArnBuilder, ResourceBuilder};

let arn = ArnBuilder::new("s3")
    .resource(ResourceBuilder::new(&format!("{}/{}", "mythings", "thing-1")).build().unwrap())
    .in_partition("aws")
    .build().unwrap();
```

Finally, it is possible to use resource-type specific functions that allow an even more direct and
simple construction (module `aws_arn::builder::{service}` - *service builder functions*.

```rust
use aws_arn::builder::s3;

let arn = s3::object("mythings", "thing-1");
```

For more, see the AWS documentation for [Amazon Resource Name
(ARN)](https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html). In a lot
of cases the documentation for elements of the ARN and Resource types use descriptions taken
directly from the AWS documentation.

# Validation

As mentioned above, both the `ARN` and `Resource` types have a `validate()` function that will
test for consistency. Also, this validation is optional if you construction these types
directly. The validation supported by these two resources is limited to syntactic values,
described in the following table. This table mentions *identifier* as a type, this is a pattern
where a string starts with an alphabetic character, then zero or more alphanumeric characters
and the characters '-' and '_'.

| Type       | Field         | Tests |
|------------|---------------|-------|
| `ARN`      | `partition`   | Either "aws" or "aws-*{identifier}*".  |
| `ARN`      | `service`     | Must be an *identifier*.  |
| `ARN`      | `region`      | Must be an *identifier*.  |
| `ARN`      | `account_id`  | A 12 character, ASCII digit, String.  |
| `ARN`      | `resource`    | Below.  |
| `Resource` | `id`          | Must not contain ':', '/', or '*'  |
| `Resource` | `path`        | Must not contain ':'  |
| `Resource` | `the_type`    | Must not contain ':', '/'  |
| `Resource` | `qualifier`   | Must not contain ':', '/'  |


# Optional Features

This crate has attempted to be as lean as possible, with a really minimal set of dependencies,
we have include the following capabilities as optional features.

* `serde_support` adds derived `Serialize` and `Deserialize` implementations for the `ARN` and
   `Resource` types. This feature is enabled by default.
* `ext_validation` adds extended, service specific, validation using an external configuration
  file. This feature is *not* enabled by default.

## Extended Validation

The feature `ext_validation` extends the capability of the `ARN::validate()` by applying a set of
rules for service and resource type pairs, for example S3 buckets or lambda functions. The rules
are defined in a configuration file, `service-formats.toml` in the crate and which is read and
the rules applied if a matching configuration is found. The file is structured as a list of
`[[format]]` maps and the following table summarizes the fields in this map.

| Name                | Type         | Required | Comments |
|---------------------|--------------|----------|----------|
| `name`              | *identifier* | **Yes**  | Service name, e.g. "s3" |
`resource_type`       | *identifier* | No       | Resource type, optional |
`partition_required`  | boolean      | **Yes**  | Must you specify a partition |
`region_required`     | boolean      | **Yes**  | Must you specify a region |
`region_wc_allowed`   | boolean      | No       | Wildcard, '*' character is allowed in a region string, default is false |
`account_id_required` | boolean      | **Yes**  | Must you specify an account ID |
`account_wc_allowed`  | boolean      | No       | Wildcard, '*' character is allowed in a region string, default is false |
`resource_format`     | enum         | **Yes**  | Defines the required format for the resource portion |
`resource_wc_allowed` | boolean      | No       | Wildcard, '*' character is allowed in a region string, default is false |

*/

// ------------------------------------------------------------------------------------------------
// Preamble
// ------------------------------------------------------------------------------------------------

#![warn(
    missing_debug_implementations,
    missing_docs,
    unused_extern_crates,
    rust_2018_idioms
)]

#[macro_use]
extern crate lazy_static;

#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize};

use regex::Regex;
use std::fmt::{Debug, Display, Error, Formatter};
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// Contains the resource part of the ARN. There **must** be a `resource-id`, there **may** be
/// a `resource-type`, and there **may** be a qualifier. The separator between type and id
/// may be prefix-like (':') or path-like (PATH_SEPARATOR).
///
/// > The content of this part of the ARN varies by service. A resource identifier can be the name
/// > or ID of the resource (for example, user/Bob or instance/i-1234567890abcdef0) or a
/// > resource path. For example, some resource identifiers include a parent resource
/// > (sub-resource-type/parent-resource/sub-resource) or a qualifier such as a version
/// > (resource-type:resource-name:qualifier).
///
/// > Some resource ARNs can include a path. For example, in Amazon S3, the resource identifier
/// > is an object name that can include slashes (/) to form a path. Similarly, IAM user names
/// > and group names can include paths.
///
/// > In some circumstances, paths can include a wildcard character, namely an asterisk (*).
///
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde_support", derive(Deserialize, Serialize))]
pub enum Resource {
    /// The wildcard resource.
    Any,
    /// Matches `resource-id`
    Id(String),
    /// Matches `resource-id(/resource-id)*`
    Path(String),
    /// Matches `resource-type:resource-id`
    TypedId {
        /// The `resource-type` component of this resource
        the_type: String,
        /// The `resource-id` component of this resource
        id: String,
    },
    /// Matches `resource-type:resource-id:qualifier`
    QTypedId {
        /// The `resource-type` component of this resource
        the_type: String,
        /// The `resource-id` component of this resource
        id: String,
        /// The `qualifier` component of this resource
        qualifier: String,
    },
}

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
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde_support", derive(Deserialize, Serialize))]
pub struct ARN {
    /// The partition that the resource is in. For standard AWS Regions, the partition is` aws`.
    /// If you have resources in other partitions, the partition is `aws-partitionname`. For
    /// example, the partition for resources in the China (Beijing) Region is `aws-cn`.
    pub partition: Option<String>,
    /// The service namespace that identifies the AWS product (for example, Amazon S3, IAM,
    /// or Amazon RDS).
    pub service: String,
    /// The Region that the resource resides in. The ARNs for some resources do not require
    /// a Region, so this component might be omitted.
    pub region: Option<String>,
    /// The ID of the AWS account that owns the resource, without the hyphens. For example,
    /// `123456789012`. The ARNs for some resources don't require an account number, so this
    /// component might be omitted.
    pub account_id: Option<String>,
    /// The content of this part of the ARN varies by service. A resource identifier can
    /// be the name or ID of the resource (for example, `user/Bob` or
    /// `instance/i-1234567890abcdef0`) or a resource path. For example, some resource
    /// identifiers include a parent resource
    /// (`sub-resource-type/parent-resource/sub-resource`) or a qualifier such as a
    /// version (`resource-type:resource-name:qualifier`).
    pub resource: Resource,
}

///
/// Errors that may arise parsing an ARN with `FromStr::from_str()`.
///
#[derive(Debug, PartialEq)]
pub enum ArnError {
    /// Need at least 6 components.
    TooFewComponents,
    /// Missing the 'arn' prefix string.
    MissingPrefix,
    /// Missing the partition component.
    MissingPartition,
    /// The partition component provided is not valid.
    InvalidPartition,
    /// Missing the service component.
    MissingService,
    /// The service component provided is not valid.
    InvalidService,
    /// Missing the region component.
    MissingRegion,
    /// The partition region provided is not valid.
    InvalidRegion,
    /// The particular resource type does not allow region wildcards.
    RegionWildcardNotAllowed,
    /// Missing the account id component.
    MissingAccountId,
    /// The partition account id provided is not valid.
    InvalidAccountId,
    /// The particular resource type does not allow account wildcards.
    AccountIdWildcardNotAllowed,
    /// Missing the resource component.
    MissingResource,
    /// The partition resource provided is not valid, the name of the particular component
    /// in error is included.
    InvalidResource(String),
    /// The particular resource type does not allow resource wildcards.
    ResourceWildcardNotAllowed,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

const ARN_PREFIX: &str = "arn";

const ARN_SEPARATOR: char = ':';

const ARN_SEPARATOR_STR: &str = ":";

const DEFAULT_PARTITION: &str = "aws";

const PATH_SEPARATOR: char = '/';

const WILD: &str = "*";

lazy_static! {
    static ref PARTITION: Regex = Regex::new(r"^aws(\-[a-zA-Z][a-zA-Z0-9\-]+)?$").unwrap();
    static ref IDENTIFIER: Regex = Regex::new(r"^[a-zA-Z][a-zA-Z0-9\-]+$").unwrap();
}

impl ARN {
    ///
    /// Validate this ARN, if the `ext_validation` feature is enabled it will be used to
    /// provide any service-specific validation.
    ///
    pub fn validate(&self) -> Result<(), ArnError> {
        if let Some(partition) = &self.partition {
            if !PARTITION.is_match(&partition) {
                return Err(ArnError::InvalidPartition);
            }
        }

        if !IDENTIFIER.is_match(&self.service) {
            return Err(ArnError::InvalidService);
        }

        if let Some(region) = &self.region {
            if !IDENTIFIER.is_match(region) {
                return Err(ArnError::InvalidRegion);
            }
        }

        if let Some(account_id) = &self.account_id {
            if account_id.len() != 12 || !account_id.chars().all(|c| c.is_ascii_digit()) {
                return Err(ArnError::InvalidAccountId);
            }
        }

        self.resource.validate()?;

        if validate::is_registered(&self.service, &self.resource) {
            validate::validate(self)?
        }
        Ok(())
    }
}

impl Display for ARN {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(
            f,
            "{}",
            vec![
                ARN_PREFIX.to_string(),
                self.partition
                    .as_ref()
                    .unwrap_or(&DEFAULT_PARTITION.to_string())
                    .clone(),
                self.service.clone(),
                self.region.as_ref().unwrap_or(&String::new()).clone(),
                self.account_id.as_ref().unwrap_or(&String::new()).clone(),
                self.resource.clone().to_string()
            ]
            .join(ARN_SEPARATOR_STR)
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
        let mut parts: Vec<&str> = s.split(ARN_SEPARATOR).collect();
        if parts.len() < 6 {
            Err(ArnError::TooFewComponents)
        } else if parts[0] != ARN_PREFIX {
            Err(ArnError::MissingPrefix)
        } else {
            let new_arn = ARN {
                partition: if parts[1].is_empty() {
                    None
                } else {
                    Some(parts[1].to_string())
                },
                service: parts[2].to_string(),
                region: if parts[3].is_empty() {
                    None
                } else {
                    Some(parts[3].to_string())
                },
                account_id: if parts[4].is_empty() {
                    None
                } else {
                    Some(parts[4].to_string())
                },
                resource: {
                    let resource_parts: Vec<&str> = parts.drain(5..).collect();
                    Resource::from_str(&resource_parts.join(ARN_SEPARATOR_STR))?
                },
            };
            new_arn.validate().map(|_| new_arn)
        }
    }
}

impl Resource {
    ///
    /// Validate the syntax of a resource, not any service-specific semantics.
    ///
    pub fn validate(&self) -> Result<(), ArnError> {
        match self {
            Resource::Id(id) => must_not_contain(id, "id", &[':', '/', '*']),
            Resource::Path(path) => must_not_contain(path, "path", &[':']),
            Resource::TypedId { the_type, id } => {
                must_not_contain(the_type, "the_type", &[':', '/', '*'])
                    .and_then(|_| must_not_contain(id, "id", &[':', '/']))
            }
            Resource::QTypedId {
                the_type,
                id,
                qualifier,
            } => must_not_contain(the_type, "the_type", &[':', '/', '*']).and_then(|_| {
                must_not_contain(id, "id", &[':', '/'])
                    .and_then(|_| must_not_contain(qualifier, "qualifier", &[':', '/', '*']))
            }),
            _ => Ok(()),
        }
    }
}

fn must_not_contain(s: &str, c: &str, chars: &[char]) -> Result<(), ArnError> {
    if s.contains(chars) {
        Err(ArnError::InvalidResource(c.to_string()))
    } else {
        Ok(())
    }
}

impl Display for Resource {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            Resource::Any => write!(f, "{}", WILD),
            Resource::Id(id) => write!(f, "{}", id),
            Resource::Path(path) => write!(f, "{}", path),
            Resource::TypedId { the_type, id } => write!(f, "{}{}{}", the_type, ARN_SEPARATOR, id),
            Resource::QTypedId {
                the_type,
                id,
                qualifier,
            } => write!(
                f,
                "{}{}{}{}{}",
                the_type, ARN_SEPARATOR, id, ARN_SEPARATOR, qualifier
            ),
        }
    }
}

impl FromStr for Resource {
    type Err = ArnError;

    ///
    /// Technically, according to Formats    
    /// * `resource-id`
    /// * `resource-type/resource-id`
    /// * `resource-type:resource-id[:qualifier]`
    ///   
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            Err(ArnError::MissingResource)
        } else if s == WILD {
            Ok(Resource::Any)
        } else if s.contains(ARN_SEPARATOR) {
            let parts: Vec<&str> = s.split(ARN_SEPARATOR).collect();
            if parts.len() == 2 {
                Ok(Resource::TypedId {
                    the_type: parts[0].to_string(),
                    id: parts[1].to_string(),
                })
            } else if parts.len() == 3 {
                Ok(Resource::QTypedId {
                    the_type: parts[0].to_string(),
                    id: parts[1].to_string(),
                    qualifier: parts[2].to_string(),
                })
            } else {
                Err(ArnError::InvalidResource(s.to_string()))
            }
        } else if s.contains(PATH_SEPARATOR) {
            Ok(Resource::Path(s.to_string()))
        } else {
            Ok(Resource::Id(s.to_string()))
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

pub mod builder;

#[cfg(feature = "ext_validation")]
mod validate;

#[cfg(not(feature = "ext_validation"))]
mod validate {
    //
    // A stub for the module when the feature is not present.
    //
    use crate::{ArnError, Resource, ARN};

    pub fn is_registered(_service: &str, _resource: &Resource) -> bool {
        false
    }

    pub fn validate(_arn: &ARN) -> Result<(), ArnError> {
        Ok(())
    }
}

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_resource_to_string() {
        assert_eq!(Resource::Id("thing".to_string()).to_string(), "thing");
        assert_eq!(
            Resource::Path("mythings/athing".to_string()).to_string(),
            "mythings/athing"
        );
        assert_eq!(
            Resource::TypedId {
                the_type: "things".to_string(),
                id: "athing".to_string()
            }
            .to_string(),
            "things:athing"
        );
        assert_eq!(
            Resource::QTypedId {
                the_type: "things".to_string(),
                id: "athing".to_string(),
                qualifier: "v2".to_string()
            }
            .to_string(),
            "things:athing:v2"
        );
    }

    #[test]
    fn test_resource_from_valid_str() {
        assert_eq!(Resource::from_str(WILD), Ok(Resource::Any));
        assert_eq!(
            Resource::from_str("athing"),
            Ok(Resource::Id("athing".to_string()))
        );
        assert_eq!(
            Resource::from_str("mythings/athing"),
            Ok(Resource::Path("mythings/athing".to_string()))
        );
        assert_eq!(
            Resource::from_str("things:athing"),
            Ok(Resource::TypedId {
                the_type: "things".to_string(),
                id: "athing".to_string()
            })
        );
        assert_eq!(
            Resource::from_str("things:athing:v2"),
            Ok(Resource::QTypedId {
                the_type: "things".to_string(),
                id: "athing".to_string(),
                qualifier: "v2".to_string()
            })
        );
    }

    #[test]
    fn test_valid_arn_to_string() {
        let arn = ARN {
            partition: None,
            service: "s3".to_string(),
            region: None,
            account_id: None,
            resource: Resource::Path("mythings/athing".to_string()),
        };
        assert_eq!(arn.to_string(), "arn:aws:s3:::mythings/athing");
    }

    #[test]
    fn test_arn_from_valid_str() {
        let arn_str = "arn:aws:s3:us-east-1:123456789012:job/23476";
        let arn: ARN = arn_str.parse().unwrap();
        assert_eq!(arn.partition, Some("aws".to_string()));
        assert_eq!(arn.service, "s3".to_string());
        assert_eq!(arn.region, Some("us-east-1".to_string()));
        assert_eq!(arn.account_id, Some("123456789012".to_string()));
    }

    #[test]
    fn test_valid_id_resource() {
        let resource = Resource::Id("s3".to_string());
        assert_eq!(resource.validate(), Ok(()));
    }

    #[test]
    fn test_invalid_id_resource() {
        let resource = Resource::Id("s:3".to_string());
        assert_eq!(
            resource.validate(),
            Err(ArnError::InvalidResource("id".to_string()))
        );
        let resource = Resource::Id("s/3".to_string());
        assert_eq!(
            resource.validate(),
            Err(ArnError::InvalidResource("id".to_string()))
        );
        let resource = Resource::Id("s3*".to_string());
        assert_eq!(
            resource.validate(),
            Err(ArnError::InvalidResource("id".to_string()))
        );
    }

    #[test]
    fn test_valid_path_resource() {
        let resource = Resource::Path("user/org/simon".to_string());
        assert_eq!(resource.validate(), Ok(()));

        let resource = Resource::Path("user/org/*".to_string());
        assert_eq!(resource.validate(), Ok(()));
    }

    #[test]
    fn test_invalid_path_resource() {
        let resource = Resource::Path("user:simon".to_string());
        assert_eq!(
            resource.validate(),
            Err(ArnError::InvalidResource("path".to_string()))
        );
    }

    #[test]
    fn test_valid_typed_id_resource() {
        let resource = Resource::TypedId {
            the_type: "user".to_string(),
            id: "simon".to_string(),
        };
        assert_eq!(resource.validate(), Ok(()));

        let resource = Resource::TypedId {
            the_type: "user".to_string(),
            id: "*".to_string(),
        };
        assert_eq!(resource.validate(), Ok(()));
    }

    #[test]
    fn test_invalid_typed_id_resource() {
        let resource = Resource::TypedId {
            the_type: "us:er".to_string(),
            id: "simon".to_string(),
        };
        assert_eq!(
            resource.validate(),
            Err(ArnError::InvalidResource("the_type".to_string()))
        );
        let resource = Resource::TypedId {
            the_type: "us/er".to_string(),
            id: "simon".to_string(),
        };
        assert_eq!(
            resource.validate(),
            Err(ArnError::InvalidResource("the_type".to_string()))
        );
        let resource = Resource::TypedId {
            the_type: "us*er".to_string(),
            id: "simon".to_string(),
        };
        assert_eq!(
            resource.validate(),
            Err(ArnError::InvalidResource("the_type".to_string()))
        );

        let resource = Resource::TypedId {
            the_type: "user".to_string(),
            id: "sim:on".to_string(),
        };
        assert_eq!(
            resource.validate(),
            Err(ArnError::InvalidResource("id".to_string()))
        );
        let resource = Resource::TypedId {
            the_type: "user".to_string(),
            id: "sim/on".to_string(),
        };
        assert_eq!(
            resource.validate(),
            Err(ArnError::InvalidResource("id".to_string()))
        );
    }

    #[test]
    fn test_valid_qtyped_id_resource() {
        let resource = Resource::QTypedId {
            the_type: "user".to_string(),
            id: "simon".to_string(),
            qualifier: "v2".to_string(),
        };
        assert_eq!(resource.validate(), Ok(()));

        let resource = Resource::QTypedId {
            the_type: "user".to_string(),
            id: "*".to_string(),
            qualifier: "v2".to_string(),
        };
        assert_eq!(resource.validate(), Ok(()));
    }

    #[test]
    fn test_invalid_qtyped_id_resource() {
        let resource = Resource::QTypedId {
            the_type: "us:er".to_string(),
            id: "simon".to_string(),
            qualifier: "v2".to_string(),
        };
        assert_eq!(
            resource.validate(),
            Err(ArnError::InvalidResource("the_type".to_string()))
        );
        let resource = Resource::QTypedId {
            the_type: "us/er".to_string(),
            id: "simon".to_string(),
            qualifier: "v2".to_string(),
        };
        assert_eq!(
            resource.validate(),
            Err(ArnError::InvalidResource("the_type".to_string()))
        );
        let resource = Resource::QTypedId {
            the_type: "us*er".to_string(),
            id: "simon".to_string(),
            qualifier: "v2".to_string(),
        };
        assert_eq!(
            resource.validate(),
            Err(ArnError::InvalidResource("the_type".to_string()))
        );

        let resource = Resource::QTypedId {
            the_type: "user".to_string(),
            id: "sim:on".to_string(),
            qualifier: "v2".to_string(),
        };
        assert_eq!(
            resource.validate(),
            Err(ArnError::InvalidResource("id".to_string()))
        );
        let resource = Resource::QTypedId {
            the_type: "user".to_string(),
            id: "sim/on".to_string(),
            qualifier: "v2".to_string(),
        };
        assert_eq!(
            resource.validate(),
            Err(ArnError::InvalidResource("id".to_string()))
        );

        let resource = Resource::QTypedId {
            the_type: "user".to_string(),
            id: "simon".to_string(),
            qualifier: "v:2".to_string(),
        };
        assert_eq!(
            resource.validate(),
            Err(ArnError::InvalidResource("qualifier".to_string()))
        );
        let resource = Resource::QTypedId {
            the_type: "user".to_string(),
            id: "simon".to_string(),
            qualifier: "v/2".to_string(),
        };
        assert_eq!(
            resource.validate(),
            Err(ArnError::InvalidResource("qualifier".to_string()))
        );
        let resource = Resource::QTypedId {
            the_type: "user".to_string(),
            id: "simon".to_string(),
            qualifier: "v*2".to_string(),
        };
        assert_eq!(
            resource.validate(),
            Err(ArnError::InvalidResource("qualifier".to_string()))
        );
    }
}
