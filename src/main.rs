use uni;
use std::env;

use uni::Package::{Npm, Pnpm, Yarn, NoPack};

fn main() {
    let args: Vec<String> = env::args().collect();
    
    
    match args.len() {
        1 => {
            uni::help_menu::help();        
        },
        _ => {
            let package = uni::parser::package();
            let args = args[1..].to_owned();
            match package {
                Npm => {
                    uni::utils::exec(args, Npm)
                },
                Yarn => {
                    uni::utils::exec(args, Yarn)
                },
                Pnpm => {
                    uni::utils::exec(args, Pnpm)
                },
                NoPack => {
                    // With package.json
                    if uni::parser::check_package_json()  {
                        uni::nopack::pjson_present(args);
                    }
                    // Without package.json
                    else {
                        uni::nopack::pjson_not_present(args);
                    }
                },
            }
        }
    }
}
