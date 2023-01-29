use crate::Package;


pub fn package() -> Package {
    crate::render::checker()
}


pub fn check_package_json() -> bool {
    std::path::Path::new("package.json").exists()
}
