extern crate clipboard;
extern crate isatty;

use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;

use std::env;
use std::error::Error;
use std::io;
use std::io::prelude::*;

pub mod mode;
use mode::Mode;

fn main() -> Result<(), Box<dyn Error>> {
    let mode = mode::detect_mode(env::args().skip(1))?;

    match mode {
        Mode::Write => write(),
        Mode::Read => read(),
        Mode::ShowHelp => show_help(),
    }
}

fn write() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    let stdin = io::stdin();
    stdin.lock().read_to_string(&mut input)?;
    let mut provider = ClipboardContext::new()?;
    provider.set_contents(input)
}

fn read() -> Result<(), Box<dyn Error>> {
    let mut provider = ClipboardContext::new()?;
    provider.get_contents().map(|res| print!("{}", res))
}

fn show_help() -> Result<(), Box<dyn Error>> {
    [
        "cliputil: clipboard utility",
        "",
        "Usage: cliputil [option]",
        "",
        "available options:",
        "    -w, --write    write stdin to clipboard.",
        "    -r, --read     display current clipboard data.",
        "    -h, --help     show this help.",
    ]
        .iter()
        .for_each(|msg| println!("{}", msg));

    Ok(())
}
