use clap::{Command, arg, crate_version};

pub fn create_cli() -> Command {
    Command::new("rwatch")
        .version(crate_version!())
        .about("Watch events on files and directories")
        .arg(arg!(--watch <VALUE>).required(true))
        .arg(arg!(--"enable-api"))
}