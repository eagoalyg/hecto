use std::{env, time::{Instant, Duration}};

use termion::{event::Key, color};

use crate::{terminal::Terminal, Document, Row};

const STATUS_FG_COLOR: color::Rgb = color::Rgb(63, 63, 63);
const STATUS_BG_COLOR: color::Rgb = color::Rgb(239, 239, 239);
const VERSION: &str = env!("CARGO_PKG_VERSION");

struct StatusMessage {
    text: String,
    time: Instant,
}

impl StatusMessage {
    fn from(msg: String) -> Self {
        Self { text: msg, time: Instant::now() } 
    } 
}

pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
    cursor_position: Position,
    offset: Position,
    document: Document,
    status_message: StatusMessage,
}

impl Editor {
    pub fn default() -> Self {
        let mut status_message = String::from("HELP: Ctrl-Q = quit | Ctrl-S = save");
        let document = if let Ok(document) = Document::open(&parse_filename()) {
            document
        } else {
            status_message = format!("ERR: Could not open file: {}", &parse_filename());
            Document::default()
        };

        Self { 
            should_quit: false,
            terminal: Terminal::default().expect("Failed to initialized terminal"),
            cursor_position: Position::default(),
            offset: Position::default(),
            document: document,
            status_message: StatusMessage::from(status_message),
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
            self.draw_status_bar();
            self.draw_message_bar();
            Terminal::cursor_position(&Position {
                x: self.cursor_position.x.saturating_sub(self.offset.x),
                y: self.cursor_position.y.saturating_sub(self.offset.y), 
            });
        }
        Terminal::cursor_show();
        Terminal::flush()
    }

    fn save(&mut self) {
        if self.document.filename.is_none() {
            let result = self.prompt("Save as: ").unwrap_or(None);
            if result.is_none() {
                self.status_message = StatusMessage::from("Save aborted".to_string());
                return;
            }
            self.document.filename = result;
        }
    
        if self.document.save().is_ok() {
            self.status_message = StatusMessage::from("File saved successfully.".to_string());
        } else {
            self.status_message = StatusMessage::from("Error writing file!".to_string());
        }
    }

    fn process_keypress(&mut self) -> Result<(), std::io::Error> {
        let pressed_key = Terminal::read_key()?;
        match pressed_key {
            Key::Ctrl('c') => self.should_quit = true,
            Key::Ctrl('s') => self.save(),
            Key::Char(c) => {
                self.document.insert(&self.cursor_position, c);
                self.cursor_move(Key::Right);
            }
            Key::Left | Key::Right | Key::Up | Key::PageUp | Key::Down | Key::PageDown => self.cursor_move(pressed_key),
            Key::Delete | Key::Backspace => {
                if self.cursor_position.x > 0 || self.cursor_position.y > 0 {
                    self.cursor_move(Key::Left);
                    self.document.delete(&self.cursor_position);
                }
            }
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
        let height = self.document.len();

        match key {
            Key::Left => {
                // press at the beginning of the line to move to the end of the previous line
                if x == 0 && y != 0 {
                    y = y.saturating_sub(1);
                    x = if let Some(row) = self.document.row(y) {
                        row.len()
                    } else {
                        0
                    };
                } else {
                    x = x.saturating_sub(1);
                }
            }
            Key::Right => {
                if x < width {
                    x = x + 1;
                } else if y < height {
                    y = y + 1;
                    x = 0;
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

    fn draw_row(&self, row: &Row) {
        let start = self.offset.x;
        let end = self.terminal.size().width as usize + self.offset.x;
        let row = row.render(start, end);
        println!("{}\r", row);
    }

    fn draw_rows(&self) {
        let height = self.terminal.size().height;
        for line in 0..height {
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

    fn draw_status_bar(&self) {
        let mut status;
        let width = self.terminal.size().width as usize;

        let mut filename = "No Name".to_string();
        if let Some(file_name) = self.document.filename.clone() {
            filename = file_name.to_string();
            filename.truncate(20);
        }
        status = format!("{} - {} lines", filename, self.document.len());

        let line_indicator = format!("{}/{}", self.cursor_position.y + 1, self.document.len());
        
        let len = status.len() + line_indicator.len();
        if width > len {
            status.push_str(&" ".repeat(width - len));
        }
        
        status.push_str(&line_indicator);
        Terminal::set_bg_color(STATUS_BG_COLOR);
        Terminal::set_fg_color(STATUS_FG_COLOR);
        println!("{}\r", status);
        Terminal::reset_bg_color();
        Terminal::reset_fg_color();
    }

    fn draw_message_bar(&self) {
        Terminal::clear_current_line(); 
        let message = &self.status_message;
        if Instant::now() - message.time < Duration::new(5, 0) {
            let mut text = message.text.clone();
            text.truncate(self.terminal.size().width as usize);
            print!("{}", text);
        }
    }

    fn prompt(&mut self, prompt: &str) -> Result<Option<String>, std::io::Error>{
        let mut result = String::new();
        loop {
            self.status_message = StatusMessage::from(format!("{}{}", prompt, result));
            self.refresh_screen()?;
            match Terminal::read_key()? {
                Key::Backspace => {
                    result.truncate(result.len() - 1);
                }
                Key::Char('\n') => break,
                Key::Char(c) => {
                    if !c.is_control() {
                        result.push(c);
                    }
                }
                Key::Esc => {
                    result.truncate(0);
                    break;
                }
                _ => ()
            }
        } 

        self.status_message = StatusMessage::from(String::new());
        match result.len() {
            0 => Ok(None),
            _ => Ok(Some(result)),
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