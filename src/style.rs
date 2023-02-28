use const_format::formatcp;
use once_cell::sync::Lazy;

use crate::html::Classes;

pub(crate) const GRAYS: &str = "gray";
pub(crate) const BACKGROUND_COLOR: &str = formatcp!("{GRAYS}-900");
pub(crate) const TEXT_COLOR: &str = formatcp!("{GRAYS}-100");

pub(crate) static PROSE_CLASSES: Lazy<Classes> =
    Lazy::new(|| classes!("prose" "prose-invert" "max-w-full"));

pub(crate) static BUTTON_CLASSES: Lazy<Classes> = Lazy::new(
    || classes!("block" "p-3" "text-lg" format!("bg-{GRAYS}-700") "rounded" "no-underline" "uppercase" "flex" "items-center"),
);

pub(crate) static IDENT_CLASSES: Lazy<Classes> =
    Lazy::new(|| classes!("whitespace-nowrap" "font-mono"));

pub(crate) const BUTTON_GAP: &str = "2";
pub(crate) const VERTICAL_GAP_CLASS: &str = "gap-5";
pub(crate) const IDENT_INTENSITY: u16 = 400;

pub(crate) static FIELD_OR_VARIANT_CLASSES: Lazy<Classes> = Lazy::new(|| {
    classes!(
        format!("bg-{GRAYS}-700")
        "p-1"
        "rounded"
        "flex"
        "flex-col"
        VERTICAL_GAP_CLASS
    )
});
