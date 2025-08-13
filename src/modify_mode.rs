use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Stylize},
    symbols::Marker,
    widgets::{
        Block,
        canvas::{Canvas, Rectangle},
    },
};

use crate::{App, grid};

#[derive(Debug)]
pub struct ModifyMode {
    pub cursor_coord: (usize, usize),
}
impl Default for ModifyMode {
    fn default() -> Self {
        Self::new(1, 1)
    }
}

impl ModifyMode {
    pub fn new(x: usize, y: usize) -> ModifyMode {
        ModifyMode {
            cursor_coord: (x, y),
        }
    }

    pub fn render(&mut self, frame: &mut Frame, block: &Block, grid: &grid::Grid, area: &Rect) {
        let canvas = Canvas::default()
            // .marker(Marker::Dot)
            .block(block.clone())
            .x_bounds([-0.0, grid.width() as f64])
            .y_bounds([-0.0, grid.height() as f64])
            .paint(|ctx| {
                // ctx.draw(&Rectangle {
                //     x: self.cursor_coord.0 as f64 + 0.5,
                //     y: self.cursor_coord.1 as f64 + 0.5,
                //     width:  0.1,
                //     height: 0.1,
                //     color: ratatui::style::Color::Gray,
                // });
                let cursor = match grid[self.cursor_coord] {
                    Some(true) => "X".fg(Color::White).bg(Color::DarkGray),
                    _ => "X".bg(Color::White).fg(Color::DarkGray),
                };
                ctx.print(
                    self.cursor_coord.0 as f64,
                    self.cursor_coord.1 as f64,
                    cursor, // "X"
                            //     .bg(Color::Reset)
                            // .fg(Color::White),
                );
            });

        frame.render_widget(canvas, *area);
    }

    // pub fn handle_key_press<T: Fn(KeyEvent) -> ()>(
    //     self,
    //     app: &mut App,
    //     key: KeyEvent,
    //     default_handler: T,
    // ) {
    //     match key.code {
    //         KeyCode::Esc | KeyCode::Char('q') => app.game_running = true,
    //         _ => {
    //             default_handler(key);
    //         }
    //     }
    // }
}
