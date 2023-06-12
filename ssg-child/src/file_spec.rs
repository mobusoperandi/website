use std::collections::BTreeSet;

use camino::Utf8PathBuf;
use futures::{future::BoxFuture, FutureExt};
use getset::Getters;
use relative_path::RelativePathBuf;
use tokio::{fs, io::AsyncWriteExt};

use crate::{
    sources::{bytes_with_file_spec_safety::Targets, FileSource},
    target_error::{TargetError, TargetErrorCause},
    target_success::TargetSuccess,
};

#[derive(Getters)]
pub struct FileSpec {
    source: Box<dyn FileSource + Send>,
    #[getset(get = "pub(crate)")]
    target: RelativePathBuf,
}

impl FileSpec {
    pub fn new<T>(target: T, source: impl FileSource + 'static + Send) -> Self
    where
        RelativePathBuf: From<T>,
    {
        Self {
            source: Box::new(source),
            target: target.into(),
        }
    }

    pub(crate) fn into_source(self) -> Box<dyn FileSource + Send> {
        self.source
    }

    pub(crate) fn generate(
        self,
        targets: BTreeSet<RelativePathBuf>,
        output_dir: Utf8PathBuf,
    ) -> BoxFuture<'static, Result<TargetSuccess, TargetError>> {
        async move {
            let this_target = self.target().clone();
            let targets = Targets::new(this_target.clone(), targets);
            let source = self.into_source();
            let task = source.obtain_content(targets);

            let file_path = this_target.to_path(output_dir);

            fs::create_dir_all(file_path.parent().unwrap())
                .await
                .map_err(|error| {
                    TargetError::new(this_target.clone(), TargetErrorCause::TargetIo(error))
                })?;

            let contents = task.await.map_err(|error| {
                TargetError::new(this_target.clone(), TargetErrorCause::Source(error))
            })?;

            let mut file_handle = fs::OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(file_path)
                .await
                .map_err(|error| {
                    TargetError::new(this_target.clone(), TargetErrorCause::TargetIo(error))
                })?;

            file_handle.write_all(&contents).await.map_err(|error| {
                TargetError::new(this_target.clone(), TargetErrorCause::TargetIo(error))
            })?;

            Ok(TargetSuccess::new(this_target))
        }
        .boxed()
    }
}
