use std::ops::{Index, IndexMut};

use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Stylize},
    symbols::Marker,
    text::Line,
    widgets::{Block, canvas::Canvas},
};

#[derive(Debug, Clone, Default)]
pub struct Grid {
    pub cells: Vec<Vec<Cell>>,
}

type Cell = Option<bool>;

impl IndexMut<(usize, usize)> for Grid {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        // &self[index.1][index.0]
        // &self.cells
        &mut self.cells[index.1][index.0]
    }
}
impl Index<(usize, usize)> for Grid {
    type Output = Cell;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.cells[index.1][index.0]
    }
}

impl Grid {
    pub fn new(height: usize, width: usize) -> Self {
        Grid {
            cells: vec![vec![None; width]; height],
        }
    }

    pub fn height(self: &Self) -> usize {
        self.cells.len()
    }
    pub fn width(self: &Self) -> usize {
        match self.cells.first() {
            Some(row) => row.len(),
            None => 0,
        }
    }
    fn shade_cell(i: usize, j: usize, iteration: usize) -> Color {
        let iter_offset = (if iteration % 256 <= 128 {
            iteration % 129
        } else {
            128 - iteration % 129
        });
        let offset = |x: usize| (x * 3 + iter_offset) as u8 % 255;
        Color::Rgb(offset(i + j), offset(i * j), offset(1))
    }
    // pub fn get_widget(
    pub fn render(&self, frame: &mut Frame, area: &Rect, iteration: usize) {
        let canvas = Canvas::default()
            .marker(Marker::Block)
            .block(
                Block::bordered()
                    .title("GoL")
                    .title_bottom(Line::from("? (Help)").right_aligned()),
            )
            .x_bounds([-0.0, self.width() as f64])
            .y_bounds([-0.0, self.height() as f64])
            .paint(move |ctx| {
                // "█"
                self.cells.iter().enumerate().for_each(|(j, row)| {
                    row.iter().enumerate().for_each(|(i, cell)| {
                        if let Some(true) = cell {
                            ctx.print(0.0, 0.0, format!("{iteration}"));
                            // ███
                            let shade = Self::shade_cell(i, j, iteration);
                            ctx.print(i as f64, j as f64, Line::from("█").fg(shade).bg(shade));
                            // ctx.draw(&Rectangle {
                            //     x: i as f64 + 0.5,
                            //     y: j as f64 + 0.5,
                            //     width: 0.1,
                            //     height: 0.1,
                            //     color: Self::shade_cell(i, j, iteration),
                            // });
                        }
                    })
                });
                ctx.layer();
            });
        frame.render_widget(canvas, *area);
    }
}

#[cfg(test)]
mod tests {
    use crate::grid::Grid;

    #[test]
    fn init() {
        let height = 3;
        let width = 4;
        let grid = Grid::new(height, width);

        assert_eq!(grid.height(), height);
        assert_eq!(grid.width(), width);
    }
}
