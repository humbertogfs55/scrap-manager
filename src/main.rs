use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow};

fn main() {
    let app = Application::new(Some("local.scrap.manager"), Default::default());

    app.connect_activate(|app| {
        let window = ApplicationWindow::new(app);
        window.set_title(Some("Scrap Manager"));
        window.set_default_size(800, 600);
        window.show();
    });

    app.run();
}
