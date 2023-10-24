use std::env;

use termion::event::Key;

use crate::{terminal::Terminal, Document, Row};

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
    cursor_position: Position,
    offset: Position,
    document: Document,
}

impl Editor {
    pub fn default() -> Self {
        Self { 
            should_quit: false,
            terminal: Terminal::default().expect("Failed to initialized terminal"),
            cursor_position: Position::default(),
            offset: Position::default(),
            document: Document::open(&parse_filename()).expect("Fail to parse file"),
        }
    }

    pub fn run(&mut self) {
        loop {
            if let Err(error) = self.refresh_screen() {
                die(error);
            }
            if self.should_quit {
                break;
            }
            if let Err(error) = self.process_keypress() {
                die(error);
            }
        }
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        Terminal::cursor_hide();
        Terminal::reset_cursor_position();
        if self.should_quit {
            Terminal::clear_screen();
            println!("Goodbye.\r");
        } else {
            self.draw_rows();
            Terminal::cursor_position(&Position {
                x: self.cursor_position.x.saturating_sub(self.offset.x),
                y: self.cursor_position.y.saturating_sub(self.offset.y), 
            });
        }
        Terminal::cursor_show();
        Terminal::flush()
    }

    fn process_keypress(&mut self) -> Result<(), std::io::Error> {
        let pressed_key = Terminal::read_key()?;
        match pressed_key {
            Key::Ctrl('c') => self.should_quit = true,
            Key::Left | Key::Right | Key::Up | Key::PageUp | Key::Down | Key::PageDown => self.cursor_move(pressed_key),
            _ => (),
        }
        self.scroll();
        Ok(())
    }

    fn scroll(&mut self) {
        let Position { x, y } = self.cursor_position;
        let width = self.terminal.size().width as usize;
        let height = self.terminal.size().height as usize;

        let offset = &mut self.offset;
        if y < offset.y {
            offset.y = y;
        } else if y >= offset.y.saturating_add(height) {
            offset.y = y.saturating_sub(height).saturating_add(1);
        }
        if x < offset.x {
            offset.x = x;
        } else if x >= offset.x.saturating_add(width) {
            offset.x = x.saturating_sub(width).saturating_add(1);
        }
    }

    fn cursor_move(&mut self, key: Key) {
        let Position {mut x, mut y} = self.cursor_position;
        let mut width = if let Some(row) = self.document.row(self.cursor_position.y) {
            row.len()
        } else {
            0
        };

        match key {
            Key::Left => x = x.saturating_sub(1),
            Key::Right => {
                if x < width {
                    x = x.saturating_add(1);
                }
            },
            Key::Down | Key::PageDown => {
                let height = self.document.len();
                if y < height {
                    y = y.saturating_add(1);
                }
            },
            Key::Up | Key::PageUp => y = y.saturating_sub(1),
            _ => (),
        }

        // fix move the row, cursor position bigger than current row's width
        width = if let Some(row) = self.document.row(y) {
            row.len()
        } else {
            0
        };
        if width < x {
            x = width;
            self.offset.x = 0;
        }

        self.cursor_position = Position {x, y};
    }

    fn draw_welcome_msg(&self) {
        let mut welcome_msg = format!("Hecto editor -- version {}\r", VERSION);
        let width = self.terminal.size().width as usize;
        let len = welcome_msg.len();
        let padding = width.saturating_sub(len) >> 1;
        let space = " ".repeat(padding.saturating_sub(1)); 
        welcome_msg.truncate(width);
        println!("~{}{}\r", space, welcome_msg)
    }

    pub fn draw_row(&self, row: &Row) {
        let start = self.offset.x;
        let end = self.terminal.size().width as usize + self.offset.x;
        let row = row.render(start, end);
        println!("{}\r", row);
    }

    fn draw_rows(&self) {
        let height = self.terminal.size().height;
        for line in 0..height - 1 {
            Terminal::clear_current_line();
            if let Some(row) = self.document.row(line as usize + self.offset.y) {
                self.draw_row(row);
            } else if self.document.is_empty() && line == height / 3 {
                self.draw_welcome_msg();
            } else {
                println!("~\r");
            }
        }
    }
}


fn die(e: std::io::Error) {
    Terminal::clear_screen();
    Terminal::reset_cursor_position();
    panic!("{}", e);
}

#[derive(Default)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

fn parse_filename() -> String {
    let mut args = env::args();
    let _ = args.next();
    if let Some(arg) = args.next() {
        return arg
    } else {
        panic!("Please input filename");
    }
}