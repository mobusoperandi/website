use std::collections::{BTreeMap, BTreeSet};

use relative_path::RelativePathBuf;

// key is expected file path, values are paths of files expecting it
#[derive(Debug, Clone, thiserror::Error)]
#[error("missing files: {0:?}")]
pub(super) struct MissingFiles(BTreeMap<RelativePathBuf, BTreeSet<RelativePathBuf>>);

impl MissingFiles {
    pub(crate) fn new(
        expected_files: BTreeMap<RelativePathBuf, BTreeSet<RelativePathBuf>>,
        processed_files: &BTreeSet<RelativePathBuf>,
    ) -> Option<Self> {
        let missing_files: BTreeMap<_, _> = expected_files
            .into_iter()
            .filter(|(expected, _expectors)| !processed_files.contains(expected))
            .collect();

        if missing_files.is_empty() {
            None
        } else {
            Some(Self(missing_files))
        }
    }
}
