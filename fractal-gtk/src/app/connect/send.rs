extern crate gtk;
use self::gtk::prelude::*;

use app::App;

impl App {
    pub fn connect_send(&self) {
        let msg_entry: gtk::Entry = self.ui.builder
            .get_object("msg_entry")
            .expect("Couldn't find msg_entry in ui file.");

        let mut op = self.op.clone();
        msg_entry.connect_activate(move |entry| if let Some(text) = entry.get_text() {
            let mut mut_text = text;
            op.lock().unwrap().send_message(mut_text);
            entry.set_text("");
        });

        op = self.op.clone();
        msg_entry.connect_paste_clipboard(move |_| {
            op.lock().unwrap().paste();
        });
    }
}
