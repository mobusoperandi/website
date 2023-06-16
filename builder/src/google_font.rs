use futures::future::BoxFuture;
use maud::{Markup, PreEscaped, Render};
use ssg_child::sources::FileSource;

use crate::relative_path::RelativePathBuf;

#[derive(Debug, Clone, derive_more::Display)]
pub(crate) struct GoogleFont(ssg_child::sources::GoogleFont);

impl GoogleFont {
    pub(crate) fn new(family: String, subset: String, variant: String) -> GoogleFont {
        Self(ssg_child::sources::GoogleFont::new(family, subset, variant))
    }

    pub(crate) fn family(&self) -> &str {
        self.0.family()
    }

    pub(crate) fn filename(&self) -> RelativePathBuf {
        format!("{}.ttf", self.family().to_lowercase()).into()
    }
}

impl FileSource for GoogleFont {
    fn obtain_content(
        &self,
    ) -> BoxFuture<Result<ssg_child::sources::FileContents, Box<dyn std::error::Error + Send>>>
    {
        self.0.obtain_content()
    }
}

impl Render for GoogleFont {
    fn render(&self) -> Markup {
        PreEscaped(format!(
            "
            @font-face {{
                font-family: '{}';
                src: url('/{}') format('truetype');
            }}",
            self.family(),
            self.filename()
        ))
    }
}
