use anyhow::Result;
use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Box as GtkBox, Orientation};

use crate::util::paths::AppPaths;

pub fn run(_paths: AppPaths) -> Result<()> {
    let app = Application::new(Some("local.scrap.manager"), Default::default());

    app.connect_activate(build_ui);

    app.run();
    Ok(())
}

fn build_ui(app: &Application) {
    let window = ApplicationWindow::new(app);
    window.set_title(Some("Scrap Manager"));
    window.set_default_size(900, 600);

    let root = GtkBox::new(Orientation::Horizontal, 0);
    window.set_child(Some(&root));

    window.present();
}
