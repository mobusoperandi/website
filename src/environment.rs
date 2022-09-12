use const_env::from_env;

#[from_env]
pub(crate) const DEVELOPMENT: bool = false;
