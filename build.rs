use fs::read_dir;
use std::env;
use std::fs;
use std::path::Path;

fn main () -> () {
    let out_dir = env::var("OUT_DIR").unwrap();
    let output_path = Path::new(&out_dir).join("inputs");
    if ! output_path.is_dir() {
        fs::create_dir(output_path.clone()).unwrap();
    }

    for entry in read_dir("./inputs").unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            println!("Skipping dir: {}", entry.file_name().as_os_str().to_str().unwrap());
        } else {
            let dest_path = output_path.clone().join(entry.file_name());
            fs::copy(path, dest_path).unwrap();
            println!("Copied {}", entry.file_name().as_os_str().to_str().unwrap());
        }
    }

    println!("cargo:rerun-if-changed=inputs/");
    println!("cargo:rerun-if-changed=build.rs");
}