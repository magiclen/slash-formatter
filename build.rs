fn main() {
    if cfg!(windows) {
        println!("cargo:rustc-cfg=from_windows");
    }
    if cfg!(unix) {
        println!("cargo:rustc-cfg=from_unix");
    }
}
