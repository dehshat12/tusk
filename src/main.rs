mod editor;
mod terminal;
mod fileio;
mod keymap;
mod filetype;

use std::env;
use std::io::Result;

use editor::Editor;
use terminal::TerminalGuard;

const VERSION: &str = "0.1.0";

fn print_help() {
    println!(
        "tusk {version}
A small TUI text editor inspired by JOE.

USAGE:
    tusk [FILE]

OPTIONS:
    --help       Show this help message
    --version    Show version information

KEYS:
    Ctrl+S       Save
    Ctrl+Q       Quit
",
        version = VERSION
    );
}

fn main() -> Result<()> {
    let mut args = env::args().skip(1);

    if let Some(arg) = args.next() {
        match arg.as_str() {
            "--help" | "-h" => {
                print_help();
                return Ok(());
            }
            "--version" | "-V" => {
                println!("tusk {}", VERSION);
                return Ok(());
            }
            _ => {}
        }
    }

    // Terminal guard must be created AFTER help/version exits
    let _term = TerminalGuard::new()?;

    let mut editor = if let Some(path) = env::args().nth(1) {
        Editor::open(path)?
    } else {
        Editor::new()
    };

    editor.run()?;
    Ok(())
}
