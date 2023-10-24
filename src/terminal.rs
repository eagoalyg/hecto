use std::io::{stdout, stdin, Write};

use termion::{input::TermRead, raw::{IntoRawMode, RawTerminal}, event::Key, color};

use crate::editor::Position;

pub struct Size {
    pub width: u16,
    pub height: u16,
}

pub struct Terminal {
    size: Size,
    _stdout: RawTerminal<std::io::Stdout>,
}

impl Terminal {
    pub fn default() -> Result<Self, std::io::Error> {
        let (width, height) = termion::terminal_size()?;
        Ok(Self {
            size: Size { width, height: height.saturating_sub(2) },
            _stdout: stdout().into_raw_mode()?, 
        })
    }

    pub fn size(&self) -> &Size {
        &self.size
    }

    pub fn clear_screen() {
        print!("{}", termion::clear::All);
    }

    pub fn cursor_position(position: &Position) {
        let mut x = position.x;
        let mut y = position.y;
        x = x.saturating_add(1);
        y = y.saturating_add(1);
        print!("{}", termion::cursor::Goto(x as u16, y as u16));
    }

    pub fn reset_cursor_position() {
        print!("{}", termion::cursor::Goto(1, 1));
    }

    pub fn flush() -> Result<(), std::io::Error> {
        stdout().flush()
    }

    pub fn read_key() -> Result<Key, std::io::Error>{
        loop {
            if let Some(key) = stdin().lock().keys().next() {
                return key;
            }
        }
    }

    pub fn cursor_hide() {
        print!("{}", termion::cursor::Hide);
    }

    pub fn cursor_show() {
        print!("{}", termion::cursor::Show);
    }

    pub fn clear_current_line() {
        print!("{}", termion::clear::CurrentLine);
    } 

    pub fn set_bg_color(color: color::Rgb) {
        print!("{}", color::Bg(color)); 
    }

    pub fn reset_bg_color() {
        print!("{}", color::Bg(color::Reset));
    }

    pub fn set_fg_color(color: color::Rgb) {
        print!("{}", color::Fg(color)); 
    }

    pub fn reset_fg_color() {
        print!("{}", color::Fg(color::Reset));
    }
}