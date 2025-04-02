use std::env;
use std::fs;
use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    
    // Get the output directory where we'll generate any needed files
    let _out_dir = env::var("OUT_DIR").unwrap();
    
    // Tell cargo to tell rustc to link against the system libraries if needed
    #[cfg(target_os = "linux")]
    {
        println!("cargo:rustc-link-lib=dylib=m"); // Link with the math library on Linux
    }
    
    // Create output directories for examples if they don't exist
    ensure_dirs_exist(&[
        "examples/output",
        "tests/output",
    ]);
    
    // You can generate code at build time if needed:
    // let dest_path = Path::new(&out_dir).join("generated_constants.rs");
    // fs::write(
    //     &dest_path,
    //     "pub const VERSION: &str = env!(\"CARGO_PKG_VERSION\");\n"
    // ).unwrap();
    
    // Print some useful information
    println!("cargo:warning=Building model-generator v{}", env::var("CARGO_PKG_VERSION").unwrap());
}

fn ensure_dirs_exist(dirs: &[&str]) {
    for dir in dirs {
        let path = Path::new(dir);
        if !path.exists() {
            fs::create_dir_all(path).unwrap_or_else(|e| {
                println!("cargo:warning=Failed to create directory {}: {}", dir, e);
            });
        }
    }
} 