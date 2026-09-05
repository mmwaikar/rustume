pub mod app;
pub mod resume;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn start_web() {
	gpui_kit::platform::web_init();
	gpui_kit::application().run(|cx| {
		gpui_kit::init(cx);
		cx.spawn(async move |cx| {
			cx.open_window(gpui_kit::WindowOptions::default(), |window, cx| app::root_view(window, cx))
				.expect("failed to open rustume web window");
		})
		.detach();
	});
}
