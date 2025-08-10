use std::{thread::sleep, time::Duration};

use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers, MediaKeyCode};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Layout},
    style::Stylize,
    text::Line,
    widgets::{Block, Paragraph},
};

mod game_of_life;
mod grid;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = App::new().run(terminal);
    ratatui::restore();
    result
}

/// The main application which holds the state and logic of the application.
#[derive(Debug, Default)]
pub struct App {
    /// Is the application running?
    running: bool,
    grid: grid::Grid,
    game: game_of_life::GameOfLifeIterator,
}

impl App {
    /// Construct a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Run the application's main loop.
    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        self.running = true;

        if let Ok(size) = terminal.size() {
            self.grid = grid::Grid::new(size.height as usize, size.width as usize)
        }

        // init the grid and run it once
        self.game = game_of_life::GameOfLifeIterator::init_with_grid(&self.grid);
        self.grid = self.game.grid.clone();

        while self.running {
            if let Some(next_grid) = self.game.next() {
                self.grid = next_grid;
            } else {
                panic!("Could not compute next grid")
            }
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
        let title = Line::from("Ratatui Simple Template")
            .bold()
            .blue()
            .centered();

        let par = Paragraph::new(format!(
            "Connor's Game of Life
            Press `Esc`, `Ctrl-C` or `q` to stop running.\n\
            grid size: {}, {}
            ",
            self.grid.height(),
            self.grid.width()
        ))
        .block(Block::bordered().title(title))
        .centered();

        let layout = Layout::new(
            ratatui::layout::Direction::Vertical,
            [Constraint::Length(5), Constraint::Fill(1)],
        )
        .split(frame.area());

        let r = layout[1];

        frame.render_widget(par, layout[0]);

        frame.render_widget(
            self.grid.get_widget(&layout[1], self.game.iteration),
            layout[1],
        );
    }

    /// Reads the crossterm events and updates the state of [`App`].
    ///
    /// If your application needs to perform work in between handling events, you can use the
    /// [`event::poll`] function to check if there are any events available with a timeout.
    fn handle_crossterm_events(&mut self) -> Result<()> {
        if let Ok(true) = event::poll(Duration::from_secs_f32(0.05)) {
            match event::read()? {
                // it's important to check KeyEventKind::Press to avoid handling key release events
                Event::Key(KeyEvent {
                    code: KeyCode::Enter,
                    modifiers: KeyModifiers,
                    kind: KeyEventKind::Press,
                    state,
                }) => self.on_enter_press(),
                Event::Key(key) if key.kind == KeyEventKind::Press => self.on_key_event(key),
                Event::Mouse(_) => {}
                Event::Resize(_, _) => {}
                _ => {}
            }
        }
        Ok(())
    }

    /// Handles the key events and updates the state of [`App`].
    fn on_key_event(&mut self, key: KeyEvent) {
        match (key.modifiers, key.code) {
            (_, KeyCode::Esc | KeyCode::Char('q'))
            | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => self.quit(),
            // Add other key handlers here.
            _ => {}
        }
    }
    /// Handles the key events and updates the state of [`App`].
    fn on_enter_press(&mut self) {
        self.game = game_of_life::GameOfLifeIterator::init_with_grid(&grid::Grid::new(
            self.grid.height(),
            self.grid.width(),
        ));
        // if let Some(next_grid) = self.game.next() {
        //     self.grid = next_grid;
        // } else {
        //     panic!("Could not compute next grid")
        // }
        // panic!("Enter Pressed")
    }

    /// Setcargo install cargo-generate running to false to quit the application.
    fn quit(&mut self) {
        self.running = false;
    }
}
