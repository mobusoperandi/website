pub struct Parent {
    output_dir: camino::Utf8PathBuf,
}

impl Parent {
    #[must_use]
    pub fn new(output_dir: camino::Utf8PathBuf) -> Self {
        Self { output_dir }
    }
}
