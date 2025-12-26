use std::io::{self, Write, Result, ErrorKind};

use crossterm::{
    cursor,
    event::{self, KeyCode},
    execute,
    terminal::{self, ClearType},
};

use crate::fileio;
use crate::keymap::{self, Command};
use crate::filetype::{self, FileType};

pub struct Editor {
    pub lines: Vec<String>,
    cx: usize,
    cy: usize,
    row_offset: usize,
    quit: bool,
    filename: Option<String>,
    filetype: FileType,
}

impl Editor {
    pub fn new() -> Self {
        Self {
            lines: vec![String::new()],
            cx: 0,
            cy: 0,
            row_offset: 0,
            quit: false,
            filename: None,
            filetype: FileType::Text,
        }
    }

    pub fn open(path: String) -> Result<Self> {
        let lines = match fileio::load_file(&path) {
            Ok(lines) if !lines.is_empty() => lines,
            Ok(_) => vec![String::new()],
            Err(e) if e.kind() == ErrorKind::NotFound => vec![String::new()],
            Err(e) => return Err(e),
        };

        let ft = filetype::detect(&path);

        Ok(Self {
            lines,
            cx: 0,
            cy: 0,
            row_offset: 0,
            quit: false,
            filename: Some(path),
            filetype: ft,
        })
    }

    pub fn run(&mut self) -> Result<()> {
        while !self.quit {
            self.draw()?;

            if let event::Event::Key(key) = event::read()? {
                self.handle_key(key.code, key.modifiers)?;
            }
        }
        Ok(())
    }

    fn handle_key(
        &mut self,
        code: KeyCode,
        mods: event::KeyModifiers,
    ) -> Result<()> {
        match keymap::lookup(code, mods) {
            Command::Quit => self.quit = true,
            Command::Save => {
                if let Some(name) = &self.filename {
                    fileio::save_file(name, &self.lines)?;
                }
            }
            Command::None => match code {
                KeyCode::Char(c) => self.insert_char(c),
                KeyCode::Enter => self.new_line(),
                KeyCode::Backspace => self.backspace(),
                KeyCode::Up | KeyCode::Down | KeyCode::Left | KeyCode::Right => {
                    self.move_cursor(code)
                }
                _ => {}
            },
        }
        Ok(())
    }

    fn insert_char(&mut self, c: char) {
        self.lines[self.cy].insert(self.cx, c);
        self.cx += 1;
    }

    fn new_line(&mut self) {
        let rest = self.lines[self.cy].split_off(self.cx);
        self.lines.insert(self.cy + 1, rest);
        self.cy += 1;
        self.cx = 0;
    }

    fn backspace(&mut self) {
        if self.cx > 0 {
            self.cx -= 1;
            self.lines[self.cy].remove(self.cx);
        }
    }

    fn move_cursor(&mut self, key: KeyCode) {
        match key {
            KeyCode::Up if self.cy > 0 => self.cy -= 1,
            KeyCode::Down if self.cy + 1 < self.lines.len() => self.cy += 1,
            KeyCode::Left if self.cx > 0 => self.cx -= 1,
            KeyCode::Right if self.cx < self.lines[self.cy].len() => self.cx += 1,
            _ => {}
        }

        let (_, height) = terminal::size().unwrap();
        if self.cy < self.row_offset {
            self.row_offset = self.cy;
        } else if self.cy >= self.row_offset + height as usize - 1 {
            self.row_offset = self.cy - height as usize + 2;
        }
    }

    fn draw(&self) -> Result<()> {
        let mut stdout = io::stdout();
        let (width, height) = terminal::size()?;

        execute!(
            stdout,
            cursor::Hide,
            cursor::MoveTo(0, 0),
            terminal::Clear(ClearType::All)
        )?;

        let text_height = height.saturating_sub(1) as usize;

        for screen_row in 0..text_height {
            let file_row = screen_row + self.row_offset;
            execute!(stdout, cursor::MoveTo(0, screen_row as u16))?;

            if let Some(line) = self.lines.get(file_row) {
                let end = line.len().min(width as usize);
                print!("{}", &line[..end]);
            } else {
                print!("~");
            }
        }

        execute!(
            stdout,
            cursor::MoveTo(0, height - 1),
            terminal::Clear(ClearType::CurrentLine)
        )?;

        let name = self.filename.as_deref().unwrap_or("[No Name]");
        print!(
            "tusk — {} [{}] — Ctrl+S save | Ctrl+Q quit",
            name,
            self.filetype.name()
        );

        let cursor_y = (self.cy.saturating_sub(self.row_offset)) as u16;
        let cursor_x = self.cx as u16;

        execute!(
            stdout,
            cursor::MoveTo(cursor_x, cursor_y),
            cursor::Show
        )?;

        stdout.flush()?;
        Ok(())
    }
}
