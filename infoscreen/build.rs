fn main() {
    if cfg!(target_os = "linux") {
        println!("cargo:rustc-link-lib=static:+whole-archive=cap");
        println!("cargo:rustc-link-lib=dylib=epd");
    }
}