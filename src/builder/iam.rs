/*!
Provides a set of simple helper functions to make ARNs for the IAM service.

These resource definitions ae take from the AWS
[documentation](https://docs.aws.amazon.com/IAM/latest/UserGuide/list_identityandaccessmanagement.html#identityandaccessmanagement-resources-for-iam-policies).
With the exception  of the root account ARN described
[here](https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_identifiers.html#identifiers-arns).
[*/

use crate::builder::ArnBuilder;
use crate::known::Service::IdentityAccessManagement;
use crate::{AccountIdentifier, Identifier, ResourceIdentifier, ARN};

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

///
/// `arn:aws:iam::123456789012:root`
///
pub fn root(account: AccountIdentifier) -> ARN {
    ArnBuilder::service_id(IdentityAccessManagement.into())
        .owned_by(account)
        .is(ResourceIdentifier::new_unchecked("root"))
        .into()
}

///
/// `arn:${Partition}:iam::${Account}:user/${UserNameWithPath}`
///
pub fn user(partition: Identifier, account: AccountIdentifier, user_name: Identifier) -> ARN {
    ArnBuilder::service_id(IdentityAccessManagement.into())
        .in_partition_id(partition)
        .owned_by(account)
        .is(ResourceIdentifier::from_id_path(&[
            Identifier::new_unchecked("user"),
            user_name,
        ]))
        .into()
}

///
/// `arn:${Partition}:iam::${Account}:role/${RoleNameWithPath}`
///
pub fn role(partition: Identifier, account: AccountIdentifier, role_name: Identifier) -> ARN {
    ArnBuilder::service_id(IdentityAccessManagement.into())
        .in_partition_id(partition)
        .owned_by(account)
        .is(ResourceIdentifier::from_id_path(&[
            Identifier::new_unchecked("role"),
            role_name,
        ]))
        .into()
}

///
/// `arn:${Partition}:iam::${Account}:group/${GroupNameWithPath}`
///
pub fn group(partition: Identifier, account: AccountIdentifier, group_name: Identifier) -> ARN {
    ArnBuilder::service_id(IdentityAccessManagement.into())
        .in_partition_id(partition)
        .owned_by(account)
        .is(ResourceIdentifier::from_id_path(&[
            Identifier::new_unchecked("group"),
            group_name,
        ]))
        .into()
}

///
/// `arn:${Partition}:iam::${Account}:policy/${PolicyNameWithPath}`
///
pub fn policy(partition: Identifier, account: AccountIdentifier, policy_name: Identifier) -> ARN {
    ArnBuilder::service_id(IdentityAccessManagement.into())
        .in_partition_id(partition)
        .owned_by(account)
        .is(ResourceIdentifier::from_id_path(&[
            Identifier::new_unchecked("policy"),
            policy_name,
        ]))
        .into()
}
