extern crate gtk;
use self::gtk::prelude::*;

use appop::AppState;

use app::App;

impl App {
    pub fn connect_headerbars(&self) {
        let op = self.op.clone();
        let btn = self.ui.builder
            .get_object::<gtk::Button>("back_button")
            .expect("Can't find back_button in ui file.");
        btn.connect_clicked(move |_| {
            op.lock().unwrap().set_state(AppState::Chat);
        });
    }
}
