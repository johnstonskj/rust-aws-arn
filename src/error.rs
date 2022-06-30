/*!
One-line description.

More detailed description, with

# Example

*/

use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// Errors that may arise parsing an ARN with `FromStr::from_str()`.
///
#[derive(Debug, PartialEq)]
pub enum ArnError {
    /// String length must be greater than 8 corresponding to `"arn:::::"`.
    TooShort,
    /// String length must be under 2048 characters..
    TooLong,
    /// Need at least 6 components.
    TooFewComponents,
    /// Invalid `Identifier` string value.
    InvalidIdentifier(String),
    /// Missing the 'arn' prefix string.
    MissingPrefix,
    /// Missing the partition component.
    MissingPartition,
    /// The partition component provided is not valid.
    InvalidPartition,
    /// Missing the service component.
    MissingService,
    /// The service component provided is not valid.
    InvalidService,
    /// Missing the region component.
    MissingRegion,
    /// The partition region provided is not valid.
    InvalidRegion,
    /// The particular resource type does not allow region wildcards.
    RegionWildcardNotAllowed,
    /// Missing the account id component.
    MissingAccountId,
    /// The partition account id provided is not valid.
    InvalidAccountId(String),
    /// The particular resource type does not allow account wildcards.
    AccountIdWildcardNotAllowed,
    /// Missing the resource component.
    MissingResource,
    /// The partition resource provided is not valid, the name of the particular component
    /// in error is included.
    InvalidResource(String),
    /// The particular resource type does not allow resource wildcards.
    ResourceWildcardNotAllowed,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Display for ArnError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:?}", self)
    }
}

impl Error for ArnError {}
