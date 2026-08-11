rust_i18n::i18n!("locales", fallback = "en");

mod components;
mod layouts;
mod views;

use gpui::*;
use layouts::root_layout;
use views::home::Home;

fn main() -> anyhow::Result<()> {
    rust_i18n::set_locale("zh-CN");

    gpui_platform::application()
        .with_assets(gpui_component_assets::Assets)
        .run(move |cx| {
            // Extend GPUI Component translations with application locales.
            rust_i18n::extend!(gpui_component);

            // Must be called before using any GPUI Component features.
            gpui_component::init(cx);

            cx.spawn(async move |cx| {
                cx.open_window(WindowOptions::default(), |window, cx| {
                    let view = cx.new(|_| Home::new());
                    // The first level on the window should be a layout root.
                    cx.new(|cx| root_layout(view, window, cx))
                })
                .expect("Failed to open window");
            })
            .detach();
        });
    Ok(())
}
