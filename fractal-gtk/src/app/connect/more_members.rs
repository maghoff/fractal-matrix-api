extern crate gtk;
use self::gtk::prelude::*;

use app::App;

impl App {
    pub fn connect_more_members_btn(&self) {
        let mlist: gtk::ListBox = self.ui.builder
            .get_object("member_list")
            .expect("Couldn't find member_list in ui file.");

        let btn = self.op.lock().unwrap().more_members_btn.clone();
        btn.show();
        let op = self.op.clone();
        btn.connect_clicked(move |_| {
            op.lock().unwrap().member_limit += 50;
            op.lock().unwrap().show_all_members();
        });
        mlist.add(&btn);
    }
}
