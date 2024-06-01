use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, derive_more::Display, Serialize, Deserialize)]
pub(crate) struct Id(String);

impl Id {
    pub(super) fn new(id: String) -> Self {
        Self(id)
    }
}
