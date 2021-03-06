extern crate rustc_version;
use rustc_version::{version_meta, Channel};

fn main() {
    match version_meta().channel {
        Channel::Nightly => {
            println!("cargo:rustc-cfg=use_asm");
        },
        _ => (),
    }
}
