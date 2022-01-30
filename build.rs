use std::path::{Path, PathBuf};
use std::process::{exit, Command};
use std::str::FromStr;

fn main() {
    //

    // let s = std::env::var('CARGO_MANIFEST_DIR');

    let dir = env!("CARGO_MANIFEST_DIR");

    let s = PathBuf::from_str(dir);
    if let Ok(ss) = s {
        let s = ss.join("spec");

        if cfg!(target_os = "windows") {
            Command::new("python")
                .current_dir(s.as_os_str())
                .arg("generator.py")
                .status()
                .expect("failed to execute process")
        } else {
            Command::new("sh")
                .current_dir(s.as_os_str())
                .arg("generator.py")
                .status()
                .expect("failed to execute process")
        };
    }
}
