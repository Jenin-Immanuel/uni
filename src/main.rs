use uni;
use std::env;

use uni::Package::{Npm, Pnpm, Yarn, NoPack};
// use uni::Instruction::{ValidCommand, Script, InvalidCommand};

fn main() {
    let args: Vec<String> = env::args().collect();
    
    
    match args.len() {
        1 => {
            uni::help_menu::help();        
        },
        _ => {
            let package = uni::parser::parse(args[1..].to_owned());
            match package {
                Npm => {

                },
                Yarn => {

                },
                Pnpm => {

                },
                NoPack => {
                    let args = args[1..].to_owned();
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
            // match ins {
            //     Script => {
            //         match package {
            //             Npm => {
            //                 uni::npm::script(args[1..].to_owned());
            //             },
            //             Yarn => {
                
            //             },
            //             Pnpm => {
                
            //             },
            //             NoPack => {
                            
            //             }
            //         };

            //     },
            //     InvalidCommand => {
            //         match package {
            //             Npm => {
            //                 uni::npm::command(args[1..].to_owned());
            //             },
            //             Yarn => {
                
            //             },
            //             Pnpm => {
                
            //             },
            //             NoPack => {
            //                 uni::nopack::check_command(args[1..].to_owned());
            //             }
            //         };
            //     },
            //     ValidCommand => {
            //         match package {
            //             Npm => {
            //                 uni::npm::command(args[1..].to_owned());
            //             },
            //             Yarn => {
                
            //             },
            //             Pnpm => {
                
            //             },
            //             NoPack => {
            //                 uni::nopack::check_command(args[1..].to_owned());
            //             }
            //         };

            //     },
            // };
        }
    }
    println!("{:?}", args);
}
