use gpui_kit::WindowOptions;

fn main() {
    gpui_kit::application().run(|cx| {
        gpui_kit::init(cx);
        cx.open_window(WindowOptions::default(), |window, cx| rustume::app::root_view(window, cx))
            .expect("failed to open rustume window");
    });
}
