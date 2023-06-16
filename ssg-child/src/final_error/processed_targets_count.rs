use std::collections::{btree_map::Entry, BTreeMap};

use relative_path::RelativePathBuf;

#[derive(Debug, Clone, Default, derive_more::IntoIterator)]
pub(super) struct ProcessedTargetsCount(BTreeMap<RelativePathBuf, usize>);

impl ProcessedTargetsCount {
    pub(crate) fn entry(&mut self, path: RelativePathBuf) -> Entry<RelativePathBuf, usize> {
        self.0.entry(path)
    }

    pub(crate) fn into_keys(self) -> impl Iterator<Item = RelativePathBuf> {
        self.0.into_keys()
    }
}
