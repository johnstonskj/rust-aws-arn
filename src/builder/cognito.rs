/*!
Provides a set of simple helper functions to make ARNs for the Cognito Identity service.
*/

use crate::builder::{ArnBuilder, ResourceBuilder};
use crate::ARN;

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

///
/// From [doc](https://docs.aws.amazon.com/IAM/latest/UserGuide/list_amazoncognitoidentity.html#amazoncognitoidentity-resources-for-iam-policies)
///
/// `arn:${Partition}:cognito-identity:${Region}:${Account}:identitypool/${IdentityPoolId}`
///
pub fn identity_pool(partition: &str, region: &str, account: &str, identity_pool_id: &str) -> ARN {
    ArnBuilder::new("cognito-identity")
        .in_partition(partition)
        .in_region(region)
        .owned_by(account)
        .is(ResourceBuilder::new(identity_pool_id)
            .is_an("identitypool")
            .build())
        .build()
}
