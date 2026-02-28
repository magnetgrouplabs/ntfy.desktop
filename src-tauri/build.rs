fn main() {
    // Skip icon processing for now
    println!("cargo:rustc-env=TAURI_ICON=skip");
    tauri_build::build()
}
