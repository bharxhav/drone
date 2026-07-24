fn main() {
    println!("cargo::rerun-if-env-changed=DRONE_RELEASE_DATE");

    if std::env::var_os("DRONE_RELEASE_DATE").is_none() {
        println!("cargo::rustc-env=DRONE_RELEASE_DATE=UNRELEASED");
    }
}
