extern crate gtk;
use self::gtk::prelude::*;

use appop::AppOp;
use globals;


impl AppOp {
    pub fn about_dialog(&self) {
        let window: gtk::ApplicationWindow = self.ui.builder
            .get_object("main_window")
            .expect("Can't find main_window in ui file.");

        let dialog = gtk::AboutDialog::new();
        dialog.set_logo_icon_name(globals::APP_ID);
        dialog.set_comments("A Matrix.org client for GNOME");
        dialog.set_copyright("© 2017–2018 Daniel García Moreno, et al.");
        dialog.set_license_type(gtk::License::Gpl30);
        dialog.set_modal(true);
        dialog.set_version(env!("CARGO_PKG_VERSION"));
        dialog.set_program_name("Fractal");
        dialog.set_website("https://wiki.gnome.org/Fractal");
        dialog.set_website_label("Learn more about Fractal");
        dialog.set_transient_for(&window);

        dialog.set_artists(&[
            "Tobias Bernard",
        ]);

        dialog.set_authors(&[
            "Daniel García Moreno",
            "Jordan Petridis",
            "Alexandre Franke",
            "Saurav Sachidanand",
            "Julian Sparber",
        ]);

        dialog.add_credit_section("Name by", &["Regina Bíró"]);

        dialog.show();
    }
}
