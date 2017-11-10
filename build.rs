use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::Command;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let fractal_res = env::var("FRACTAL_RES").unwrap_or(String::from("res"));
    let dest_path = Path::new(&out_dir).join("config.rs");
    let mut f = File::create(&dest_path).unwrap();

    let code = format!("
        mod config {{
            pub fn datadir(res: &str) -> String {{
                let out = String::from(\"{}/\");
                out + res
            }}
        }}
    ", fractal_res);

    f.write_all(code.as_bytes()).unwrap();

    // Compile Gresource
    Command::new("glib-compile-resources")
        .args(&["--generate", "resources.xml"])
        .current_dir("res")
        .status()
        .unwrap();
}
