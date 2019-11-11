/*!
Provides a set of simple helper functions to make ARNs for the S3 service.
*/

use crate::builder::{ArnBuilder, ResourceBuilder};
use crate::ARN;

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

///
/// The service name portion of the ARN.
///
pub const SERVICE_NAME: &str = "s3";

///
/// From [doc]( https://docs.aws.amazon.com/IAM/latest/UserGuide/list_amazons3.html#amazons3-resources-for-iam-policies)
///
/// `arn:${Partition}:s3:::${BucketName}`
///
pub fn bucket_in(partition: &str, bucket_name: &str) -> ARN {
    ArnBuilder::new(SERVICE_NAME)
        .in_partition(partition)
        .is(ResourceBuilder::new(bucket_name).build())
        .build()
}

pub fn bucket(bucket_name: &str) -> ARN {
    bucket_in("aws", bucket_name)
}

///
/// From [doc]( https://docs.aws.amazon.com/IAM/latest/UserGuide/list_amazons3.html#amazons3-resources-for-iam-policies)
///
/// `arn:${Partition}:s3:::${BucketName}/${ObjectName}`
///
pub fn object_in(partition: &str, bucket_name: &str, object_name: &str) -> ARN {
    ArnBuilder::new(SERVICE_NAME)
        .in_partition(partition)
        .is(ResourceBuilder::new(&format!("{}/{}", bucket_name, object_name)).build())
        .build()
}

pub fn object(bucket_name: &str, object_name: &str) -> ARN {
    object_in("aws", bucket_name, object_name)
}

///
/// From [doc]( https://docs.aws.amazon.com/IAM/latest/UserGuide/list_amazons3.html#amazons3-resources-for-iam-policies)
///
/// `arn:${Partition}:s3:${Region}:${Account}:job/${JobId}`
///
pub fn job_in(partition: &str, region: &str, account: &str, job_id: &str) -> ARN {
    ArnBuilder::new(SERVICE_NAME)
        .in_partition(partition)
        .in_region(region)
        .owned_by(account)
        .is(ResourceBuilder::new(job_id).build())
        .build()
}

pub fn job(region: &str, account: &str, job_id: &str) -> ARN {
    job_in("aws", region, account, job_id)
}
