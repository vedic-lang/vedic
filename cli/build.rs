#[cfg(windows)]
use std::env;

#[cfg(windows)]
use winres;

#[cfg(windows)]
fn main() {
    let desc = env::var("CARGO_PKG_DESCRIPTION").unwrap();
    let version = env::var("CARGO_PKG_VERSION").unwrap();
    let authour = env::var("CARGO_PKG_AUTHORS").unwrap();
    let mut res = winres::WindowsResource::new();

    res.set("FileDescription", &desc);
    res.set("FileVersion", &version);
    res.set("ProductName", "vedic");
    res.set("ProductVersion", &version);
    res.set("CompanyName", &authour);
    res.set("LegalCopyright", "Copyright (C) 2022");
    res.set("OriginalFilename", "vedic.exe");
    res.set_icon("../assets/vedic-lang.ico");
    res.compile()
        .expect("Failed to run the Windows resource compiler (rc.exe)");
}

#[cfg(not(windows))]
fn main() {}
