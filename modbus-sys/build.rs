
extern crate pkg_config;
extern crate gcc;

use std::env;
use std::fs;
use std::path::{PathBuf, Path};
use std::process::Command;
use std::io::ErrorKind;


pub fn main() {

    let target = env::var("TARGET").unwrap();

    
    match pkg_config::find_library("libmodbus") {
        Ok(lib) => {
            for path in lib.include_paths.iter() {
                println!("cargo:include={}", path.display());
            }
            return
        }
        Err(e) => println!("Couldn't find libmodbus from \
                            pkgconfig ({:?}), compiling it from source...", e),
    }

    if !Path::new("modbus/.git").exists() {
        let _ = Command::new("git").args(&["submodule", "update", "--init"])
                                        .status();
    }

    /*
    println!("cargo:rustc-link-search={}/lib", dst.display());
    println!("cargo:rustc-link-lib=static=curl");
    println!("cargo:root={}", dst.display());
    println!("cargo:include={}/include", dst.display());
    */

}
