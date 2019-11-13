/*!
Provides a more natural builder interface for constructing ARNs.

The builder pattern allows for a more readable construction of ARNs, and in this case we
provide a number of *verb* prefixes on *noun* constructors, so we have `in_region` as well as
`and_region` which is more readable if it is preceded by `in_partition`. For the account id
field there is `in_account`, `and_account`, `any_account`, and `owned_by`; all of these
accomplish the same goal but allow for a choice that makes code easir to understand.

# Resource-Specific Constructor Functions

For the service-specific submodules (`iam`, `lambda`, `s3`, etc.) the functions are simply named
for the noun that represents the resource type as described in the AWS documentation. As the
partition in commonly left to default to "aws" there are also a set of `{noun}_in()` functions
that take a partition, and corresponding `{noun}()` functions which do not.

In some cases where an ARN may be dependent on another, for example an S3 object ARN might be
constructed from an existing bucket ARN, additional `{noun}_from(other,...)` functions will
be provided.

# Example

The following shows the construction of an AWS versioned layer ARN.

```rust
use aws_arn::builder::*;

let arn = ArnBuilder::new("lambda")
    .resource(
        ResourceBuilder::new("my-layer")
            .is_a("layer")
            .with_version(3)
            .build().unwrap(),
    )
    .in_region("us-east-2")
    .owned_by("123456789012")
    .build().expect("badly formatted ARN?");
println!("ARN: '{}'", arn);
```

This should print `ARN: 'arn:aws:lambda:us-east-2:123456789012:layer:my-layer:3'`.
*/

use crate::{ArnError, Resource, ARN, WILD};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// Builder type for the resource portion of an ARN.
///
#[derive(Debug)]
pub struct ResourceBuilder {
    resource: Resource,
}

///
/// Builder type for an AWS ARN.
///
#[derive(Debug)]
pub struct ArnBuilder {
    arn: ARN,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl ResourceBuilder {
    /// Construct a resource with the specified `id` or `path`. If the string contains a '/'
    /// character a Path is created, else an Id.
    pub fn new(id_or_path: &str) -> Self {
        if id_or_path.contains(':') {
            panic!("You can't create qualified things");
        } else if id_or_path.contains('/') {
            ResourceBuilder {
                resource: Resource::Path(id_or_path.to_string()),
            }
        } else {
            ResourceBuilder {
                resource: Resource::Id(id_or_path.to_string()),
            }
        }
    }

    /// Construct a resource with a wildcard `id`.
    pub fn any() -> Self {
        Self::new(WILD)
    }

    /// Add the specific `type` to this resource (path-like style).
    #[allow(clippy::wrong_self_convention)]
    pub fn is_a(&mut self, the_type: &str) -> &mut Self {
        let new_type = the_type.to_string();
        match &self.resource {
            Resource::Any => {
                self.resource = Resource::TypedId {
                    id: WILD.to_string(),
                    the_type: new_type,
                }
            }
            Resource::Id(id) => {
                self.resource = Resource::TypedId {
                    id: id.clone(),
                    the_type: new_type,
                }
            }
            Resource::Path(path) => {
                self.resource = Resource::TypedId {
                    id: path.clone(),
                    the_type: new_type,
                }
            }
            Resource::TypedId { the_type: _, id } => {
                self.resource = Resource::TypedId {
                    id: id.clone(),
                    the_type: new_type,
                }
            }
            Resource::QTypedId {
                the_type: _,
                id,
                qualifier,
            } => {
                self.resource = Resource::QTypedId {
                    id: id.clone(),
                    the_type: new_type,
                    qualifier: qualifier.clone(),
                }
            }
        };
        self
    }

    /// Add the specific `type` to this resource (path-like style).
    #[allow(clippy::wrong_self_convention)]
    pub fn is_an(&mut self, the_type: &str) -> &mut Self {
        self.is_a(the_type)
    }

    /// Add the specific `type` to this resource (path-like style).
    pub fn has_type(&mut self, the_type: &str) -> &mut Self {
        self.is_a(the_type)
    }

    /// Add a `qualifier` to this resource
    pub fn with(&mut self, qualifier: &str) -> &mut Self {
        let new_qualifier = qualifier.to_string();
        match &self.resource {
            Resource::Any => {
                self.resource = Resource::QTypedId {
                    id: WILD.to_string(),
                    the_type: WILD.to_string(),
                    qualifier: new_qualifier,
                }
            }
            Resource::Id(id) => {
                self.resource = Resource::QTypedId {
                    id: id.clone(),
                    the_type: WILD.to_string(),
                    qualifier: new_qualifier,
                }
            }
            Resource::Path(path) => {
                self.resource = Resource::QTypedId {
                    id: path.clone(),
                    the_type: WILD.to_string(),
                    qualifier: new_qualifier,
                }
            }
            Resource::TypedId { the_type, id } => {
                self.resource = Resource::QTypedId {
                    id: id.clone(),
                    the_type: the_type.to_string(),
                    qualifier: new_qualifier,
                }
            }
            Resource::QTypedId {
                the_type,
                id,
                qualifier: _,
            } => {
                self.resource = Resource::QTypedId {
                    id: id.clone(),
                    the_type: the_type.to_string(),
                    qualifier: new_qualifier,
                }
            }
        };
        self
    }

    /// Add a version number, as a `qualifier`, to this resource
    pub fn with_version(&mut self, version: i32) -> &mut Self {
        self.with(version.to_string().as_str());
        self
    }

    /// Construct a `Resource` from this `ResourceBuilder`.
    pub fn build(&self) -> Result<Resource, ArnError> {
        let new_resource = self.resource.clone();
        new_resource.validate().map(|_| new_resource)
    }
}

impl ArnBuilder {
    /// Construct an ARN for the specified `service`.
    pub fn new(service: &str) -> Self {
        ArnBuilder {
            arn: ARN {
                partition: None,
                service: service.to_string(),
                region: None,
                account_id: None,
                resource: Resource::Id(String::new()),
            },
        }
    }

    /// Set a specific `partition` for this ARN.
    pub fn in_partition(&mut self, partition: &str) -> &mut Self {
        self.arn.partition = Some(partition.to_string());
        self
    }

    /// Set a specific `partition` for this ARN.
    pub fn in_any_partition(&mut self) -> &mut Self {
        self.arn.partition = None;
        self
    }

    /// Set a specific `region` for this ARN.
    pub fn in_region(&mut self, region: &str) -> &mut Self {
        self.arn.region = Some(region.to_string());
        self
    }

    /// Set a specific `region` for this ARN.
    pub fn and_region(&mut self, region: &str) -> &mut Self {
        self.in_region(region)
    }

    /// Set `region` to a wildcard for this ARN.
    pub fn in_any_region(&mut self) -> &mut Self {
        self.in_region(WILD)
    }

    /// Set a specific `account` for this ARN.
    pub fn in_account(&mut self, account: &str) -> &mut Self {
        self.arn.account_id = Some(account.to_string());
        self
    }

    /// Set a specific `account` for this ARN.
    pub fn and_account(&mut self, account: &str) -> &mut Self {
        self.in_account(account)
    }

    /// Set a specific `account` for this ARN.
    pub fn owned_by(&mut self, account: &str) -> &mut Self {
        self.in_account(account)
    }

    /// Set `account` to a wildcard for this ARN.
    pub fn in_any_account(&mut self) -> &mut Self {
        self.in_account(WILD)
    }

    /// Set a specific `resource` for this ARN.
    pub fn resource(&mut self, resource: Resource) -> &mut Self {
        self.arn.resource = resource;
        self
    }

    /// Set a specific `resource` for this ARN.
    pub fn is(&mut self, resource: Resource) -> &mut Self {
        self.resource(resource)
    }

    /// Set a specific `resource` for this ARN.
    pub fn a(&mut self, resource: Resource) -> &mut Self {
        self.resource(resource)
    }

    /// Set `resource` to a wildcard for this ARN.
    pub fn any_resource(&mut self) -> &mut Self {
        self.arn.resource = Resource::Any;
        self
    }

    /// Set `resource` to a wildcard for this ARN.
    pub fn for_any_resource(&mut self) -> &mut Self {
        self.any_resource()
    }

    /// Construct an `ARN` from this `ArnBuilder`.
    pub fn build(&self) -> Result<ARN, ArnError> {
        let new_arn = self.arn.clone();
        new_arn.validate().map(|_| new_arn)
    }
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

pub mod cognito;

pub mod iam;

pub mod lambda;

pub mod s3;

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::{ArnBuilder, ResourceBuilder};

    #[test]
    fn test_s3_bucket() {
        let arn = ArnBuilder::new("s3")
            .resource(ResourceBuilder::new("my-bucket").build().unwrap())
            .build()
            .unwrap();
        assert_eq!(arn.to_string(), "arn:aws:s3:::my-bucket");
    }

    #[test]
    fn test_lambda_layer() {
        let arn = ArnBuilder::new("lambda")
            .resource(
                ResourceBuilder::new("my-layer")
                    .is_a("layer")
                    .with_version(3)
                    .build()
                    .unwrap(),
            )
            .in_region("us-east-2")
            .owned_by("123456789012")
            .build()
            .unwrap();
        assert_eq!(
            arn.to_string(),
            "arn:aws:lambda:us-east-2:123456789012:layer:my-layer:3"
        );
    }
}
