use std::env;
use std::path::Path;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=wokwi-chip.json");

    let manifest_dir_string = env::var("CARGO_MANIFEST_DIR").unwrap();
    let build_type = env::var("PROFILE").unwrap();
    let output_path = PathBuf::from(
        Path::new(&manifest_dir_string)
            .join("target")
            .join("wasm32-unknown-unknown")
            .join(build_type),
    );

    let json_name = format!("{}.json", env::var("CARGO_PKG_NAME").unwrap());
    let input_file = Path::new(&manifest_dir_string).join("wokwi-chip.json");
    let output_file = Path::new(&output_path).join(json_name);
    let res = std::fs::copy(input_file, output_file);
    // If failed to copy, print the error and exit with error code
    if let Err(e) = res {
        println!("cargo:warning=Failed to copy wokwi-chips.json: {}", e);
        std::process::exit(1);
    }
}
