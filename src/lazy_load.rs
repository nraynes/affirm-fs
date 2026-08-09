use crate::AffirmFsError;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum LazyLoad<T> {
    /// Indicates that the item has yet to be retrieved.
    Stale,

    /// Indicates there was an error during the last attempt to retrieve the item.
    Err(AffirmFsError),

    /// Indicates that the item has been retrieved.
    Has(T),
}
