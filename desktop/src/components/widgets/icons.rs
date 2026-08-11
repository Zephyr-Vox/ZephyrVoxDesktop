use gpui::*;
use gpui_component::*;

/// Icon pair shown on the home page.
pub fn app_icons() -> impl IntoElement {
    div()
        .flex()
        .gap_2()
        .items_center()
        .child(Icon::new(IconName::Inbox))
        .child(Icon::new(IconName::Bot))
}
