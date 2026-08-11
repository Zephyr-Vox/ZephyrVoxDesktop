use gpui::*;
use gpui_component::StyledExt;

use crate::components::misc::brand::brand_title;
use crate::components::widgets::{app_icons, primary_button};

/// Home page of the desktop application.
pub struct Home;

impl Home {
    pub fn new() -> Self {
        Self
    }
}

impl Render for Home {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
            .v_flex()
            .gap_2()
            .size_full()
            .items_center()
            .justify_center()
            .child(brand_title())
            .child(app_icons())
            .child(primary_button())
    }
}
