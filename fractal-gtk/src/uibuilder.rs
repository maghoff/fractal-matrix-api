extern crate gtk;
use uibuilder::gtk::BuilderExt;


#[derive(Clone)]
pub struct UI {
    pub builder: gtk::Builder,
}

impl UI {
    pub fn new() -> UI {

        let builder = gtk::Builder::new();
        builder.add_from_resource("/org/gnome/Fractal/ui/user_menu.ui");
        builder.add_from_resource("/org/gnome/Fractal/ui/main_window.ui");

        builder.add_from_resource("/org/gnome/Fractal/ui/add_room_menu.ui");
        builder.add_from_resource("/org/gnome/Fractal/ui/autocomplete.ui");
        builder.add_from_resource("/org/gnome/Fractal/ui/direct_chat.ui");
        builder.add_from_resource("/org/gnome/Fractal/ui/filechooser.ui");
        builder.add_from_resource("/org/gnome/Fractal/ui/invite.ui");
        builder.add_from_resource("/org/gnome/Fractal/ui/invite_user.ui");
        builder.add_from_resource("/org/gnome/Fractal/ui/join_room.ui");
        builder.add_from_resource("/org/gnome/Fractal/ui/leave_room.ui");
        builder.add_from_resource("/org/gnome/Fractal/ui/members.ui");
        builder.add_from_resource("/org/gnome/Fractal/ui/new_room.ui");
        builder.add_from_resource("/org/gnome/Fractal/ui/room_config.ui");
        builder.add_from_resource("/org/gnome/Fractal/ui/room_menu.ui");

        UI { builder }
    }
}
