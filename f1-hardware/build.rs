fn main() {
    if cfg!(feature = "esp32c3") {
        println!("cargo::rustc-link-arg=-Trom_coexist.x");
        println!("cargo::rustc-link-arg=-Trom_functions.x");
        println!("cargo::rustc-link-arg=-Trom_phy.x");
    }
}