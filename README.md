# Crate aws-arn
Provides types, builders, and other helpers to manipulate AWS Amazon Resource Name (ARN) strings.

![MIT License](https://img.shields.io/badge/license-mit-118811.svg)
![Minimum Rust Version](https://img.shields.io/badge/Min%20Rust-1.34-green.svg)
[![crates.io](https://img.shields.io/crates/v/aws-arn.svg)](https://crates.io/crates/aws-arn)
[![docs.rs](https://docs.rs/aws-arn/badge.svg)](https://docs.rs/aws-arn)
[![travis.ci](https://travis-ci.org/johnstonskj/rust-aws-arn.svg?branch=master)](https://travis-ci.org/johnstonskj/rust-aws-arn)
[![GitHub stars](https://img.shields.io/github/stars/johnstonskj/rust-aws-arn.svg)](https://github.com/johnstonskj/rust-aws-arn/stargazers)

The ARN is a key component of all AWS service APIs and yet nearly all client toolkits treat it simply as a string. While this may be a reasonable and expedient decision, it seems there might be a need to not only ensure correctness of ARNs with validators but also constructors that allow making these strings correclt in the first place. 

This crate provides three levels of ARN manipulation, the first is the direct construction of an ARN type (module `aws_arn` - the core `Resource` and `ARN` types).

```rust
let arn = ARN {
    partition: Some("aws".to_string(),
    service: "s3".to_string(),
    region: None,
    account: None,
    resource: Resource::Path("".to_string())};
```

Or, alternatively using `FromStr` you can parse a string into an ARN.

```rust
let arn: ARN = "arn:aws:s3:::mythings/thing-1".parse().expect("didn't look like an ARN");
```

The next is to use a more readable builder which also allows you to ignore those fields in the ARN you don't always need (module `aws_arn::builder` - the `ResourceBuilder` and `ArnBuilder` types providing a more fluent style of ARN construction).
 
```rust
let arn = ArnBuilder::new("s3")
    .resource(ResourceBuilder::new(&format!("{}/{}", "mythings", "thing-1")).build())
    .in_partition("aws")
    .build();
```
 
Finally, it is possible to use resource-type specific functions that allow an even more direct and simple construction (module `aws_arn::builder::{service}` - *service builder functions*.
 
```rust
let arn = s3::object("mythings", "thing-1");
```

## Features

This crate has attempted to be as lean as possible, with a really minimal set of dependencies, we have include the following as features.

* `serde_support` derives `Serialize` and `Deserialize` for the `ARN` and `Resource` types.
* `ext_validation` adds extended, service specific, validation using an external configuration file.

## Changes

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
