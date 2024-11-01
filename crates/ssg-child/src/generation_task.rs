use std::{future::IntoFuture, pin::Pin};

use futures::{Future, Stream, StreamExt};

use crate::{
    file_error::FileError,
    file_success::FileSuccess,
    final_error::{FinalError, FinalErrorBuilder},
};

type FileResultFn = Box<dyn Fn(&Result<FileSuccess, FileError>) + 'static>;

pub struct GenerationTask {
    file_results: Pin<Box<dyn Stream<Item = Result<FileSuccess, FileError>>>>,
    file_result_fn: Option<FileResultFn>,
}

impl GenerationTask {
    pub(crate) fn new(
        stream: impl Stream<Item = Result<FileSuccess, FileError>> + 'static,
    ) -> Self {
        Self {
            file_results: Box::pin(stream),
            file_result_fn: None,
        }
    }

    pub fn set_file_result_fn(
        &mut self,
        file_result_fn: impl Fn(&Result<FileSuccess, FileError>) + 'static,
    ) {
        self.file_result_fn = Some(Box::new(file_result_fn));
    }
}

impl IntoFuture for GenerationTask {
    type Output = Result<(), FinalError>;
    type IntoFuture = Pin<Box<dyn Future<Output = Self::Output>>>;

    fn into_future(self) -> Self::IntoFuture {
        let Self {
            file_results,
            file_result_fn,
        } = self;

        let future = async {
            let final_error = file_results
                .map(move |file_result| {
                    if let Some(f) = &file_result_fn {
                        f(&file_result);
                    };

                    file_result
                })
                .fold(FinalErrorBuilder::default(), |builder, result| async move {
                    builder.add(&result)
                })
                .await
                .build();

            if let Some(final_error) = final_error {
                Err(final_error)
            } else {
                Ok(())
            }
        };

        Box::pin(future)
    }
}
