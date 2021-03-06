/*!
Provides a set of simple helper functions to make ARNs for the Cognito Identity service.

These resource definitions ae take from the AWS
[documentation](https://docs.aws.amazon.com/IAM/latest/UserGuide/list_amazoncognitoidentity.html#amazoncognitoidentity-resources-for-iam-policies).
*/

use crate::builder::{ArnBuilder, ResourceBuilder};
use crate::{ArnError, ARN};

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

///
/// The service name portion of the ARN.
///
pub const SERVICE_NAME: &str = "cognito-identity";

///
/// `arn:${Partition}:cognito-identity:${Region}:${Account}:identitypool/${IdentityPoolId}`
///
pub fn identity_pool(
    partition: &str,
    region: &str,
    account: &str,
    identity_pool_id: &str,
) -> Result<ARN, ArnError> {
    ArnBuilder::new(SERVICE_NAME)
        .in_partition(partition)
        .in_region(region)
        .owned_by(account)
        .is(ResourceBuilder::new(identity_pool_id)
            .is_an("identitypool")
            .build()?)
        .build()
}
