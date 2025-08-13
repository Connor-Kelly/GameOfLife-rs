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
mod help_popup;
mod modify_mode;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = App::new().run(terminal);
    ratatui::restore();
    result
}

#[derive(Debug)]
enum Mode {
    Run,
    Mod,
    Help,
}
impl Default for Mode {
    fn default() -> Self {
        Self::Run
    }
}

/// The main application which holds the state and logic of the application.
#[derive(Debug, Default)]
pub struct App {
    /// Is the application running?
    running: bool,
    // mode: Mode,
    grid: grid::Grid,
    game_running: bool,
    show_help: bool,
    mod_mode: modify_mode::ModifyMode,
    game: game_of_life::GameOfLifeIterator,
}

impl App {
    /// Construct a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    fn mode(&self) -> Mode {
        if self.show_help {
            Mode::Help
        } else if !self.game_running {
            Mode::Mod
        } else {
            Mode::Run
        }
    }

    /// Run the application's main loop.
    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        self.running = true;
        self.game_running = true;

        if let Ok(size) = terminal.size() {
            self.grid = grid::Grid::new(size.height as usize, size.width as usize)
        }

        // init the grid and run it once
        self.game = game_of_life::GameOfLifeIterator::init_with_grid(&self.grid);
        self.grid = self.game.grid.clone();

        while self.running {
            // self.mode = self.mode();
            if self.game_running {
                if let Some(next_grid) = self.game.next() {
                    self.grid = next_grid;
                } else {
                    panic!("Could not compute next grid")
                }
            }

            terminal.draw(|frame| self.render(frame))?;
            self.handle_crossterm_events()?;
        }
        Ok(())
    }

    /// Renders the user interface.
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

        frame.render_widget(par, layout[0]);

        self.grid.render(frame, &layout[1], self.game.iteration);

        if !self.game_running {
            self.mod_mode.render(frame, &self.grid, &layout[1]);
        }

        if self.show_help {
            help_popup::render_help_popup(frame);
        }
    }

    /// Reads the crossterm events and updates the state of [`App`].
    fn handle_crossterm_events(&mut self) -> Result<()> {
        if let Ok(true) = event::poll(Duration::from_secs_f32(0.025)) {
            match event::read()? {
                Event::Key(key) if key.kind == KeyEventKind::Press => self.on_key_modal(key),
                Event::Mouse(_) => {}
                Event::Resize(_, _) => {}
                _ => {}
            }
        }
        Ok(())
    }

    /// Handles the key events and updates the state of [`App`].
    fn on_key_modal(&mut self, key: KeyEvent) {
        match self.mode() {
            Mode::Run => match (key.modifiers, key.code) {
                (_, KeyCode::Enter) => self.on_enter_press(),
                _ => {
                    self.handle_any_mode_key(key);
                }
            },
            Mode::Mod => match key.code {
                KeyCode::Esc | KeyCode::Char('q') => self.game_running = true,
                KeyCode::Char('k') => {
                    if self.mod_mode.cursor_coord.1 < self.grid.height() {
                        self.mod_mode.cursor_coord.1 += 1
                    }
                }
                KeyCode::Char('j') => {
                    if self.mod_mode.cursor_coord.1 > 0 {
                        self.mod_mode.cursor_coord.1 -= 1
                    }
                }
                KeyCode::Char('l') => {
                    if self.mod_mode.cursor_coord.0 < self.grid.width() {
                        self.mod_mode.cursor_coord.0 += 1
                    }
                }
                KeyCode::Char('h') => {
                    if self.mod_mode.cursor_coord.0 > 0 {
                        self.mod_mode.cursor_coord.0 -= 1
                    }
                },
                KeyCode::Char('t') => {
                    self.grid[self.mod_mode.cursor_coord] = match self.grid[self.mod_mode.cursor_coord] {
                        Some(b) => Some(!b),
                        None => Some(false),
                    };
                    self.game.grid = self.grid.clone();
                }
                _ => {
                    self.handle_any_mode_key(key);
                }
            },
            Mode::Help => match key.code {
                KeyCode::Esc | KeyCode::Char('q') => self.show_help = false,
                _ => {
                    self.handle_any_mode_key(key);
                }
            },
        }
    }

    fn handle_any_mode_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('c') | KeyCode::Char('C') if key.modifiers == KeyModifiers::CONTROL => {
                self.quit()
            }
            KeyCode::Char('?') => self.show_help = !self.show_help,
            KeyCode::Char(' ') => self.game_running = !self.game_running,
            _ => {}
        }
    }
    /// Handles the key events and updates the state of [`App`].
    fn on_enter_press(&mut self) {
        self.game = game_of_life::GameOfLifeIterator::init_with_grid(&grid::Grid::new(
            self.grid.height(),
            self.grid.width(),
        ));
    }

    /// Setcargo install cargo-generate running to false to quit the application.
    fn quit(&mut self) {
        self.running = false;
    }
}
