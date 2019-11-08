/*!
Provides a more natural builder interface for constructing ARNs.

# Example

The following shows the construction of an AWS versioned layer ARN.

```rust
use aws_arn::builder::*;

let arn = ArnBuilder::new("lambda")
    .resource(
        ResourceBuilder::new("my-layer")
            .is_a("layer")
            .with_version(3)
            .build(),
    )
    .in_region("us-east-2")
    .owned_by("123456789012")
    .build();
println!("ARN: '{}'", arn);
```

This should print `ARN: 'arn:aws:lambda:us-east-2:123456789012:layer:my-layer:3'`.
*/

use crate::{Resource, ARN, WILD};

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

impl ResourceBuilder {
    /// Construct a resource with the specified `id`.
    pub fn new(id: &str) -> Self {
        ResourceBuilder {
            resource: Resource::Id(id.to_string()),
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
    pub fn build(&self) -> Resource {
        self.resource.clone()
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

    /// Set a specific `region` for this ARN.
    pub fn in_region(&mut self, region: &str) -> &mut Self {
        self.arn.region = Some(region.to_string());
        self
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

    /// Construct an `ARN` from this `ArnBuilder`.
    pub fn build(&self) -> ARN {
        self.arn.clone()
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
            .resource(ResourceBuilder::new("my-bucket").build())
            .build();
        assert_eq!(arn.to_string(), "arn:aws:s3:::my-bucket");
    }

    #[test]
    fn test_lambda_layer() {
        let arn = ArnBuilder::new("lambda")
            .resource(
                ResourceBuilder::new("my-layer")
                    .is_a("layer")
                    .with_version(3)
                    .build(),
            )
            .in_region("us-east-2")
            .owned_by("123456789012")
            .build();
        assert_eq!(
            arn.to_string(),
            "arn:aws:lambda:us-east-2:123456789012:layer:my-layer:3"
        );
    }
}
