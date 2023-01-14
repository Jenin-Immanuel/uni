use crate::Package::{self, Npm, Yarn, Pnpm, NoPack};
use crate::Instruction::{self};
use core::panic;
// use std::{path::Path};

use serde_json::Value as JsonValue;
// use std::process::{Command};


pub fn parse(args: Vec<String>) -> (Package, Instruction) {
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
            crate::help_menu::no_pack();


            // Sample Process
        }
    };
    
    let res = check_instruction(&args[0]);

    (package, res)
}


fn check_instruction(ins: &str) -> Instruction {
    let list = vec![
        "help",
        "init",
        "install",
        "i",
        "add",
        "uninstall",
        "remove",
    ];

    if list.contains(&ins)  {
        return Instruction::ValidCommand
    }
    if is_script(&ins) {
        return Instruction::Script
    }
    

    Instruction::InvalidCommand
    // Ok(ins.to_owned())
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
            panic!("Unable to open the file\nCheck whether the file is present");
        },
    };

    for (i, _) in scripts.as_object().unwrap() {
        if ins == i {
            return true
        }
    }

    false
} 