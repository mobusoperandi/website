use std::collections::{BTreeMap, BTreeSet};

use relative_path::RelativePathBuf;

// key is expected target, values are targets expecting it
#[derive(Debug, Clone, thiserror::Error)]
#[error("missing targets: {0:?}")]
pub(super) struct MissingTargets(BTreeMap<RelativePathBuf, BTreeSet<RelativePathBuf>>);

impl MissingTargets {
    pub(crate) fn new(
        expected_targets: BTreeMap<RelativePathBuf, BTreeSet<RelativePathBuf>>,
        processed_targets: &BTreeSet<RelativePathBuf>,
    ) -> Option<Self> {
        let missing_targets: BTreeMap<_, _> = expected_targets
            .into_iter()
            .filter(|(expected, _expectors)| !processed_targets.contains(expected))
            .collect();

        if missing_targets.is_empty() {
            None
        } else {
            Some(Self(missing_targets))
        }
    }
}
