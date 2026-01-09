pub fn get_version() -> String {
    format!("{}", env!("CARGO_PKG_VERSION"))
}

pub fn version() {
    println!("ruetta - v{}", get_version());
}
