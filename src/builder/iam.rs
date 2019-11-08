/*!
Provides a set of simple helper functions to make ARNs for the IAM service.
*/

use crate::builder::{ArnBuilder, ResourceBuilder};
use crate::ARN;

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

///
/// From [doc](https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_identifiers.html#identifiers-arns)
///
/// `arn:aws:iam::123456789012:root`
///
pub fn root(account: &str) -> ARN {
    ArnBuilder::new("iam")
        .owned_by(account)
        .is(ResourceBuilder::new("root").build())
        .build()
}

///
/// From [doc](https://docs.aws.amazon.com/IAM/latest/UserGuide/list_identityandaccessmanagement.html#identityandaccessmanagement-resources-for-iam-policies)
///
/// `arn:${Partition}:iam::${Account}:user/${UserNameWithPath}`
///
pub fn user(partition: &str, account: &str, user_name: &str) -> ARN {
    ArnBuilder::new("iam")
        .in_partition(partition)
        .owned_by(account)
        .is(ResourceBuilder::new(user_name).is_an("user").build())
        .build()
}

///
/// From [doc](https://docs.aws.amazon.com/IAM/latest/UserGuide/list_identityandaccessmanagement.html#identityandaccessmanagement-resources-for-iam-policies)
///
/// `arn:${Partition}:iam::${Account}:role/${RoleNameWithPath}`
///
pub fn role(partition: &str, account: &str, role_name: &str) -> ARN {
    ArnBuilder::new("iam")
        .in_partition(partition)
        .owned_by(account)
        .is(ResourceBuilder::new(role_name).is_an("role").build())
        .build()
}

///
/// From [doc](https://docs.aws.amazon.com/IAM/latest/UserGuide/list_identityandaccessmanagement.html#identityandaccessmanagement-resources-for-iam-policies)
///
/// `arn:${Partition}:iam::${Account}:group/${GroupNameWithPath}`
///
pub fn group(partition: &str, account: &str, group_name: &str) -> ARN {
    ArnBuilder::new("iam")
        .in_partition(partition)
        .owned_by(account)
        .is(ResourceBuilder::new(group_name).is_an("group").build())
        .build()
}

///
/// From [doc](https://docs.aws.amazon.com/IAM/latest/UserGuide/list_identityandaccessmanagement.html#identityandaccessmanagement-resources-for-iam-policies)
///
/// `arn:${Partition}:iam::${Account}:policy/${PolicyNameWithPath}`
///
pub fn policy(partition: &str, account: &str, policy_name: &str) -> ARN {
    ArnBuilder::new("iam")
        .in_partition(partition)
        .owned_by(account)
        .is(ResourceBuilder::new(policy_name).is_an("policy").build())
        .build()
}
