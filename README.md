# Crate aws-arn

Provides the types, builders, and other helpers to manipulate AWS
[Amazon Resource Name (ARN)](https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html)
strings.

![MIT License](https://img.shields.io/badge/license-mit-118811.svg)
![Minimum Rust Version](https://img.shields.io/badge/Min%20Rust-1.34-green.svg)
[![crates.io](https://img.shields.io/crates/v/aws-arn.svg)](https://crates.io/crates/aws-arn)
[![docs.rs](https://docs.rs/aws-arn/badge.svg)](https://docs.rs/aws-arn)
![Build](https://github.com/johnstonskj/rust-aws-arn/workflows/Rust/badge.svg)
![Audit](https://github.com/johnstonskj/rust-aws-arn/workflows/Security%20audit/badge.svg)
[![GitHub stars](https://img.shields.io/github/stars/johnstonskj/rust-aws-arn.svg)](https://github.com/johnstonskj/rust-aws-arn/stargazers)


The ARN is a key component of all AWS service APIs and yet nearly all client
toolkits treat it simply as a string. While this may be a reasonable and
expedient decision, it seems there might be a need to not only ensure
correctness of ARNs with validators but also constructors that allow making
these strings correclt in the first place.

# ARN Types

This crate provides a number of levels of ARN manipulation, the first is the
direct construction of an ARN type using the core `ResourceName`,
`Identifier`, `AccountIdentifier`, and `ResourceIdentifier` types.

```rust
use aws_arn::{ResourceName, ResourceIdentifier};
use aws_arn::known::{Partition, Service};
use std::str::FromStr;

let arn = ResourceName {
    partition: Some(Partition::default().into()),
    service: Service::S3.into(),
    region: None,
    account_id: None,
    resource: ResourceIdentifier::from_str("mythings/thing-1").unwrap()
};
```

In the example above, as we are defining a minimal ResourceName we could use one of the
defined constructor functions.

```rust
use aws_arn::{ResourceName, ResourceIdentifier};
use aws_arn::known::Service;
use std::str::FromStr;

let arn = ResourceName::aws(
    Service::S3.into(),
    ResourceIdentifier::from_str("mythings/thing-1").unwrap()
);
```

Alternatively, using `FromStr,` you can parse an existing ARN string into an ARN value.

```rust
use aws_arn::ResourceName;
use std::str::FromStr;

let arn: ResourceName = "arn:aws:s3:::mythings/thing-1"
    .parse()
    .expect("didn't look like an ResourceName");
```

Another approach is to use a more readable *builder* which also allows you to ignore those fields
in the ARN you don't always need and uses a more fluent style of ARN construction.

```rust
45Muse aws_arn::builder::{ArnBuilder, ResourceBuilder};
use aws_arn::known::{Partition, Service};
use aws_arn::{ResourceName, Identifier};
use std::str::FromStr;

let arn: ResourceName = ArnBuilder::service_id(Service::S3.into())
    .resource(ResourceBuilder::named(Identifier::from_str("mythings").unwrap())
        .resource_name(Identifier::new_unchecked("my-layer"))
        .build_resource_path())
    .in_partition_id(Partition::Aws.into())
    .into();
```

Finally, it is possible to use resource-type specific functions that allow an even more direct and
simple construction (module `aws_arn::builder::{service}` - *service builder functions*, although
at this time there are few supported services.

```rust
use aws_arn::builder::s3;
use aws_arn::Identifier;
use std::str::FromStr;

let arn = s3::object(
    Identifier::from_str("mythings").unwrap(),
    Identifier::from_str("thing-1").unwrap(),
);
```

For more, see the AWS documentation for [Amazon Resource Name
(ARN)](https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html) documentation.

# Optional Features

This crate has attempted to be as lean as possible, with a really minimal set of dependencies,
we have include the following capabilities as optional features.

* `builders` adds the builder module. This feature is enabled by default, it also requires the
  `known` feature.
* `known` adds a module containing enums for partitions, regions, and services.
  This feature is enabled by default.
* `serde_support` adds derived `Serialize` and `Deserialize` implementations for the `ARN` and
  `Resource` types. This feature is enabled by default.

## Changes

**Version 0.3.1**

* Added unit tests for `AccountIdentifier`.

**Version 0.3.0**

* **Breaking Change**: Renamed `ARN` to `ResourceName`.
* **Breaking Change**: Renamed `ArnError` to `Error`.
* Added interface for common Identifier operations.
* Added variable expansion for `ResourceIdentifier` and `ResourceName`.
* Added more unit tests.

**Version 0.2.1**

* Created a new `AccountIdentifier` type for the 12-digit value.
* `consts` feature renamed `known`.

**Version 0.2.0**

* Relaxed validation of identifiers and resource identifiers.
* Removed `Resource` type which added a lot of the validation confusion.
* Using new `Identifier` and `ResourceIdentifier` types to construct correct `ARN` values without the need for any
  external validation methods.
* Replaced `ResourceBuilder` with one for `ResourceIdentifier` values.
* Removed `ext_validation` feature
* Added `consts` feature
* Placed `builder` module into new `builders` feature.
* Added a lot more tests including an `examples.txt` file that is just a long list to be parsed.
* Fixed Github [issue-2](https://github.com/johnstonskj/rust-aws-arn/issues/2).
  
**Version 0.1.1**

* Documentation additions and fixes, in both README and Rustdoc.
* Renamed service builder functions, added a parent/child pattern for s3.
* Added Serde feature.
* Made external validation optional.

**Version 0.1.0**

* Initial commit.
* Provides basic ARN type with `Display` and `FromStr`.
* Provides scaffolding for service-specific validation.
* Provides initial set of service builder, `make_{format}`, functions for ARN construction.

## TODO

* More tests!
* More service formats for validation.
* More service builder functions.
