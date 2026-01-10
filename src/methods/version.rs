pub fn get_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

pub fn version() {
    println!("ruetta - v{}", get_version());
}
