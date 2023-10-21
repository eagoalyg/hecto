use termion::event::Key;

use crate::terminal::Terminal;

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
    cursor_position: Position,
}

impl Editor {
    pub fn default() -> Self {
        Self { 
            should_quit: false,
            terminal: Terminal::default().expect("Failed to initialized terminal"),
            cursor_position: Position {
                x: 0,
                y: 0,
            },
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
            Terminal::cursor_position(&self.cursor_position);
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
        Ok(())
    }

    fn cursor_move(&mut self, key: Key) {
        let Position {mut x, mut y} = self.cursor_position;
        match key {
            Key::Left => x = x.saturating_sub(1),
            Key::Right => {
                if x < self.terminal.size().width as usize {
                    x = x.saturating_add(1);
                }
            },
            Key::Down | Key::PageDown => {
                if y < self.terminal.size().height as usize {
                    y = y.saturating_add(1);
                }
            },
            Key::Up | Key::PageUp => y = y.saturating_sub(1),
            _ => (),
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

    fn draw_rows(&self) {
        let height = self.terminal.size().height;
        for line in 0..height - 1 {
            Terminal::clear_current_line();
            if line == height / 3 {
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

pub struct Position {
    pub x: usize,
    pub y: usize,
}