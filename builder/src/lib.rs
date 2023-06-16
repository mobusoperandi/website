#![deny(
    clippy::all,
    clippy::complexity,
    clippy::correctness,
    clippy::pedantic,
    clippy::perf,
    clippy::style,
    clippy::suspicious
)]

use camino::Utf8PathBuf;
use once_cell::sync::Lazy;

pub static OUTPUT_DIR: Lazy<Utf8PathBuf> = Lazy::new(|| {
    [env!("CARGO_MANIFEST_DIR"), "..", ".vercel/output/static"]
        .iter()
        .collect()
});
