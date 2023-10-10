#![warn(clippy::all, clippy::pedantic)]

mod dev;
mod parent;

pub use dev::{dev, DevError};
pub use parent::Parent;
