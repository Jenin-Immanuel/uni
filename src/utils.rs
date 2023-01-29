use std::process::Command;
use crate::Package::{self, Npm, Yarn, Pnpm};



#[cfg(target_os = "windows")]
pub fn exec(args: Vec<String>, p: Package) {
    let mut mut_args = args.clone();
    let a = build_args(&mut mut_args, p);
    let mut a = Command::new("cmd").args(&a).spawn().expect("Nothin");
    match a.wait() {
        Ok(_) => (),
        Err(_) => {
            println!("Something wrong")
        },
    };
}

#[cfg(target_os = "windows")]
pub fn build_args(args: &mut Vec<String>, p: Package) -> Vec<String> {
    let installs = ["i", "install", "add"];
    let remove = ["remove", "rm"]; 

    let mut a = args.clone();
    let ch = a[0].to_string();
    a.insert(0, String::from("/c"));
    
    match p {
        Npm => {
            a.insert(1, String::from("npm"));
            // Installs
            if installs.contains(&&ch[..]) {
                a.insert(2, String::from("install"));
            }
            // Removes
            if remove.contains(&&ch[..]) {
                a.insert(2, String::from("remove"));
            }
            a.remove(3);
        },
        Yarn => {
            a.insert(1, String::from("yarn"));
            
            // Installs
            if installs.contains(&&ch[..]) {
                a.insert(2, String::from("add"));

                // Support --save-dev in yarn
                if a.contains(&String::from("--save-dev")) {
                    let mut a_iter = a.to_owned().into_iter();
                    let i = a_iter
                                .position(|x| x == String::from("--save-dev"));
                        
                    let i = match i {
                        Some(a) => a,
                        None => {
                            crate::errors::display_err_msg("Save dev not found");
                            std::process::exit(0);
                        },
                    };

                    a.insert(i, String::from("-D"));
                    a.remove(i + 1);
                }
                print!("Here\n");
            }
            // Removes
            if remove.contains(&&ch[..]) {
                a.insert(2, String::from("remove"));
            }
            a.remove(3);
            println!("{:?}", a);
        },
        Pnpm => {
            a.insert(1, String::from("pnpm"));
            // Installs
            if installs.contains(&&ch[..]) {
                a.insert(2, String::from("install"));
            }
            // Removes
            if remove.contains(&&ch[..]) {
                a.insert(2, String::from("remove"));
            }
            a.remove(3);
        },
        _ => {
            panic!("Invalid input");
        }
    }
    println!("From windows");
    a
}


#[cfg(target_os = "linux")]
pub fn build_args(args: Vec<String>) -> Vec<String> {
    println!("From linux");
    println!("Implementation pending");
    args
}

#[cfg(target_os = "windows")]
pub fn command(args: Vec<String>, p: crate::Package) {
    let mut mut_args = args.clone();
    let a = build_args(&mut mut_args, p);
    let mut a = Command::new("cmd").args(&a).spawn().expect("Nothin");
    match a.wait() {
        Ok(_) => (),
        Err(_) => {
            println!("Something wrong")
        },
    };
}

pub fn exit_safetly() {
    std::process::exit(0);
}
