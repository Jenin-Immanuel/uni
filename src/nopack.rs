use std::io::Write;

use crate::Package::{self, Npm, Pnpm, Yarn, NoPack};

use colored::Colorize;
use inquire::{Select, InquireError};

pub fn pjson_present(args: Vec<String>) {
    if check_instruction_commands(&args[0]) {
        if &args[0] == "init" {
            let msg = r#"Package.json is already present"#;
            println!("\n{}", msg.blue());
        }
        else {
            let ans = ask_package();
            let ans = return_enum_package(ans);
            crate::utils::exec(args, ans);
        }
    }
    else {
        let ans = ask_package();
        let ans = return_enum_package(ans);
        crate::utils::exec(args, ans);
    }
}

pub fn pjson_not_present(args: Vec<String>) {

    // Special cases
    if check_instruction_commands(&args[0]) {
        // Self implementation
        if &args[0] == "init" {
            let content = r#"{
    "name": "test",
    "version": "1.0.0",
    "description": "",
    "main": "index.js",
    "scripts": {
        "test": "echo \"Error: no test specified\" && exit 1"
    },
    "author": "",
    "license": "ISC"
}"#;
            let mut fop = std::fs::File::create("package.json").expect("Failed to create");
            fop.write_all(content.as_bytes()).expect("Failed to write");
        }
        // GLobal installation
        else if args.contains(&String::from("-g")){

            let ans = ask_package();
            let ans = return_enum_package(ans);
            crate::utils::exec(args, ans);
        }
        else {
            crate::errors::display_err_msg("Package.json file not present");
        }
    }
    else {
        let ans = ask_package();
        let ans = return_enum_package(ans);
        crate::utils::exec(args, ans);
    }
}


fn ask_package<'a>() -> &'a str {
    let ops = vec!["npm", "yarn", "pnpm"];
    let ans: Result<&str, InquireError> = Select::new("Select the package to use:", ops).prompt();
    let ans = match ans {
        Ok(c) => c,
        Err(_) => {
            crate::errors::display_err_msg("Invalid input");
            std::process::exit(0)
        },
    };
    ans
}

fn return_enum_package(p: &str) -> Package {
    if p == "npm" {
        return Npm
    }
    else if p == "yarn" {
        return Yarn
    }
    else if p == "pnpm" {
        return Pnpm
    }
    else {
        NoPack
    }
}

// pub fn check_command(args: Vec<String>) {
    
//     if check_instruction_commands(&args[0]) {
//         let pack = match ask_package() {
//             Ok(s) => s,
//             Err(e) => {
//                 panic!("{}", e)
//             },
//         };
//         println!("{:?}", pack);
//     }   
// }

fn check_instruction_commands(ins: &str) -> bool {
    let ins_set = [
        "init",
        "i",
        "install",
        "add",
        "remove",
        "rm",
    ];
    
    if ins_set.contains(&ins) {
        return true
    }
    false
}



// #[warn(unused_variables)]
// #[cfg(target_os = "windows")]
// fn execute(args: Vec<String>) {

// }

// #[cfg(target_os = "linux")]
// fn execute(args: Vec<String>) {

// }


