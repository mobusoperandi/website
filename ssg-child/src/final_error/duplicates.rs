use std::collections::BTreeMap;

use relative_path::RelativePathBuf;

use super::processed_targets_count::ProcessedTargetsCount;

#[derive(Debug, Clone, thiserror::Error)]
#[error("duplicates: {0:?}")]
pub struct Duplicates(BTreeMap<RelativePathBuf, usize>);

impl Duplicates {
    pub(super) fn from_processed_targets_count(
        processed_targets_count: ProcessedTargetsCount,
    ) -> Option<Self> {
        let duplicates = processed_targets_count
            .into_iter()
            .filter(|(_target, count)| *count > 1)
            .collect::<BTreeMap<RelativePathBuf, usize>>();

        if duplicates.is_empty() {
            None
        } else {
            Some(Self(duplicates))
        }
    }
}
