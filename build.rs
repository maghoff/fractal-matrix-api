use std::env;
use std::process::Command;

fn main() {
    // I think this is expected by Meson.
    let fractal_res = env::var("FRACTAL_RES").unwrap_or(String::from("res"));

    // Compile Gresource
    Command::new("glib-compile-resources")
        .args(&["--generate", "resources.xml"])
        .current_dir("res")
        .status()
        .unwrap();
}
