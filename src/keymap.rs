use crossterm::event::{KeyCode, KeyModifiers};

#[derive(Clone, Copy)]
pub enum Command {
    Quit,
    Save,
    None,
}

pub fn lookup(code: KeyCode, mods: KeyModifiers) -> Command {
    match (code, mods) {
        (KeyCode::Char('q'), KeyModifiers::CONTROL) => Command::Quit,
        (KeyCode::Char('s'), KeyModifiers::CONTROL) => Command::Save,
        _ => Command::None,
    }
}
