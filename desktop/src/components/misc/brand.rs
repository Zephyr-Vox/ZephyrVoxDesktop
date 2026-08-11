use gpui::*;
use rust_i18n::t;

/// Brand title used by the home page.
pub fn brand_title() -> impl IntoElement {
    div().child(t!("app.title"))
}
