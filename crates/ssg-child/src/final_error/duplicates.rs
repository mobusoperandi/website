use std::collections::BTreeMap;

use relative_path::RelativePathBuf;

use super::processed_files_count::ProcessedFilesCount;

#[derive(Debug, Clone, thiserror::Error)]
#[error("duplicates: {0:?}")]
pub struct Duplicates(BTreeMap<RelativePathBuf, usize>);

impl Duplicates {
    pub(super) fn from_processed_files_count(
        processed_files_count: ProcessedFilesCount,
    ) -> Option<Self> {
        let duplicates = processed_files_count
            .into_iter()
            .filter(|(_path, count)| *count > 1)
            .collect::<BTreeMap<RelativePathBuf, usize>>();

        if duplicates.is_empty() {
            None
        } else {
            Some(Self(duplicates))
        }
    }
}
