extern crate gtk;
use self::gtk::prelude::*;

use app::App;

impl App {
    pub fn connect_attach(&self) {
        let attach_button: gtk::Button = self.ui.builder
            .get_object("attach_button")
            .expect("Couldn't find attach_button in ui file.");

        let op = self.op.clone();
        attach_button.connect_clicked(move |_| {
            op.lock().unwrap().attach_file();
        });
    }
}
