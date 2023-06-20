use camino::Utf8PathBuf;
use futures::{future::BoxFuture, FutureExt};
use getset::Getters;
use relative_path::RelativePathBuf;
use tokio::{fs, io::AsyncWriteExt};

use crate::{
    file_error::{FileError, FileErrorCause},
    file_success::FileSuccess,
    sources::FileSource,
};

#[derive(Getters)]
pub struct FileSpec {
    source: Box<dyn FileSource + Send>,
    #[getset(get = "pub(crate)")]
    path: RelativePathBuf,
}

impl FileSpec {
    pub fn new<T>(path: T, source: impl FileSource + 'static + Send) -> Self
    where
        RelativePathBuf: From<T>,
    {
        Self {
            source: Box::new(source),
            path: path.into(),
        }
    }

    pub(crate) fn into_source(self) -> Box<dyn FileSource + Send> {
        self.source
    }

    pub(crate) fn generate(
        self,
        output_dir: Utf8PathBuf,
    ) -> BoxFuture<'static, Result<FileSuccess, FileError>> {
        async move {
            let this_path = self.path().clone();
            let source = self.into_source();
            let task = source.obtain_content();

            let file_path = this_path.to_path(output_dir);

            fs::create_dir_all(file_path.parent().unwrap())
                .await
                .map_err(|error| {
                    FileError::new(this_path.clone(), FileErrorCause::OutputIo(error))
                })?;

            let contents = task.await.map_err(|error| {
                FileError::new(this_path.clone(), FileErrorCause::Source(error))
            })?;

            let mut file_handle = fs::OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(file_path)
                .await
                .map_err(|error| {
                    FileError::new(this_path.clone(), FileErrorCause::OutputIo(error))
                })?;

            file_handle
                .write_all(contents.bytes())
                .await
                .map_err(|error| {
                    FileError::new(this_path.clone(), FileErrorCause::OutputIo(error))
                })?;

            let expected_files = contents.expected_files().cloned();

            Ok(FileSuccess::new(this_path, expected_files))
        }
        .boxed()
    }
}
