/*!
Provides a set of simple helper functions to make ResourceNames for the Cognito Identity service.

These resource definitions ae take from the AWS
[documentation](https://docs.aws.amazon.com/IAM/latest/UserGuide/list_amazoncognitoidentity.html#amazoncognitoidentity-resources-for-iam-policies).
*/

use crate::builder::ArnBuilder;
use crate::known::Service::CognitoIdentity;
use crate::{AccountIdentifier, Identifier, IdentifierLike, ResourceIdentifier, ResourceName};

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

///
/// `arn:${Partition}:cognito-identity:${Region}:${Account}:identitypool/${IdentityPoolId}`
///
pub fn identity_pool(
    partition: Identifier,
    region: Identifier,
    account: AccountIdentifier,
    identity_pool_id: Identifier,
) -> ResourceName {
    ArnBuilder::service_id(CognitoIdentity.into())
        .in_partition_id(partition)
        .in_region_id(region)
        .owned_by(account)
        .is(ResourceIdentifier::from_id_path(&[
            Identifier::new_unchecked("identitypool"),
            identity_pool_id,
        ]))
        .into()
}
