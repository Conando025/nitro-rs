pub fn main() {
    println!("cargo::rerun-if-changed=example/arm7.ld");
    println!("cargo::rerun-if-changed=example/arm9.ld");
}
