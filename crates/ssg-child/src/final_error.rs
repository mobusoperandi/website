mod duplicates;
mod failed_files;
mod missing_files;
mod processed_files_count;

use std::{
    collections::{BTreeMap, BTreeSet},
    fmt::Display,
};

use relative_path::RelativePathBuf;

use crate::{file_success::FileSuccess, sources::ExpectedFiles, FileError};

use self::{
    duplicates::Duplicates, failed_files::FailedFiles, missing_files::MissingFiles,
    processed_files_count::ProcessedFilesCount,
};

#[derive(Debug, Clone, getset::Getters, thiserror::Error)]
pub struct FinalError {
    duplicates: Option<Duplicates>,
    missing_files: Option<MissingFiles>,
    failed_files: Option<FailedFiles>,
}

impl Display for FinalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(duplicates) = &self.duplicates {
            writeln!(f, "{duplicates}")?;
        }

        if let Some(missing_files) = &self.missing_files {
            writeln!(f, "{missing_files}")?;
        }

        if let Some(failed_files) = &self.failed_files {
            writeln!(f, "{failed_files}")?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Default)]
pub(crate) struct FinalErrorBuilder {
    processed_files_count: ProcessedFilesCount,
    expected_files: BTreeMap<RelativePathBuf, BTreeSet<RelativePathBuf>>,
    failed_files: BTreeSet<RelativePathBuf>,
}

impl FinalErrorBuilder {
    pub(crate) fn add(mut self, processing_result: &Result<FileSuccess, FileError>) -> Self {
        let (path, expected_files) = match &processing_result {
            Ok(success) => {
                let path = success.path().clone();
                let expected_files = success.expected_files().clone();
                (path, expected_files)
            }
            Err(file_error) => {
                let path = file_error.path().clone();
                self.failed_files.insert(path.clone());
                (path, ExpectedFiles::default())
            }
        };

        expected_files.into_iter().for_each(|expected_file| {
            self.expected_files
                .entry(expected_file)
                .or_default()
                .insert(path.clone());
        });

        *self.processed_files_count.entry(path).or_default() += 1;

        self
    }

    pub(crate) fn build(self) -> Option<FinalError> {
        let processed_files = self
            .processed_files_count
            .clone()
            .into_keys()
            .collect::<BTreeSet<RelativePathBuf>>();

        let missing_files = MissingFiles::new(self.expected_files, &processed_files);

        let duplicates = Duplicates::from_processed_files_count(self.processed_files_count);

        let failed_files = if self.failed_files.is_empty() {
            None
        } else {
            Some(FailedFiles::new(self.failed_files))
        };

        if duplicates.is_some() || missing_files.is_some() || failed_files.is_some() {
            Some(FinalError {
                duplicates,
                missing_files,
                failed_files,
            })
        } else {
            None
        }
    }
}
