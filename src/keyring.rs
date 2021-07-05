extern crate cryptex;

use cryptex::{KeyRing};

#[cfg(target_os = "linux")]
use cryptex::keyring::linux::LinuxOsKeyRing as OsKeyRing;

pub fn test() {
    let secret_names = OsKeyRing::list_secrets()
        .unwrap_or_else(|e| panic!(e.to_string()));
    for s in secret_names {
        println!("{:?}", s);
    }
}