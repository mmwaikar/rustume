use gpui_kit::WindowOptions;

fn main() {
    gpui_kit::application()
        .with_assets(gpui_kit::assets::Assets::new(""))
        .run(|cx| {
            rustume::configure_application(cx);
            let options = WindowOptions {
                window_bounds: Some(gpui_kit::WindowBounds::Maximized(
                    gpui_kit::Bounds::default(),
                )),
                ..Default::default()
            };
            cx.open_window(options, |window, cx| rustume::app::root_view(window, cx))
                .expect("failed to open rustume window");
        });
}
