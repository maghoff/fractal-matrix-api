extern crate gtk;
use self::gtk::prelude::*;

use glib;
use std::sync::{Arc, Mutex};

use app::App;

impl App {
    pub fn connect_direct_chat(&self) {
        let op = &self.op;

        let cancel = self.ui.builder
            .get_object::<gtk::Button>("cancel_direct_chat")
            .expect("Can't find cancel_direct_chat in ui file.");
        let invite = self.ui.builder
            .get_object::<gtk::Button>("direct_chat_button")
            .expect("Can't find direct_chat_button in ui file.");
        let entry = self.ui.builder
            .get_object::<gtk::Entry>("to_chat_entry")
            .expect("Can't find to_chat_entry in ui file.");
        let dialog = self.ui.builder
            .get_object::<gtk::Dialog>("direct_chat_dialog")
            .expect("Can't find direct_chat_dialog in ui file.");

        // this is used to cancel the timeout and not search for every key input. We'll wait 500ms
        // without key release event to launch the search
        let source_id: Arc<Mutex<Option<glib::source::SourceId>>> = Arc::new(Mutex::new(None));
        entry.connect_key_release_event(clone!(op => move |entry, _| {
            {
                let mut id = source_id.lock().unwrap();
                if let Some(sid) = id.take() {
                    glib::source::source_remove(sid);
                }
            }

            let sid = gtk::timeout_add(500, clone!(op, entry, source_id => move || {
                op.lock().unwrap().search_invite_user(entry.get_text());
                *(source_id.lock().unwrap()) = None;
                gtk::Continue(false)
            }));

            *(source_id.lock().unwrap()) = Some(sid);
            glib::signal::Inhibit(false)
        }));

        dialog.connect_delete_event(clone!(op => move |_, _| {
            op.lock().unwrap().close_direct_chat_dialog();
            glib::signal::Inhibit(true)
        }));
        cancel.connect_clicked(clone!(op => move |_| {
            op.lock().unwrap().close_direct_chat_dialog();
        }));
        invite.set_sensitive(false);
        invite.connect_clicked(clone!(op => move |_| {
            op.lock().unwrap().start_chat();
        }));
    }
}
