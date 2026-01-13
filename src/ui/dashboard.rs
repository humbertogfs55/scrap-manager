use gtk4::prelude::*;
use gtk4::{Box as GtkBox, Button, Orientation, ScrolledWindow};

pub fn build_dashboard() -> GtkBox {
    let root = GtkBox::new(Orientation::Horizontal, 0);

    // LEFT: actions panel
    let actions = GtkBox::new(Orientation::Vertical, 12);
    actions.set_margin_top(12);
    actions.set_margin_bottom(12);
    actions.set_margin_start(12);
    actions.set_margin_end(12);

    actions.set_width_request(220);

    let buy_btn = Button::with_label("Buy");
    let sell_btn = Button::with_label("Sell");

    actions.append(&buy_btn);
    actions.append(&sell_btn);

    // RIGHT: transactions list placeholder
    let list_container = GtkBox::new(Orientation::Vertical, 0);
    actions.set_margin_top(12);
    actions.set_margin_bottom(12);
    actions.set_margin_start(12);
    actions.set_margin_end(12);

    let scroller = ScrolledWindow::builder()
        .child(&list_container)
        .vexpand(true)
        .hexpand(true)
        .build();

    root.append(&actions);
    root.append(&scroller);

    root
}
