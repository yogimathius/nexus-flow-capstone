fn main() {
    // Tell Cargo to rerun this build script if .env changes
    println!("cargo:rerun-if-changed=.env");

    // Tell Cargo to rerun the build script if any file in migrations changes
    println!("cargo:rerun-if-changed=migrations");
}
