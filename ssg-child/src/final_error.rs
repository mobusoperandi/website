mod duplicates;
mod failed_targets;
mod missing_targets;
mod processed_targets_count;

use std::{
    collections::{BTreeMap, BTreeSet},
    fmt::Display,
};

use relative_path::RelativePathBuf;

use crate::{sources::ExpectedTargets, target_success::TargetSuccess, TargetError};

use self::{
    duplicates::Duplicates, failed_targets::FailedTargets, missing_targets::MissingTargets,
    processed_targets_count::ProcessedTargetsCount,
};

#[derive(Debug, Clone, getset::Getters, thiserror::Error)]
pub struct FinalError {
    duplicates: Option<Duplicates>,
    missing_targets: Option<MissingTargets>,
    failed_targets: Option<FailedTargets>,
}

impl Display for FinalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(duplicates) = &self.duplicates {
            writeln!(f, "{duplicates}")?
        }

        if let Some(missing_targets) = &self.missing_targets {
            writeln!(f, "{missing_targets}")?
        }

        if let Some(failed_targets) = &self.failed_targets {
            writeln!(f, "{failed_targets}")?
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Default)]
pub(crate) struct FinalErrorBuilder {
    processed_targets_count: ProcessedTargetsCount,
    expected_targets: BTreeMap<RelativePathBuf, BTreeSet<RelativePathBuf>>,
    failed_targets: BTreeSet<RelativePathBuf>,
}

impl FinalErrorBuilder {
    pub(crate) fn add(mut self, processing_result: Result<TargetSuccess, TargetError>) -> Self {
        let (target, expected_targets) = match &processing_result {
            Ok(success) => {
                let target = success.path().clone();
                let expected_targets = success.expected_targets().clone();
                (target, expected_targets)
            }
            Err(target_error) => {
                let target = target_error.spec_target_path().clone();
                self.failed_targets.insert(target.clone());
                (target, ExpectedTargets::default())
            }
        };

        expected_targets.into_iter().for_each(|expected_target| {
            self.expected_targets
                .entry(expected_target)
                .or_default()
                .insert(target.clone());
        });

        *self.processed_targets_count.entry(target).or_default() += 1;

        self
    }

    pub(crate) fn build(self) -> Option<FinalError> {
        let missing_targets = MissingTargets::new(
            self.expected_targets,
            self.processed_targets_count.keys().collect(),
        );

        let duplicates = Duplicates::from_processed_targets_count(self.processed_targets_count);

        let failed_targets = if self.failed_targets.is_empty() {
            None
        } else {
            Some(FailedTargets::new(self.failed_targets))
        };

        if duplicates.is_some() || missing_targets.is_some() || failed_targets.is_some() {
            Some(FinalError {
                duplicates,
                failed_targets,
                missing_targets,
            })
        } else {
            None
        }
    }
}
