use aws_arn::builder::ArnBuilder;
use aws_arn::known::Region::UsEast2;
use aws_arn::known::Service::{Lambda, S3};
use aws_arn::{AccountIdentifier, Identifier, ResourceIdentifier, ResourceName};
use std::str::FromStr;

#[test]
fn test_s3_bucket() {
    let arn: ResourceName = ArnBuilder::service_id(S3.into())
        .resource(ResourceIdentifier::from_str("my-bucket").unwrap())
        .into();
    assert_eq!(arn.to_string(), "arn:aws:s3:::my-bucket");
}

#[test]
fn test_lambda_layer() {
    let arn: ResourceName = ArnBuilder::service_id(Lambda.into())
        .resource(ResourceIdentifier::from_qualified_id(&[
            Identifier::from_str("layer").unwrap(),
            Identifier::from_str("my-layer").unwrap(),
            Identifier::from_str(&3.to_string()).unwrap(),
        ]))
        .in_region_id(UsEast2.into())
        .owned_by(AccountIdentifier::from_str("123456789012").unwrap())
        .into();
    assert_eq!(
        arn.to_string(),
        "arn:aws:lambda:us-east-2:123456789012:layer:my-layer:3"
    );
}
