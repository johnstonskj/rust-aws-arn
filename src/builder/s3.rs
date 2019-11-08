/*!
Provides a set of simple helper functions to make ARNs for the S3 service.
*/

use crate::builder::{ArnBuilder, ResourceBuilder};
use crate::ARN;

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

///
/// From [doc]( https://docs.aws.amazon.com/IAM/latest/UserGuide/list_amazons3.html#amazons3-resources-for-iam-policies)
///
/// `arn:${Partition}:s3:::${BucketName}`
///
pub fn bucket(partition: &str, bucket_name: &str) -> ARN {
    ArnBuilder::new("s3")
        .in_partition(partition)
        .is(ResourceBuilder::new(bucket_name).build())
        .build()
}

///
/// From [doc]( https://docs.aws.amazon.com/IAM/latest/UserGuide/list_amazons3.html#amazons3-resources-for-iam-policies)
///
/// `arn:${Partition}:s3:::${BucketName}/${ObjectName}`
///
pub fn object(partition: &str, bucket_name: &str, object_name: &str) -> ARN {
    ArnBuilder::new("s3")
        .in_partition(partition)
        .is(ResourceBuilder::new(&format!("{}/{}", bucket_name, object_name)).build())
        .build()
}

///
/// From [doc]( https://docs.aws.amazon.com/IAM/latest/UserGuide/list_amazons3.html#amazons3-resources-for-iam-policies)
///
/// `arn:${Partition}:s3:${Region}:${Account}:job/${JobId}`
///
pub fn job(partition: &str, region: &str, account: &str, job_id: &str) -> ARN {
    ArnBuilder::new("s3")
        .in_partition(partition)
        .in_region(region)
        .owned_by(account)
        .is(ResourceBuilder::new(job_id).build())
        .build()
}
