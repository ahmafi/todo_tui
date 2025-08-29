use std::fmt::Debug;

use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    text::Line,
    widgets::{Block, Borders, Paragraph},
};

use crate::todo::list::TodoList;

/// helper function to create a centered rect using up certain percentage of the available rect `r`
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    // Cut the given rectangle into three vertical pieces
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    // Then cut the middle vertical piece into three width-wise pieces
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1] // Return the middle chunk
}

enum Screens {
    Main,
    New,
    Remove,
}

impl Debug for Screens {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Main => f.write_str("Main"),
            Self::New => f.write_str("New"),
            Self::Remove => f.write_str("Remove"),
        }
    }
}

impl Default for Screens {
    fn default() -> Self {
        Self::Main
    }
}

/// The main application which holds the state and logic of the application.
#[derive(Debug, Default)]
pub struct TuiApp {
    /// Is the application running?
    running: bool,
    current_screen: Screens,
    todo_list: TodoList,
}

impl TuiApp {
    /// Construct a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Run the application's main loop.
    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        self.running = true;
        while self.running {
            terminal.draw(|frame| self.render(frame))?;
            self.handle_crossterm_events()?;
        }
        Ok(())
    }

    /// Renders the user interface.
    ///
    /// This is where you add new widgets. See the following resources for more information:
    ///
    /// - <https://docs.rs/ratatui/latest/ratatui/widgets/index.html>
    /// - <https://github.com/ratatui/ratatui/tree/main/ratatui-widgets/examples>
    fn render(&mut self, frame: &mut Frame) {
        let layout =
            Layout::new(Direction::Horizontal, [Constraint::Ratio(1, 3); 3]).split(frame.area());

        frame.render_widget(
            Paragraph::new("0").block(Block::new().borders(Borders::ALL)),
            layout[0],
        );
        frame.render_widget(
            Paragraph::new("1").block(Block::new().borders(Borders::ALL)),
            layout[1],
        );
        frame.render_widget(
            Paragraph::new("2").block(Block::new().borders(Borders::ALL)),
            layout[2],
        );

        match self.current_screen {
            Screens::Main => {}
            Screens::New => {
                let popup_block = Block::default()
                    .title("Enter a new todo")
                    .borders(Borders::ALL)
                    .style(Style::default().bg(Color::DarkGray));
                let area = centered_rect(60, 25, frame.area());
                frame.render_widget(popup_block, area);
            }
            Screens::Remove => {}
        }
    }

    /// Reads the crossterm events and updates the state of [`App`].
    ///
    /// If your application needs to perform work in between handling events, you can use the
    /// [`event::poll`] function to check if there are any events available with a timeout.
    fn handle_crossterm_events(&mut self) -> Result<()> {
        match event::read()? {
            // it's important to check KeyEventKind::Press to avoid handling key release events
            Event::Key(key) if key.kind == KeyEventKind::Press => self.on_key_event(key),
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
            _ => {}
        }
        Ok(())
    }

    /// Handles the key events and updates the state of [`App`].
    fn on_key_event(&mut self, key: KeyEvent) {
        match (key.modifiers, key.code) {
            (_, KeyCode::Esc | KeyCode::Char('q'))
            | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => self.quit(),
            (_, KeyCode::Char('n')) => self.current_screen = Screens::New,
            // Add other key handlers here.
            _ => {}
        }
    }

    /// Set running to false to quit the application.
    fn quit(&mut self) {
        self.running = false;
    }
}
