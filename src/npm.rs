use std::process::Command;

pub fn script(args: Vec<String>) {
    let mut a = args.clone();
    a.insert(0, String::from("/c"));
    a.insert(1, String::from("npm"));
    
    println!("{:?}", a);
    // Build args
    // let a: [&str; n + 1];
    // a[0] = "/c";
    // let mut i = 1;
    // while i <= n {
    //     a[i] = &args[i - 1];
    //     i += 1;
    // }

    // ["/c", "npm", "help"]
    let mut a = Command::new("cmd").args(&a).spawn().expect("Nothin");
	match a.wait() {
		Ok(_) => (),
		Err(_) => {
			println!("Something wrong")
		},
	};
}