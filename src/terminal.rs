use std::io::{self, Result};
use crossterm::{
    execute,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    cursor,
};

/// RAII guard that guarantees terminal cleanup
pub struct TerminalGuard {
    active: bool,
}

impl TerminalGuard {
    /// Initialize terminal (raw mode + alt screen)
    pub fn new() -> Result<Self> {
        terminal::enable_raw_mode()?;
        execute!(
            io::stdout(),
            EnterAlternateScreen,
            cursor::Hide
        )?;

        Ok(Self { active: true })
    }
}

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        if self.active {
            let _ = execute!(
                io::stdout(),
                LeaveAlternateScreen,
                cursor::Show
            );
            let _ = terminal::disable_raw_mode();
            self.active = false;
        }
    }
}

/// Legacy-style explicit initialization (optional)
pub fn init_terminal() -> Result<()> {
    terminal::enable_raw_mode()?;
    execute!(
        io::stdout(),
        EnterAlternateScreen,
        cursor::Hide
    )?;
    Ok(())
}

/// Legacy-style explicit restore (optional)
pub fn restore_terminal() -> Result<()> {
    let _ = execute!(
        io::stdout(),
        LeaveAlternateScreen,
        cursor::Show
    );
    let _ = terminal::disable_raw_mode();
    Ok(())
}

