use const_format::formatcp;
use once_cell::sync::Lazy;

use crate::html::Classes;

pub(crate) const GRAYS: &str = "gray";
pub(crate) const BACKGROUND_COLOR: &str = formatcp!("{GRAYS}-900");
pub(crate) const TEXT_COLOR: &str = formatcp!("{GRAYS}-100");
pub(crate) static PROSE_CLASSES: Lazy<Classes> = Lazy::new(|| classes!("prose" "prose-invert"));
