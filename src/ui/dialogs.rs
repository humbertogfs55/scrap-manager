use gtk4::prelude::*;
use gtk4::{ButtonsType, MessageDialog, MessageType, Window};

pub fn show_error(parent: &Window, message: &str) {
    let dialog = MessageDialog::new(
        Some(parent),
        gtk4::DialogFlags::MODAL,
        MessageType::Error,
        ButtonsType::Close,
        message,
    );

    dialog.connect_response(|d, _| d.close());
    dialog.show();
}
