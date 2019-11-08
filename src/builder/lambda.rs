/*!
Provides a set of simple helper functions to make ARNs for the Lambda service.
*/

use crate::builder::{ArnBuilder, ResourceBuilder};
use crate::ARN;

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

///
/// From [doc](https://docs.aws.amazon.com/IAM/latest/UserGuide/list_awslambda.html#awslambda-resources-for-iam-policies)
///
/// `arn:${Partition}:lambda:${Region}:${Account}:function:${FunctionName}`
///
pub fn function(partition: &str, region: &str, account: &str, function_name: &str) -> ARN {
    ArnBuilder::new("lambda")
        .in_partition(partition)
        .in_region(region)
        .owned_by(account)
        .is(ResourceBuilder::new(function_name).is_a("function").build())
        .build()
}

///
/// From [doc](https://docs.aws.amazon.com/IAM/latest/UserGuide/list_awslambda.html#awslambda-resources-for-iam-policies)
///
/// `arn:${Partition}:lambda:${Region}:${Account}:layer:${LayerName}`
///
pub fn layer(partition: &str, region: &str, account: &str, layer_name: &str) -> ARN {
    ArnBuilder::new("lambda")
        .in_partition(partition)
        .in_region(region)
        .owned_by(account)
        .is(ResourceBuilder::new(layer_name).is_a("layer").build())
        .build()
}

///
/// From [doc](https://docs.aws.amazon.com/IAM/latest/UserGuide/list_awslambda.html#awslambda-resources-for-iam-policies)
///
/// `arn:${Partition}:lambda:${Region}:${Account}:layer:${LayerName}:${LayerVersion}`
///
pub fn layer_version(
    partition: &str,
    region: &str,
    account: &str,
    layer_name: &str,
    layer_version: i32,
) -> ARN {
    ArnBuilder::new("lambda")
        .in_partition(partition)
        .in_region(region)
        .owned_by(account)
        .is(ResourceBuilder::new(layer_name)
            .is_a("layer")
            .with_version(layer_version)
            .build())
        .build()
}

///
/// From [doc](https://docs.aws.amazon.com/IAM/latest/UserGuide/list_awslambda.html#awslambda-resources-for-iam-policies)
///
/// `arn:${Partition}:lambda:${Region}:${Account}:event-source-mapping:${UUID}`
///
pub fn event_source_mapping(
    partition: &str,
    region: &str,
    account: &str,
    mapping_uuid: &str,
) -> ARN {
    ArnBuilder::new("lambda")
        .in_partition(partition)
        .in_region(region)
        .owned_by(account)
        .is(ResourceBuilder::new(mapping_uuid)
            .is_an("event-source-mapping")
            .build())
        .build()
}
