use std::collections::BTreeMap;

use maud::{html, Markup, Render};

use super::{Description, StatusIndicator};

#[derive(Debug, Clone, derive_more::IntoIterator)]
pub(crate) struct Legend(BTreeMap<StatusIndicator, Description>);

impl FromIterator<(StatusIndicator, Description)> for Legend {
    fn from_iter<T: IntoIterator<Item = (StatusIndicator, Description)>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl Render for Legend {
    fn render(&self) -> Markup {
        html! {
            "Legend:"
            dl class=(classes!("grid", "grid-cols-[auto_auto_1fr]")) {
                @for (indicator, description) in &self.0 {
                    dt class=(classes!("text-2xl")) { (indicator) }
                    "\u{00A0}—\u{00A0}"
                    dd { (description) }
                }
            }
        }
    }
}
