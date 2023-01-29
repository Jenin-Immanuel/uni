use std::process::Command;

#[cfg(target_os = "windows")]
pub fn build_args(args: Vec<String>) -> Vec<String> {
    let mut a = args.clone();
    a.insert(0, String::from("/c"));
    a.insert(1, String::from("npm"));
    println!("From windows");
    a
}

#[cfg(target_os = "linux")]
pub fn build_args(args: Vec<String>) -> Vec<String> {
    println!("From linux");
    args
}


pub fn script(args: Vec<String>) {
    // let mut a = args.clone();
    
    // println!("{:?}", a);
    // Build args
    // let a: [&str; n + 1];
    // a[0] = "/c";
    // let mut i = 1;
    // while i <= n {
    //     a[i] = &args[i - 1];
    //     i += 1;
    // }

    // ["/c", "npm", "help"]
    if cfg!(target_os = "windows") {
        let a = build_args(args.clone());
        let mut a = Command::new("cmd").args(&a).spawn().expect("Nothin");
        match a.wait() {
            Ok(_) => (),
            Err(_) => {
                println!("Something wrong")
            },
        };
    } 
    else if cfg!(target_os = "linux") {
        println!("From linux");
        let mut a = Command::new("npm").args(&args).spawn().expect("Nothin");
        match a.wait() {
            Ok(_) => (),
            Err(_) => {
                println!("Something wrong")
            },
        };
    }
}

pub fn command(args: Vec<String>) {
    if cfg!(target_os = "windows") {
        let a = build_args(args.clone());
        let mut a = Command::new("cmd").args(&a).spawn().expect("Nothin");
        match a.wait() {
            Ok(_) => (),
            Err(_) => {
                println!("Something wrong")
            },
        };
    } 
    else if cfg!(target_os = "linux") {
        println!("From linux");
        let mut a = Command::new("npm").args(&args).spawn().expect("Nothin");
        match a.wait() {
            Ok(_) => (),
            Err(_) => {
                println!("Something wrong")
            },
        };
    }
}