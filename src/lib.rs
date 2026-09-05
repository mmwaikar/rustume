pub mod app;
pub mod resume;

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
    let application = gpui_kit::application().run_embedded(|cx| {
        gpui_kit::init(cx);
        cx.text_system()
            .add_fonts(vec![std::borrow::Cow::Borrowed(include_bytes!(
                "../assets/IBMPlexSans-Regular.ttf",
            ))])
            .expect("failed to load bundled web font");
        gpui_kit::component::Theme::global_mut(cx).font_family = "IBM Plex Sans".into();
        gpui_kit::component::Theme::global_mut(cx).mono_font_family = "IBM Plex Sans".into();
        gpui_kit::component::Theme::sync_base(cx);
        cx.open_window(gpui_kit::WindowOptions::default(), |window, cx| {
            app::root_view(window, cx)
        })
        .expect("failed to open rustume web window");
    });
    WEB_APPLICATION.with(|slot| *slot.borrow_mut() = Some(application));
}
