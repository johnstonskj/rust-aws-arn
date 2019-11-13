/*!
Provides a set of simple helper functions to make ARNs for the Lambda service.

These resource definitions ae take from the AWS
[documentation](https://docs.aws.amazon.com/IAM/latest/UserGuide/list_awslambda.html#awslambda-resources-for-iam-policies).
*/

use crate::builder::{ArnBuilder, ResourceBuilder};
use crate::{ArnError, ARN};

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

///
/// The service name portion of the ARN.
///
pub const SERVICE_NAME: &str = "lambda";

///
/// `arn:${Partition}:lambda:${Region}:${Account}:function:${FunctionName}`
///
pub fn function(
    partition: &str,
    region: &str,
    account: &str,
    function_name: &str,
) -> Result<ARN, ArnError> {
    ArnBuilder::new(SERVICE_NAME)
        .in_partition(partition)
        .in_region(region)
        .owned_by(account)
        .is(ResourceBuilder::new(function_name)
            .is_a("function")
            .build()?)
        .build()
}

///
/// `arn:${Partition}:lambda:${Region}:${Account}:layer:${LayerName}`
///
pub fn layer(
    partition: &str,
    region: &str,
    account: &str,
    layer_name: &str,
) -> Result<ARN, ArnError> {
    ArnBuilder::new(SERVICE_NAME)
        .in_partition(partition)
        .in_region(region)
        .owned_by(account)
        .is(ResourceBuilder::new(layer_name).is_a("layer").build()?)
        .build()
}

///
/// `arn:${Partition}:lambda:${Region}:${Account}:layer:${LayerName}:${LayerVersion}`
///
pub fn layer_version(
    partition: &str,
    region: &str,
    account: &str,
    layer_name: &str,
    layer_version: i32,
) -> Result<ARN, ArnError> {
    ArnBuilder::new(SERVICE_NAME)
        .in_partition(partition)
        .in_region(region)
        .owned_by(account)
        .is(ResourceBuilder::new(layer_name)
            .is_a("layer")
            .with_version(layer_version)
            .build()?)
        .build()
}

///
/// `arn:${Partition}:lambda:${Region}:${Account}:event-source-mapping:${UUID}`
///
pub fn event_source_mapping(
    partition: &str,
    region: &str,
    account: &str,
    mapping_uuid: &str,
) -> Result<ARN, ArnError> {
    ArnBuilder::new(SERVICE_NAME)
        .in_partition(partition)
        .in_region(region)
        .owned_by(account)
        .is(ResourceBuilder::new(mapping_uuid)
            .is_an("event-source-mapping")
            .build()?)
        .build()
}
