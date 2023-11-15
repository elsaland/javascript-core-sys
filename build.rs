use std::env;
use std::process::{Command, Stdio};
use std::path::PathBuf;

#[cfg(unix)]
fn main() {
    // "Tools/Scripts/build-webkit --jsc-only --cmakeargs="-DENABLE_STATIC_JSC=ON -DUSE_THIN_ARCHIVES=OFF"
    let mut build = Command::new("Tools/Scripts/build-webkit");

    build
        .arg("--jsc-only")
        .arg("--cmakeargs=\"-DENABLE_STATIC_JSC=ON -DUSE_THIN_ARCHIVES=OFF\"")
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .current_dir("WebKit");

    if cfg!(debug_assertions) {
        build.arg("--debug");
    }

    let build = build.status()
        .expect("Failed to build WebKit");

    assert!(build.success());
    let root = env::var("CARGO_MANIFEST_DIR").unwrap();
    println!("cargo:rustc-link-search=native={}/WebKit/WebKitBuild/JSCOnly/Release/lib", root);
    println!("cargo:rustc-link-lib=static=JavaScriptCore");

    let header = format!("{}/WebKit/WebKitBuild/JSCOnly/Release/JavaScriptCore/Headers/JavaScriptCore/JavaScript.h", root);
    let bindings = bindgen::Builder::default()
        .header(header)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(root);
    bindings
        .write_to_file(out_path.join("src/bindings.rs"))
        .expect("Couldn't write bindings!");
}