#![deny(clippy::all, clippy::pedantic)]

use camino::Utf8PathBuf;
use once_cell::sync::Lazy;

pub static OUTPUT_DIR: Lazy<Utf8PathBuf> = Lazy::new(|| {
    [env!("CARGO_MANIFEST_DIR"), "..", ".vercel/output/static"]
        .iter()
        .collect()
});
