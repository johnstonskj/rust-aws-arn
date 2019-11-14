/*!
Implement rule-based, service-specific validation using an external configuration file.
*/

use crate::{ArnError, Resource, ARN};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn is_registered(service: &str, resource: &Resource) -> bool {
    FORMATS.contains_key(&make_key(service, resource))
}

pub fn validate(arn: &ARN) -> Result<(), ArnError> {
    match FORMATS.get(&make_key(&arn.service, &arn.resource)) {
        Some(format) => {
            println!("Format: {:?}", format);
            // ------------------------------------------------------------------------------------
            if format.partition_required && arn.partition == None {
                return Err(ArnError::MissingPartition);
            }
            // ------------------------------------------------------------------------------------
            match &arn.region {
                None => {
                    if format.region_required {
                        return Err(ArnError::MissingRegion);
                    }
                }
                Some(region) => {
                    if !format.region_wc_allowed && region.contains('*') {
                        return Err(ArnError::RegionWildcardNotAllowed);
                    }
                }
            }
            // ------------------------------------------------------------------------------------
            match &arn.account_id {
                None => {
                    if format.account_id_required {
                        return Err(ArnError::MissingAccountId);
                    }
                }
                Some(account_id) => {
                    if !format.account_wc_allowed && account_id.contains('*') {
                        return Err(ArnError::AccountIdWildcardNotAllowed);
                    }
                }
            }
            // ------------------------------------------------------------------------------------
            match &arn.resource {
                Resource::Any => {
                    if !format.resource_wc_allowed {
                        return Err(ArnError::ResourceWildcardNotAllowed);
                    }
                }
                Resource::Id(id) => {
                    if format.resource_format != ResourceFormat::Id {
                        return Err(ArnError::InvalidResource);
                    } else if !format.resource_wc_allowed && id.contains('*') {
                        return Err(ArnError::ResourceWildcardNotAllowed);
                    }
                }
                Resource::Path(path) => {
                    if format.resource_format != ResourceFormat::Path {
                        return Err(ArnError::InvalidResource);
                    } else if !format.resource_wc_allowed && path.contains('*') {
                        return Err(ArnError::ResourceWildcardNotAllowed);
                    }
                }
                Resource::TypedId { the_type, id } => {
                    if format.resource_format != ResourceFormat::TypeId {
                        return Err(ArnError::InvalidResource);
                    } else if the_type.contains('*')
                        || (!format.resource_wc_allowed && id.contains('*'))
                        || the_type.is_empty()
                        || id.is_empty()
                    {
                        return Err(ArnError::ResourceWildcardNotAllowed);
                    }
                }
                Resource::QTypedId {
                    the_type,
                    id,
                    qualifier,
                } => {
                    if format.resource_format != ResourceFormat::QTypeId {
                        return Err(ArnError::InvalidResource);
                    } else if the_type.contains('*')
                        || (!format.resource_wc_allowed
                            && (id.contains('*') || qualifier.contains('*')))
                        || the_type.is_empty()
                        || id.is_empty()
                        || qualifier.is_empty()
                    {
                        return Err(ArnError::ResourceWildcardNotAllowed);
                    }
                }
            }
            Ok(())
        }
        None => Ok(()),
    }
}

// ------------------------------------------------------------------------------------------------
// Implementation
// ------------------------------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
enum ResourceFormat {
    Id,
    Path,
    TypeId,
    QTypeId,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct ServiceArnFormat {
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_type: Option<String>,
    partition_required: bool,
    region_required: bool,
    #[serde(default)]
    region_wc_allowed: bool,
    account_id_required: bool,
    #[serde(default)]
    account_wc_allowed: bool,
    resource_format: ResourceFormat,
    #[serde(default)]
    resource_wc_allowed: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct ServiceArnFormats {
    format: Vec<ServiceArnFormat>,
}

lazy_static! {
    static ref FORMATS: HashMap<String, ServiceArnFormat> = load_formats();
}

fn load_formats() -> HashMap<String, ServiceArnFormat> {
    let raw_data = include_bytes!("service-formats.toml");
    let mut formats: ServiceArnFormats = toml::from_slice(raw_data).unwrap();
    formats
        .format
        .drain(0..)
        .map(|f| (make_key_str(&f.name, &f.resource_type), f))
        .collect::<HashMap<String, ServiceArnFormat>>()
}

fn make_key(s_name: &str, resource: &Resource) -> String {
    let resource_type = match resource {
        Resource::TypedId { the_type, id: _ } => {
            let new_type = the_type.to_string();
            Some(new_type)
        }
        Resource::QTypedId {
            the_type,
            id: _,
            qualifier: _,
        } => {
            let new_type = the_type.clone();
            Some(new_type)
        }
        _ => None,
    };
    make_key_str(s_name, &resource_type)
}

fn make_key_str(s_name: &str, r_type: &Option<String>) -> String {
    match r_type {
        Some(r_type) => format!("{}-{}", s_name, r_type),
        None => s_name.to_string(),
    }
}

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Resource;

    #[test]
    fn test_serializes() {
        // arn:aws:iam::123456789012:user/Development/product_1234/*
        let iam = ServiceArnFormat {
            name: "iam".to_string(),
            resource_type: Some("user".to_string()),
            partition_required: true,
            region_required: false,
            region_wc_allowed: false,
            account_id_required: true,
            account_wc_allowed: false,
            resource_format: ResourceFormat::Path,
            resource_wc_allowed: false,
        };
        let services = ServiceArnFormats { format: vec![iam] };
        let toml = toml::to_string(&services).unwrap();
        println!("{}", toml);
    }

    #[test]
    fn test_deserializes() {
        // arn:aws:iam::123456789012:user/Development/product_1234/*
        let iam = r#"[[format]]
name = "iam"
resource_type = "user"
partition_required = true
region_required = false
account_id_required = true
resource_format = "Path"
"#;
        let formats: ServiceArnFormats = toml::from_str(iam).unwrap();
        println!(
            "{}-{:?}",
            formats.format.get(0).unwrap().name,
            formats.format.get(0).unwrap().resource_type
        );
    }

    #[test]
    fn test_contains_iam() {
        assert!(is_registered(
            "iam",
            &Resource::TypedId {
                the_type: "user".to_string(),
                id: "id".to_string()
            }
        ));
        assert!(!is_registered(
            "iam",
            &Resource::TypedId {
                the_type: "foo".to_string(),
                id: "id".to_string()
            }
        ));
        assert!(!is_registered(
            "foo",
            &Resource::TypedId {
                the_type: "user".to_string(),
                id: "id".to_string()
            }
        ));
    }
}
