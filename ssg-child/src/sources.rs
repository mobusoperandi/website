mod static_byte_slice;
pub mod bytes_with_file_spec_safety;
pub mod google_font;
pub mod http;

use futures::future::BoxFuture;
pub use google_font::*;
pub use http::*;

use crate::Targets;

pub trait FileSource {
    fn obtain_content(
        &self,
        targets: Targets,
    ) -> BoxFuture<Result<Vec<u8>, Box<dyn std::error::Error + Send>>>;
}
