
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