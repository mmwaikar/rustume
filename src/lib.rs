pub mod app;
pub mod resume;

pub fn configure_application(cx: &mut gpui_kit::App) {
    gpui_kit::init(cx);
    cx.text_system()
        .add_fonts(vec![std::borrow::Cow::Borrowed(include_bytes!(
            "../assets/Inter-Regular.otf",
        ))])
        .expect("failed to load bundled Inter font");
    gpui_kit::component::Theme::global_mut(cx).font_family = "Inter".into();
    gpui_kit::component::Theme::global_mut(cx).mono_font_family = "Inter".into();
    gpui_kit::component::Theme::sync_base(cx);
}

#[cfg(target_arch = "wasm32")]
use std::cell::RefCell;

#[cfg(target_arch = "wasm32")]
thread_local! {
    static WEB_APPLICATION: RefCell<Option<gpui_kit::ApplicationHandle>> = const { RefCell::new(None) };
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn start_web() {
    gpui_kit::platform::web_init();
    let application = gpui_kit::application()
        .with_assets(gpui_kit::assets::Assets::new(""))
        .run_embedded(|cx| {
            configure_application(cx);
            let options = gpui_kit::WindowOptions {
                window_bounds: Some(gpui_kit::WindowBounds::Maximized(
                    gpui_kit::Bounds::default(),
                )),
                ..Default::default()
            };
            cx.open_window(options, |window, cx| app::root_view(window, cx))
                .expect("failed to open rustume web window");
        });
    WEB_APPLICATION.with(|slot| *slot.borrow_mut() = Some(application));
}
