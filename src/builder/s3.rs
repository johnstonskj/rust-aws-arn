/*!
Provides a set of simple helper functions to make ARNs for the S3 service.

These resource definitions ae take from the AWS
[documentation]( https://docs.aws.amazon.com/IAM/latest/UserGuide/list_amazons3.html#amazons3-resources-for-iam-policies)
*/

use crate::builder::ArnBuilder;
use crate::known::Partition;
use crate::known::Service::S3;
use crate::{AccountIdentifier, Identifier, ResourceIdentifier, ARN};

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

///
/// `arn:${Partition}:s3:::${BucketName}`
///
pub fn bucket_in(partition: Identifier, bucket_name: Identifier) -> ARN {
    ArnBuilder::service_id(S3.into())
        .in_partition_id(partition)
        .is(bucket_name.into())
        .into()
}

///
/// `arn:aws:s3:::${BucketName}`
///
pub fn bucket(bucket_name: Identifier) -> ARN {
    bucket_in(Partition::default().into(), bucket_name)
}

///
/// `arn:${Partition}:s3:::${BucketName}/${ObjectName}`
///
pub fn object_in(partition: Identifier, bucket_name: Identifier, object_name: Identifier) -> ARN {
    ArnBuilder::service_id(S3.into())
        .in_partition_id(partition)
        .is(ResourceIdentifier::from_id_path(&[
            bucket_name,
            object_name,
        ]))
        .into()
}

///
/// `arn:aws:s3:::${BucketName}/${ObjectName}`
///
pub fn object(bucket_name: Identifier, object_name: Identifier) -> ARN {
    object_in(Partition::default().into(), bucket_name, object_name)
}

///
/// `arn:aws:s3:::${BucketName}/${ObjectName}`
///
/// This function will panic if `bucket` is not an ARN for an S3 bucket.
///
pub fn object_from(bucket: &ARN, object_name: Identifier) -> ARN {
    if bucket.service != S3.into() {
        panic!("You can't make an S3 object from a {} ARN.", bucket.service);
    }
    ARN {
        resource: ResourceIdentifier::from_path(&[bucket.resource.clone(), object_name.into()]),
        ..bucket.clone()
    }
}

///
/// `arn:${Partition}:s3:${Region}:${Account}:job/${JobId}`
///
pub fn job_in(
    partition: Identifier,
    region: Identifier,
    account: AccountIdentifier,
    job_id: Identifier,
) -> ARN {
    ArnBuilder::service_id(S3.into())
        .in_partition_id(partition)
        .in_region_id(region)
        .owned_by(account)
        .is(job_id.into())
        .into()
}

///
/// `arn:aws:s3:${Region}:${Account}:job/${JobId}`
///
pub fn job(region: Identifier, account: AccountIdentifier, job_id: Identifier) -> ARN {
    job_in(Partition::default().into(), region, account, job_id)
}
