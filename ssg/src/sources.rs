use self::{bytes_with_file_spec_safety::BytesWithFileSpecSafety, google_font::GoogleFont};

pub mod bytes_with_file_spec_safety;
pub mod google_font;
pub mod http;

use reqwest::Url;

pub enum FileSource {
    Static(&'static [u8]),
    BytesWithFileSpecSafety(BytesWithFileSpecSafety),
    GoogleFont(GoogleFont),
    Http(Url),
}
