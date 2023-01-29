#[derive(Debug)]
pub enum Package {
    Npm,
    Pnpm,
    Yarn,
    NoPack,
}

pub enum Instruction {
    ValidCommand,
    Script,
    InvalidCommand,
}


pub mod help_menu;

pub mod parser;

pub mod render;


pub mod npm;

pub mod nopack;

pub mod utils;

pub mod errors {
    use colored::Colorize;

    pub fn display_err_msg(msg: &str) {
        display_msg_helper(msg);
    }
    fn display_msg_helper(msg: &str) {
        println!("\n\nError: {}", msg.red());
        crate::utils::exit_safetly();
    }
}