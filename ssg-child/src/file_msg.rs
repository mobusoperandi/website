use relative_path::RelativePathBuf;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct FileMsg {
    pub path: RelativePathBuf,
    pub contents: Vec<u8>,
}

impl FileMsg {
    pub const CHANNEL_ENV_VAR: &str = concat!(env!("CARGO_CRATE_NAME"), "_MSG_CHANNEL_KEY");
}
