#![warn(clippy::all, clippy::pedantic)]

use camino::Utf8PathBuf;
use once_cell::sync::Lazy;

pub static OUTPUT_DIR: Lazy<Utf8PathBuf> = Lazy::new(|| {
    Utf8PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join(".vercel/output/static")
});
