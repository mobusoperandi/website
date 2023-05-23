use std::path::PathBuf;

use once_cell::sync::Lazy;

pub static OUTPUT_DIR: Lazy<PathBuf> = Lazy::new(|| {
    [env!("CARGO_MANIFEST_DIR"), "..", ".vercel/output/static"]
        .iter()
        .collect()
});
