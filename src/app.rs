use anyhow::Result;
use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow};

use crate::ui;
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

    let dashboard = ui::dashboard::build_dashboard();
    window.set_child(Some(&dashboard));

    window.present();
}
