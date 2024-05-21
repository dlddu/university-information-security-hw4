use std::{env, fs};
use std::path::{PathBuf, Path};

fn main() {
    // Tell cargo to look for shared libraries in the specified directory
    let libdir_path = PathBuf::from("vendor/lib")
        .canonicalize()
        .expect("cannot canonicalize vendor path");
    println!("cargo:rustc-link-search={}", libdir_path.to_str().unwrap());
    println!("cargo:rustc-link-search={}", env::var("OUT_DIR").unwrap());

    // If on Linux or MacOS, tell the linker where the shared libraries are
    // on runtime (i.e. LD_LIBRARY_PATH)
    match target_and_arch() {
        (Target::Linux, _) | (Target::MacOS, _) => {
            println!("cargo:rustc-link-arg=-Wl,-rpath,{}",
                     env::var("OUT_DIR").unwrap());
        }
    }

    // Tell cargo to link against the shared library for the specific platform.
    // IMPORTANT: On macOS and Linux the shared library must be linked without 
    // the "lib" prefix and the ".so" suffix. On Windows the ".dll" suffix must 
    // be omitted.
    match target_and_arch() {
        (Target::Linux, Arch::X86_64) => {
            println!("cargo:rustc-link-lib=darknet_predict");
            copy_dylib_to_target_dir("libdarknet_predict.so");
        }
        (Target::Linux, Arch::AARCH64) => {
            println!("cargo:rustc-link-lib=darknet_predict");
            copy_dylib_to_target_dir("libdarknet_predict.so");
        }
        (Target::MacOS, Arch::X86_64) => {
            println!("cargo:rustc-link-lib=darknet_predict");
            copy_dylib_to_target_dir("libdarknet_predict.dylib");
        }
        (Target::MacOS, Arch::AARCH64) => {
            println!("cargo:rustc-link-lib=darknet_predict");
            copy_dylib_to_target_dir("libdarknet_predict.dylib");
        }
    }

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        .clang_arg("-v")
        .derive_debug(true)
        .derive_default(true)
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

fn copy_dylib_to_target_dir(dylib: &str) {
    let out_dir = env::var("OUT_DIR").unwrap();
    let src = Path::new("vendor/lib");
    let dst = Path::new(&out_dir);
    let _ = fs::copy(src.join(dylib), dst.join(dylib));
}

enum Target {
    Linux,
    MacOS
}

enum Arch {
    X86_64,
    AARCH64,
}

fn target_and_arch() -> (Target, Arch) {
    let os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    let arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    match (os.as_str(), arch.as_str()) {
        // Linux targets
        ("linux", "x86_64") => (Target::Linux, Arch::X86_64),
        ("linux", "aarch64") => (Target::Linux, Arch::AARCH64),
        // MacOS targets
        ("macos", "x86_64") => (Target::MacOS, Arch::X86_64),
        ("macos", "aarch64") => (Target::MacOS, Arch::AARCH64),
        _ => panic!("Unsupported operating system {} and architecture {}", os, arch),
    }
}
