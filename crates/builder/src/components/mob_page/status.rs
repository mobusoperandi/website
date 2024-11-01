use maud::{html, Markup, Render};

use crate::mob;

pub(super) struct Status(mob::Status);
impl Status {
    pub(crate) fn new(status: mob::Status) -> Self {
        Self(status)
    }
}

impl Render for Status {
    fn render(&self) -> maud::Markup {
        type WrapperFn = fn(&str) -> Markup;

        fn status_wrapper_false(content: &str) -> Markup {
            html!(s class=(classes!("opacity-70")) { (content) })
        }

        fn status_wrapper_true(content: &str) -> Markup {
            html!(span { (content) })
        }

        let (short_wrapper, open_wrapper, full_wrapper, public_wrapper, terminated_wrapper): (
            WrapperFn,
            WrapperFn,
            WrapperFn,
            WrapperFn,
            WrapperFn,
        ) = match self.0 {
            mob::Status::Short(_) => (
                status_wrapper_true,
                status_wrapper_false,
                status_wrapper_false,
                status_wrapper_false,
                status_wrapper_false,
            ),
            mob::Status::Open(_) => (
                status_wrapper_false,
                status_wrapper_true,
                status_wrapper_false,
                status_wrapper_false,
                status_wrapper_false,
            ),
            mob::Status::Full(_) => (
                status_wrapper_false,
                status_wrapper_false,
                status_wrapper_true,
                status_wrapper_false,
                status_wrapper_false,
            ),
            mob::Status::Public(_) => (
                status_wrapper_false,
                status_wrapper_false,
                status_wrapper_false,
                status_wrapper_true,
                status_wrapper_false,
            ),
            mob::Status::Renamed(_) => {
                unreachable!()
            }
            mob::Status::Terminated(_) => (
                status_wrapper_false,
                status_wrapper_false,
                status_wrapper_false,
                status_wrapper_false,
                status_wrapper_true,
            ),
        };

        html! {
            div class=(classes!("flex", "flex-col", "items-center", "gap-1", "text-lg")) {
                div class=(classes!("flex", "gap-4", "uppercase", "tracking-widest")) {
                    (short_wrapper("short")) (open_wrapper("open")) (full_wrapper("full")) (public_wrapper("public")) (terminated_wrapper("terminated"))
                }
                p class="tracking-wide" { (mob::Status::description(self.0.as_ref())) }
            }
        }
    }
}
