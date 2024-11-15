use futures::future::BoxFuture;
use maud::{Markup, PreEscaped, Render};
use ssg_child::sources::FileSource;

use crate::relative_path::RelativePathBuf;

#[derive(Debug, Clone)]
pub(crate) struct TrueTypeFont {
    bytes: &'static [u8],
    family: &'static str,
}

impl TrueTypeFont {
    pub(crate) const fn new(bytes: &'static [u8], family: &'static str) -> TrueTypeFont {
        Self { bytes, family }
    }

    pub(crate) fn family(&self) -> &str {
        self.family
    }

    pub(crate) fn filename(&self) -> RelativePathBuf {
        format!("{}.ttf", self.family().to_lowercase()).into()
    }
}

impl FileSource for TrueTypeFont {
    fn obtain_content(
        &self,
    ) -> BoxFuture<Result<ssg_child::sources::FileContents, Box<dyn std::error::Error + Send>>>
    {
        self.bytes.obtain_content()
    }
}

impl Render for TrueTypeFont {
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
