use std::path::{Path, PathBuf};
use std::process::Command;
use std::str::FromStr;

fn main() {
    println!("cargo:rerun-if-changed=spec/generator.py");

    let dir = env!("CARGO_MANIFEST_DIR");
    const SPECS_DIR: &str = "spec";
    const OUTPUT_FILE: &str = "src/model/generated";

    let s = PathBuf::from_str(dir);
    if let Ok(ss) = s {
        let generator_path = ss.join(SPECS_DIR);
        let output_path = ss.join(Path::new(OUTPUT_FILE));

        let status = if cfg!(target_os = "windows") {
            Command::new("python")
                .current_dir(generator_path.as_os_str())
                .arg("generator.py")
                .arg(generator_path.as_os_str()) // where to look for spec files
                .arg(output_path.as_os_str()) // where to write generated .rs
                .status()
                .expect("failed to execute process")
        } else {
            Command::new("sh")
                .current_dir(generator_path.as_os_str())
                .arg("generator.py")
                .arg(generator_path.as_os_str())
                .arg(output_path.as_os_str())
                .status()
                .expect("failed to execute process")
        };

        if let Some(status_code) = status.code() {
            if status_code != 0 {
                panic!("codegen failed with status {}", status_code);
            }
        }
    }
}
