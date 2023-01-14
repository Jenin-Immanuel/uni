
pub fn help_str<'a>() -> &'a str {
    r#"Hello"#
}

pub fn help() {
    println!("Hello");
}

pub fn no_pack() {
    let msg = format!(r#"No Package lock file found

Run 'uni init' to create the package of your choice
1. npm
2. yarn
3. pnpm

{}
    "#, help_str());
    println!("{}", msg);
}