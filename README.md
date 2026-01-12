src/
 ├── main.rs          
 ├── app.rs           
 ├── db/
 │    ├── mod.rs
 │    ├── schema.rs
 │    └── queries.rs
 ├── domain/
 │    ├── mod.rs
 │    ├── transaction.rs
 │    └── inventory.rs
 ├── ui/
 │    ├── mod.rs
 │    ├── dashboard.rs
 │    ├── buy.rs
 │    ├── sell.rs
 │    └── dialogs.rs
 └── util/
      ├── mod.rs
      ├── paths.rs
      └── logging.rs


# gtk4 error wrapper using anyhow canonical: 

´´´rust

use anyhow::Result;

fn on_action(parent: &gtk4::ApplicationWindow) {
    if let Err(err) = try_on_action(parent) {
        log::error!("{err:#}");
        crate::ui::dialogs::show_error(parent, "Operation failed");
    }
}

fn try_on_action(_parent: &gtk4::ApplicationWindow) -> Result<()> {
    // real logic goes here
    Ok(())
}

´´´

# log mechanism using simplelog

log::info!("opened database");
log::warn!("invalid user input: {}", reason);
log::error!("{err:#}");

___
