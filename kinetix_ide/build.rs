use std::env;
use std::path::Path;

fn main() {
    // Re-run this build script if assets change
    println!("cargo:rerun-if-changed=assets/");
    println!("cargo:rerun-if-changed=build.rs");

    let out_dir = env::var("OUT_DIR").unwrap();
    let dest = Path::new(&out_dir);

    // Emit version info
    let version = env!("CARGO_PKG_VERSION");
    println!("cargo:rustc-env=KINETIX_VERSION={version}");

    // Platform-specific window icon embedding on Windows
    #[cfg(target_os = "windows")]
    {
        let mut res = winres::WindowsResource::new();
        if Path::new("assets/icon.ico").exists() {
            res.set_icon("assets/icon.ico");
        }
        res.set("ProductName", "Kinetix IDE");
        res.set("FileDescription", "Kinetix Language IDE");
        res.set("LegalCopyright", "Copyright (C) 2026 Kinetix Team");
        if let Err(e) = res.compile() {
            eprintln!("Warning: could not compile Windows resources: {e}");
        }
    }

    println!("cargo:rustc-env=KINETIX_OUT_DIR={}", dest.display());
}
