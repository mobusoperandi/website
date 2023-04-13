use schema::Schema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Schema)]
/// A link that showcases the mob
pub(crate) enum Link {
    /// A YouTube channel ID
    ///
    /// Example:
    ///
    /// ```yaml
    /// !YouTube "@mobseattle"
    /// ```
    YouTube(String),
}
