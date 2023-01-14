use uni;
use std::env;

use uni::Package::{Npm, Pnpm, Yarn, NoPack};
use uni::Instruction::{ValidCommand, Script, InvalidCommand};

fn main() {
    let args: Vec<String> = env::args().collect();
    
    
    match args.len() {
        1 => {
            uni::help_menu::help();        
        },
        _ => {
            // let _res = match uni::parser::parse(args[1..].to_owned()) {
            //     Ok(()) => "Ok",
            //     Err(_) => {
            //         std::process::exit(1);
            //     },
            // };
            let (package, ins ) = uni::parser::parse(args[1..].to_owned());
            match ins {
                Script => {
                    match package {
                        Npm => {
                            uni::npm::script(args[1..].to_owned());
                        },
                        Yarn => {
                
                        },
                        Pnpm => {
                
                        },
                        NoPack => {
                            println!("Currently no package");
                        }
                    };

                },
                InvalidCommand => {

                },
                ValidCommand => {

                },
            };
        }
    }
    println!("{:?}", args);
}
