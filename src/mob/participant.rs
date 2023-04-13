pub(super) mod person;

use schema::Schema;
use serde::{Deserialize, Serialize};

pub(crate) use self::person::Person;

#[derive(Debug, Clone, Serialize, Deserialize, Schema)]
/// A participant in a mob
pub(crate) enum Participant {
    /// A mob member who prefers to remain anonymous
    ///
    /// Example:
    ///
    /// ```yaml
    /// !Hidden
    /// ```
    Hidden,
    /// A mob member who wishes to be publically listed"whitespace-nowrap" "font-mono"
    ///
    /// Example:
    ///
    /// ```yaml
    /// !Public
    /// name: Forbany Klenbin
    /// social_url: https://example.com/fk
    /// avatar_url: https://example.com/fk.png
    /// ```
    Public(Person),
}
