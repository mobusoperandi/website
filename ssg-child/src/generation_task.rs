use std::{future::IntoFuture, pin::Pin};

use futures::{Future, Stream, StreamExt};

use crate::{
    final_error::{FinalError, FinalErrorBuilder},
    target_error::TargetError,
    target_success::TargetSuccess,
};

type OnTargetResult = Box<dyn Fn(&Result<TargetSuccess, TargetError>) + 'static>;

pub struct GenerationTask {
    target_results: Pin<Box<dyn Stream<Item = Result<TargetSuccess, TargetError>>>>,
    on_target_result: Option<OnTargetResult>,
}

impl GenerationTask {
    pub(crate) fn new(
        stream: impl Stream<Item = Result<TargetSuccess, TargetError>> + 'static,
    ) -> Self {
        Self {
            target_results: Box::pin(stream),
            on_target_result: None,
        }
    }

    pub fn on_target_result(&mut self, f: impl Fn(&Result<TargetSuccess, TargetError>) + 'static) {
        self.on_target_result = Some(Box::new(f));
    }
}

impl IntoFuture for GenerationTask {
    type Output = Result<(), FinalError>;
    type IntoFuture = Pin<Box<dyn Future<Output = Self::Output>>>;

    fn into_future(self) -> Self::IntoFuture {
        let Self {
            target_results,
            on_target_result,
        } = self;

        let future = async {
            let final_error = target_results
                .map(move |target_result| {
                    if let Some(f) = &on_target_result {
                        f(&target_result);
                    };

                    target_result
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
