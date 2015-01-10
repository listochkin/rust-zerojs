#![allow(unstable)]
use std::io::Command;
use std::os;

fn main() {
    let out_dir = os::getenv("OUT_DIR").unwrap();

    // 1. duktape.o
    Command::new("clang").args(&["deps/duktape/src/duktape.c", "-c", "-fPIC", "-o"])
                       .arg(format!("{}/duktape.o", out_dir))
                       .status().unwrap();
    // 2. dt.o
    Command::new("clang").args(&["-I", "deps/duktape/src", "src/dt/dt.c", "-c", "-fPIC", "-o"])
                       .arg(format!("{}/dt.o", out_dir))
                       .status().unwrap();
    // 3. libdt.a
    Command::new("ar").args(&["crus", "libdt.a", "dt.o", "duktape.o"])
                      .cwd(&Path::new(&out_dir))
                      .status().unwrap();
    // 4. configure rustc
    println!("cargo:rustc-flags=-L {} -l dt:static", out_dir);
}
