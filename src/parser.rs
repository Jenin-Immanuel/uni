use crate::Package::{self, Npm, Yarn, Pnpm, NoPack};
use crate::Instruction::{self};
use core::panic;
// use std::{path::Path};

use serde_json::Value as JsonValue;
// use std::process::{Command};


pub fn parse(args: Vec<String>) -> Package {
    println!("Recieved args: {:?}", args);
 
    let package: Package = crate::render::checker();

    match package {
        Npm => {
            println!("NPM package");
        },
        Yarn => {

        },
        Pnpm => {

        },
        NoPack => {
            println!("Currently no package");
            // crate::help_menu::no_pack();


            // Sample Process
        }
    };
    
    // let res = check_instruction(&args[0]);

    package
}


pub fn check_package_json() -> bool {
    std::path::Path::new("package.json").exists()
}

fn check_instruction(ins: &str) -> Instruction {
    let list = [
        "help",
        "init",
        "install",
        "i",
        "add",
        "uninstall",
        "remove",
        "view",
    ];

    if list.contains(&ins)  {
        return Instruction::ValidCommand
    }
    if is_script(&ins) {
        return Instruction::Script
    }
    Instruction::InvalidCommand
}

fn is_script(ins: &str) -> bool {
    // let f;
    // if Path::new("package.json").exists() {
    //     f = std::fs::OpenOptions::new()
    //         .read(true)
    //         .open("package.json");
    // }
    // else {
        
    // }
    let scripts = match std::fs::read_to_string("package.json") {
        Ok(s) => serde_json::from_str::<JsonValue>(&s).unwrap()["scripts"].clone(),
        Err(_) => {
            crate::errors::display_err_msg("Unable to open the file\nCheck whether the file is present");
            std::process::exit(0);
        },
    };

    for (i, _) in scripts.as_object().unwrap() {
        if ins == i {
            return true
        }
    }

    false
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_script_valid() {
        let ins = "script";
        assert_eq!(false, is_script(ins));
    }
}
