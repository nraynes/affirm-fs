use crate::AffirmFsError;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum Contents {
    /// Indicates that contents of a file have yet to be retrieved.
    /// This is a way of saving space when the contents of a file are not needed for a comparison.
    Stale,

    /// Indicates there was an error during the last attempt to retrieve a files contents.
    Err(AffirmFsError),

    /// Indicates that the contents of a file have been attempted to be retrieved.
    Has(Vec<u8>),
}
