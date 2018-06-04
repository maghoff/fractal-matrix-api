extern crate gtk;
extern crate gettextrs;

use self::gtk::prelude::*;
use self::gettextrs::gettext;

use app::App;

impl App {
    pub fn connect_directory(&self) {
        let q = self.ui.builder
            .get_object::<gtk::Entry>("directory_search_entry")
            .expect("Can't find directory_search_entry in ui file.");

        let directory_choice_label = self.ui.builder
            .get_object::<gtk::Label>("directory_choice_label")
            .expect("Can't find directory_choice_label in ui file.");

        let default_matrix_server_radio = self.ui.builder
            .get_object::<gtk::RadioButton>("default_matrix_server_radio")
            .expect("Can't find default_matrix_server_radio in ui file.");

        let other_protocol_radio = self.ui.builder
            .get_object::<gtk::RadioButton>("other_protocol_radio")
            .expect("Can't find other_protocol_radio in ui file.");

        let protocol_combo = self.ui.builder
            .get_object::<gtk::ComboBox>("protocol_combo")
            .expect("Can't find protocol_combo in ui file.");

        let protocol_model = self.ui.builder
            .get_object::<gtk::ListStore>("protocol_model")
            .expect("Can't find protocol_model in ui file.");

        let other_homeserver_radio = self.ui.builder
            .get_object::<gtk::RadioButton>("other_homeserver_radio")
            .expect("Can't find other_homeserver_radio in ui file.");

        let other_homeserver_url_entry = self.ui.builder
            .get_object::<gtk::Entry>("other_homeserver_url_entry")
            .expect("Can't find other_homeserver_url_entry in ui file.");

        let other_homeserver_url = self.ui.builder
            .get_object::<gtk::EntryBuffer>("other_homeserver_url")
            .expect("Can't find other_homeserver_url in ui file.");

        let scroll = self.ui.builder
            .get_object::<gtk::ScrolledWindow>("directory_scroll")
            .expect("Can't find directory_scroll in ui file.");

        let mut op = self.op.clone();
        scroll.connect_edge_reached(move |_, dir| if dir == gtk::PositionType::Bottom {
            op.lock().unwrap().load_more_rooms();
        });

        op = self.op.clone();
        q.connect_activate(move |_| { op.lock().unwrap().search_rooms(false); });

        default_matrix_server_radio.connect_toggled(clone!(directory_choice_label, default_matrix_server_radio, protocol_combo, other_homeserver_url_entry => move |_| {
            if default_matrix_server_radio.get_active() {
                protocol_combo.set_sensitive(false);
                other_homeserver_url_entry.set_sensitive(false);
            }

            directory_choice_label.set_text(&gettext("Default Matrix Server"));
        }));

        other_protocol_radio.connect_toggled(clone!(directory_choice_label, other_protocol_radio, protocol_combo, protocol_model, other_homeserver_url_entry => move |_| {
            if other_protocol_radio.get_active() {
                protocol_combo.set_sensitive(true);
                other_homeserver_url_entry.set_sensitive(false);
            }

            let active = protocol_combo.get_active();
            let protocol: String = match protocol_model.iter_nth_child(None, active) {
                Some(it) => {
                    let v = protocol_model.get_value(&it, 0);
                    v.get().unwrap()
                }
                None => String::from(""),
            };

            directory_choice_label.set_text(&protocol);
        }));

        protocol_combo.connect_changed(clone!(directory_choice_label, protocol_combo, protocol_model => move |_| {
            let active = protocol_combo.get_active();
            let protocol: String = match protocol_model.iter_nth_child(None, active) {
                Some(it) => {
                    let v = protocol_model.get_value(&it, 0);
                    v.get().unwrap()
                }
                None => String::from(""),
            };

            directory_choice_label.set_text(&protocol);
        }));

        other_homeserver_radio.connect_toggled(clone!(other_homeserver_radio, protocol_combo, other_homeserver_url_entry => move |_| {
            if other_homeserver_radio.get_active() {
                protocol_combo.set_sensitive(false);
                other_homeserver_url_entry.set_sensitive(true);
            }
        }));

        other_homeserver_url_entry.connect_changed(clone!(directory_choice_label, other_homeserver_url => move |_| {
            directory_choice_label.set_text(&other_homeserver_url.get_text());
        }));
    }
}
