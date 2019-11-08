/*!
Provides types, builders, and other helpers to manipulate AWS Amazon Resource Name (ARN) strings.

For more, see the AWS documentation for [Amazon Resource Name
(ARN)](https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html).
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

use regex::Regex;
use std::fmt::{Debug, Display, Error, Formatter};
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// Contains the resource part of the ARN. There **mus** be a `resource-id`, there **may* be
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
    /// Missing the account id component.
    MissingAccountId,
    /// The partition account id provided is not valid.
    InvalidAccountId,
    /// Missing the resource component.
    MissingResource,
    /// The partition resource provided is not valid.
    InvalidResource,
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
    static ref SERVICE: Regex = Regex::new(r"^[a-zA-Z][a-zA-Z0-9\-]+$").unwrap();
}

impl ARN {
    ///
    /// Validate this ARN, if provided the `validators` struct will be used to also
    /// provide any service-specific validation.
    ///
    pub fn validate(&self) -> Result<(), ArnError> {
        if let Some(partition) = &self.partition {
            if !PARTITION.is_match(&partition) {
                return Err(ArnError::InvalidPartition);
            }
        }
        if !SERVICE.is_match(&self.service) {
            return Err(ArnError::InvalidService);
        }

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
            Ok(ARN {
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
            })
        }
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
                Err(ArnError::InvalidResource)
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

mod validate;

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resource_to_string() {
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
    fn test_resource_from_str() {
        assert_eq!(Resource::from_str(WILD), Ok(Resource::Any));
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
    fn test_arn_to_string() {
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
    fn test_arn_from_str() {
        let arn_str = "arn:aws:s3:us-east-1:123456789012:job/23476";
        let arn: ARN = arn_str.parse().unwrap();
        assert_eq!(arn.partition, Some("aws".to_string()));
        assert_eq!(arn.service, "s3".to_string());
        assert_eq!(arn.region, Some("us-east-1".to_string()));
        assert_eq!(arn.account_id, Some("123456789012".to_string()));
    }
}
