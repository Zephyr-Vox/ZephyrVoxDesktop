use gpui::*;
use gpui_component::button::{Button, ButtonVariants};
use rust_i18n::t;

/// Primary action button for the home page.
pub fn primary_button() -> impl IntoElement {
    Button::new("hello")
        .primary()
        .label(t!("app.hello"))
        .on_click(|_, _, _| println!("Clicked!"))
}
