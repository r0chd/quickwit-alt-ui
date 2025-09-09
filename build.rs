use std::env;

fn main() {
    let backend_url = env::var("QW_BACKEND_URL").unwrap_or("http://localhost:7280".to_string());

    println!("cargo:rustc-env=QW_BACKEND_URL_BUILT={}", backend_url);

    println!("cargo:rerun-if-env-changed=QW_BACKEND_URL");
}
