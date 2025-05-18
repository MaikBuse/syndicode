use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{Block, Borders};
use std::fmt;
use tui_textarea::{CursorMove, Input, Key, Scrolling, TextArea};

use crate::presentation::theme::CYBER_FG;

// Default style for the TextArea's wrapping Block (title and border)
// Analogous to unselected input block style from ServiceDetailWidget
pub const RESPONSE_TEXTAREA_BLOCK_DEFAULT_STYLE: Style =
    Style::new().fg(CYBER_FG).add_modifier(Modifier::DIM);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Normal,
    Insert,
    Visual,
    Operator(char),
}

impl Mode {
    pub fn block<'a>(&self) -> Block<'a> {
        let help = match self {
            Self::Normal => "type Esc to quit, type i to enter insert mode",
            Self::Insert => "type Esc to back to normal mode",
            Self::Visual => "type y to yank, type d to delete, type Esc to back to normal mode",
            Self::Operator(_) => "move cursor to apply operator",
        };
        let title = format!("{} MODE ({})", self, help);
        Block::default()
            .borders(Borders::ALL)
            .title(title)
            .style(RESPONSE_TEXTAREA_BLOCK_DEFAULT_STYLE)
    }

    pub fn cursor_style(&self) -> Style {
        let color = match self {
            Self::Normal => Color::Reset,
            Self::Insert => Color::LightBlue,
            Self::Visual => Color::LightYellow,
            Self::Operator(_) => Color::LightGreen,
        };
        Style::default().fg(color).add_modifier(Modifier::REVERSED)
    }
}

impl fmt::Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Self::Normal => write!(f, "NORMAL"),
            Self::Insert => write!(f, "INSERT"),
            Self::Visual => write!(f, "VISUAL"),
            Self::Operator(c) => write!(f, "OPERATOR({})", c),
        }
    }
}

// How the Vim emulation state transitions
pub enum Transition {
    Nop,
    Mode(Mode),
    Pending(Input),
    Quit,
}

// State of Vim emulation
#[derive(Debug, Clone)]
pub struct Vim {
    pub mode: Mode,
    pub pending: Input, // Pending input to handle a sequence with two keys like gg
}

impl Vim {
    pub fn new(mode: Mode) -> Self {
        Self {
            mode,
            pending: Input::default(),
        }
    }

    pub fn with_pending(self, pending: Input) -> Self {
        Self {
            mode: self.mode,
            pending,
        }
    }

    pub fn transition(&self, response_detail_textarea: &mut TextArea, input: Input) -> Transition {
        if input.key == Key::Null {
            return Transition::Nop;
        }

        match self.mode {
            Mode::Normal | Mode::Visual | Mode::Operator(_) => {
                match input {
                    Input {
                        key: Key::Char('h'),
                        ..
                    } => response_detail_textarea.move_cursor(CursorMove::Back),
                    Input {
                        key: Key::Char('j'),
                        ..
                    } => response_detail_textarea.move_cursor(CursorMove::Down),
                    Input {
                        key: Key::Char('k'),
                        ..
                    } => response_detail_textarea.move_cursor(CursorMove::Up),
                    Input {
                        key: Key::Char('l'),
                        ..
                    } => response_detail_textarea.move_cursor(CursorMove::Forward),
                    Input {
                        key: Key::Char('w'),
                        ..
                    } => response_detail_textarea.move_cursor(CursorMove::WordForward),
                    Input {
                        key: Key::Char('e'),
                        ctrl: false,
                        ..
                    } => {
                        response_detail_textarea.move_cursor(CursorMove::WordEnd);
                        if matches!(self.mode, Mode::Operator(_)) {
                            response_detail_textarea.move_cursor(CursorMove::Forward);
                            // Include the text under the cursor
                        }
                    }
                    Input {
                        key: Key::Char('b'),
                        ctrl: false,
                        ..
                    } => response_detail_textarea.move_cursor(CursorMove::WordBack),
                    Input {
                        key: Key::Char('^'),
                        ..
                    } => response_detail_textarea.move_cursor(CursorMove::Head),
                    Input {
                        key: Key::Char('$'),
                        ..
                    } => response_detail_textarea.move_cursor(CursorMove::End),
                    Input {
                        key: Key::Char('D'),
                        ..
                    } => {
                        response_detail_textarea.delete_line_by_end();
                        return Transition::Mode(Mode::Normal);
                    }
                    Input {
                        key: Key::Char('C'),
                        ..
                    } => {
                        response_detail_textarea.delete_line_by_end();
                        response_detail_textarea.cancel_selection();
                        return Transition::Mode(Mode::Insert);
                    }
                    Input {
                        key: Key::Char('p'),
                        ..
                    } => {
                        response_detail_textarea.paste();
                        return Transition::Mode(Mode::Normal);
                    }
                    Input {
                        key: Key::Char('u'),
                        ctrl: false,
                        ..
                    } => {
                        response_detail_textarea.undo();
                        return Transition::Mode(Mode::Normal);
                    }
                    Input {
                        key: Key::Char('r'),
                        ctrl: true,
                        ..
                    } => {
                        response_detail_textarea.redo();
                        return Transition::Mode(Mode::Normal);
                    }
                    Input {
                        key: Key::Char('x'),
                        ..
                    } => {
                        response_detail_textarea.delete_next_char();
                        return Transition::Mode(Mode::Normal);
                    }
                    Input {
                        key: Key::Char('i'),
                        ..
                    } => {
                        response_detail_textarea.cancel_selection();
                        return Transition::Mode(Mode::Insert);
                    }
                    Input {
                        key: Key::Char('a'),
                        ..
                    } => {
                        response_detail_textarea.cancel_selection();
                        response_detail_textarea.move_cursor(CursorMove::Forward);
                        return Transition::Mode(Mode::Insert);
                    }
                    Input {
                        key: Key::Char('A'),
                        ..
                    } => {
                        response_detail_textarea.cancel_selection();
                        response_detail_textarea.move_cursor(CursorMove::End);
                        return Transition::Mode(Mode::Insert);
                    }
                    Input {
                        key: Key::Char('o'),
                        ..
                    } => {
                        response_detail_textarea.move_cursor(CursorMove::End);
                        response_detail_textarea.insert_newline();
                        return Transition::Mode(Mode::Insert);
                    }
                    Input {
                        key: Key::Char('O'),
                        ..
                    } => {
                        response_detail_textarea.move_cursor(CursorMove::Head);
                        response_detail_textarea.insert_newline();
                        response_detail_textarea.move_cursor(CursorMove::Up);
                        return Transition::Mode(Mode::Insert);
                    }
                    Input {
                        key: Key::Char('I'),
                        ..
                    } => {
                        response_detail_textarea.cancel_selection();
                        response_detail_textarea.move_cursor(CursorMove::Head);
                        return Transition::Mode(Mode::Insert);
                    }
                    Input {
                        key: Key::Char('q'),
                        ..
                    } => return Transition::Quit,
                    Input {
                        key: Key::Char('e'),
                        ctrl: true,
                        ..
                    } => response_detail_textarea.scroll((1, 0)),
                    Input {
                        key: Key::Char('y'),
                        ctrl: true,
                        ..
                    } => response_detail_textarea.scroll((-1, 0)),
                    Input {
                        key: Key::Char('d'),
                        ctrl: true,
                        ..
                    } => response_detail_textarea.scroll(Scrolling::HalfPageDown),
                    Input {
                        key: Key::Char('u'),
                        ctrl: true,
                        ..
                    } => response_detail_textarea.scroll(Scrolling::HalfPageUp),
                    Input {
                        key: Key::Char('f'),
                        ctrl: true,
                        ..
                    } => response_detail_textarea.scroll(Scrolling::PageDown),
                    Input {
                        key: Key::Char('b'),
                        ctrl: true,
                        ..
                    } => response_detail_textarea.scroll(Scrolling::PageUp),
                    Input {
                        key: Key::Char('v'),
                        ctrl: false,
                        ..
                    } if self.mode == Mode::Normal => {
                        response_detail_textarea.start_selection();
                        return Transition::Mode(Mode::Visual);
                    }
                    Input {
                        key: Key::Char('V'),
                        ctrl: false,
                        ..
                    } if self.mode == Mode::Normal => {
                        response_detail_textarea.move_cursor(CursorMove::Head);
                        response_detail_textarea.start_selection();
                        response_detail_textarea.move_cursor(CursorMove::End);
                        return Transition::Mode(Mode::Visual);
                    }
                    Input { key: Key::Esc, .. } if self.mode == Mode::Normal => {
                        return Transition::Quit;
                    }
                    Input { key: Key::Esc, .. }
                    | Input {
                        key: Key::Char('v'),
                        ctrl: false,
                        ..
                    } if self.mode == Mode::Visual => {
                        response_detail_textarea.cancel_selection();
                        return Transition::Mode(Mode::Normal);
                    }
                    Input {
                        key: Key::Char('g'),
                        ctrl: false,
                        ..
                    } if matches!(
                        self.pending,
                        Input {
                            key: Key::Char('g'),
                            ctrl: false,
                            ..
                        }
                    ) =>
                    {
                        response_detail_textarea.move_cursor(CursorMove::Top)
                    }
                    Input {
                        key: Key::Char('G'),
                        ctrl: false,
                        ..
                    } => response_detail_textarea.move_cursor(CursorMove::Bottom),
                    Input {
                        key: Key::Char(c),
                        ctrl: false,
                        ..
                    } if self.mode == Mode::Operator(c) => {
                        // Handle yy, dd, cc. (This is not strictly the same behavior as Vim)
                        response_detail_textarea.move_cursor(CursorMove::Head);
                        response_detail_textarea.start_selection();
                        let cursor = response_detail_textarea.cursor();
                        response_detail_textarea.move_cursor(CursorMove::Down);
                        if cursor == response_detail_textarea.cursor() {
                            response_detail_textarea.move_cursor(CursorMove::End);
                            // At the last line, move to end of the line instead
                        }
                    }
                    Input {
                        key: Key::Char(op @ ('y' | 'd' | 'c')),
                        ctrl: false,
                        ..
                    } if self.mode == Mode::Normal => {
                        response_detail_textarea.start_selection();
                        return Transition::Mode(Mode::Operator(op));
                    }
                    Input {
                        key: Key::Char('y'),
                        ctrl: false,
                        ..
                    } if self.mode == Mode::Visual => {
                        response_detail_textarea.move_cursor(CursorMove::Forward); // Vim's text selection is inclusive
                        response_detail_textarea.copy();
                        return Transition::Mode(Mode::Normal);
                    }
                    Input {
                        key: Key::Char('d'),
                        ctrl: false,
                        ..
                    } if self.mode == Mode::Visual => {
                        response_detail_textarea.move_cursor(CursorMove::Forward); // Vim's text selection is inclusive
                        response_detail_textarea.cut();
                        return Transition::Mode(Mode::Normal);
                    }
                    Input {
                        key: Key::Char('c'),
                        ctrl: false,
                        ..
                    } if self.mode == Mode::Visual => {
                        response_detail_textarea.move_cursor(CursorMove::Forward); // Vim's text selection is inclusive
                        response_detail_textarea.cut();
                        return Transition::Mode(Mode::Insert);
                    }
                    input => return Transition::Pending(input),
                }

                // Handle the pending operator
                match self.mode {
                    Mode::Operator('y') => {
                        response_detail_textarea.copy();
                        Transition::Mode(Mode::Normal)
                    }
                    Mode::Operator('d') => {
                        response_detail_textarea.cut();
                        Transition::Mode(Mode::Normal)
                    }
                    Mode::Operator('c') => {
                        response_detail_textarea.cut();
                        Transition::Mode(Mode::Insert)
                    }
                    _ => Transition::Nop,
                }
            }
            Mode::Insert => match input {
                Input { key: Key::Esc, .. }
                | Input {
                    key: Key::Char('c'),
                    ctrl: true,
                    ..
                } => Transition::Mode(Mode::Normal),
                input => {
                    response_detail_textarea.input(input); // Use default key mappings in insert mode
                    Transition::Mode(Mode::Insert)
                }
            },
        }
    }
}
