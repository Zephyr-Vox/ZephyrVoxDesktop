use gpui::*;
use gpui_component::*;

use crate::views::home::Home;

/// Builds the window root with the application background.
pub fn root_layout(view: Entity<Home>, window: &mut Window, cx: &mut Context<Root>) -> Root {
    Root::new(view, window, cx).bg(cx.theme().background)
}
