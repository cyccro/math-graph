use gtk::{gdk::Display, prelude::*, Box, CssProvider, DrawingArea, Orientation};
use gtk4 as gtk;
use inputs::Inputs;
mod draw;
mod helpers;
mod inputs;
mod math;
mod rgb;
mod vec2;
fn init_window(
    app: &gtk::Application,
    name: &str,
    width: i32,
    height: i32,
) -> gtk::ApplicationWindow {
    let window = gtk::ApplicationWindow::new(app);
    window.set_title(Some(name));
    window.set_default_size(width, height);
    window
}

fn initialize_css() {
    let provider = CssProvider::new();
    provider.load_from_path("./styles.css");
    let display = Display::default().unwrap();
    gtk4::style_context_add_provider_for_display(
        &display,
        &provider,
        gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

fn main() -> gtk::glib::ExitCode {
    let app = gtk::Application::builder()
        .application_id("com.cyccro.DesmosCopy") //github
        .build();
    app.connect_activate(|app| {
        let window = init_window(app, "Desmos copy", 800, 600);
        initialize_css();

        let b = Box::new(Orientation::Horizontal, 0);
        let draw = {
            let drawing = DrawingArea::new();
            drawing.set_vexpand(true);
            drawing.set_hexpand(true);
            drawing.set_draw_func(|_area, ctx, w, h| {
                println!("{w} {h}");
                if let Err(e) = draw::start_draw(ctx, w, h) {
                    panic!("{e}");
                };
            });
            drawing
        };
        b.append(&Inputs::new(3, Orientation::Vertical).gtk());
        b.append(&draw);
        window.set_child(Some(&b));
        window.show();
    });
    app.run()
}
