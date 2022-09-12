use const_env::from_env;

#[from_env]
pub(crate) const DEVELOPMENT: bool = false;

pub(crate) const OUTPUT_DIR: &str = env!("OUTPUT_DIR");
