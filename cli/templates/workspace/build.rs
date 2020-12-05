fn main() {
    println!("cargo:rustc-link-search={{ dependency_path.display() }}");
}
