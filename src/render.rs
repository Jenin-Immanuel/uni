use std::path::Path;

use crate::Package;

pub fn checker() -> Package {
    let a = file_check();
    a
}

fn file_check() -> Package {
    if Path::new("package-lock.json").exists() {
        return Package::Npm
    };
    if Path::new("yarn.lock").exists() {
        return Package::Yarn
    };
    if Path::new("pnpm-lock.yml").exists() {
        return Package::Pnpm
    };
    Package::NoPack
}

