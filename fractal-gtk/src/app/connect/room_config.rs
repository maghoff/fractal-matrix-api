extern crate gtk;
extern crate gdk_pixbuf;
use self::gtk::prelude::*;

use glib;
use self::gdk_pixbuf::Pixbuf;

use app::App;

impl App {
    pub fn connect_room_config(&self) {
        let dialog = self.ui.builder
            .get_object::<gtk::Dialog>("room_config_dialog")
            .expect("Can't find room_config_dialog in ui file.");
        let btn = self.ui.builder
            .get_object::<gtk::Button>("room_dialog_close")
            .expect("Can't find room_dialog_close in ui file.");
        btn.connect_clicked(clone!(dialog => move |_| {
            dialog.hide();
        }));
        dialog.connect_delete_event(clone!(dialog => move |_, _| {
            dialog.hide();
            glib::signal::Inhibit(true)
        }));

        let avatar = self.ui.builder
            .get_object::<gtk::Image>("room_avatar_image")
            .expect("Can't find room_avatar_image in ui file.");
        let avatar_btn = self.ui.builder
            .get_object::<gtk::Button>("room_avatar_filechooser")
            .expect("Can't find room_avatar_filechooser in ui file.");
        let avatar_fs = self.ui.builder
            .get_object::<gtk::FileChooserDialog>("file_chooser_dialog")
            .expect("Can't find file_chooser_dialog in ui file.");

        let fs_set = self.ui.builder
            .get_object::<gtk::Button>("file_chooser_set")
            .expect("Can't find file_chooser_set in ui file.");
        let fs_cancel = self.ui.builder
            .get_object::<gtk::Button>("file_chooser_cancel")
            .expect("Can't find file_chooser_cancel in ui file.");
        let fs_preview = self.ui.builder
            .get_object::<gtk::Image>("file_chooser_preview")
            .expect("Can't find file_chooser_preview in ui file.");

        fs_cancel.connect_clicked(clone!(avatar_fs => move |_| {
            avatar_fs.hide();
        }));
        avatar_fs.connect_delete_event(move |d, _| {
            d.hide();
            glib::signal::Inhibit(true)
        });

        fs_set.connect_clicked(clone!(avatar_fs, avatar => move |_| {
            avatar_fs.hide();
            if let Some(fname) = avatar_fs.get_filename() {
                if let Some(name) = fname.to_str() {
                    if let Ok(pixbuf) = Pixbuf::new_from_file_at_size(name, 100, 100) {
                        avatar.set_from_pixbuf(&pixbuf);
                    } else {
                        avatar.set_from_icon_name("image-missing", 5);
                    }
                }
            }
        }));

        avatar_fs.connect_selection_changed(move |fs| {
            if let Some(fname) = fs.get_filename() {
                if let Some(name) = fname.to_str() {
                    if let Ok(pixbuf) = Pixbuf::new_from_file_at_size(name, 100, 100) {
                        fs_preview.set_from_pixbuf(&pixbuf);
                    }
                }
            }
        });

        avatar_btn.connect_clicked(clone!(avatar_fs => move |_| {
            avatar_fs.present();
        }));

        let btn = self.ui.builder
            .get_object::<gtk::Button>("room_dialog_set")
            .expect("Can't find room_dialog_set in ui file.");
        let op = self.op.clone();
        btn.connect_clicked(clone!(dialog => move |_| {
            op.lock().unwrap().change_room_config();
            dialog.hide();
        }));
    }
}
