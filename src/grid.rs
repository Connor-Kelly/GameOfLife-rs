use std::{cell, fmt::format, ops::Index};

use ratatui::{
    layout::Rect,
    style::Color,
    symbols::Marker,
    text::Line,
    widgets::{
        Block, Paragraph, Widget,
        canvas::{Canvas, Context, Map, MapResolution, Points, Rectangle},
    },
};

#[derive(Debug, Clone, Default)]
pub struct Grid {
    pub cells: Vec<Vec<Cell>>,
}

type Cell = Option<bool>;

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
    pub fn get_widget(
        &self,
        area: &Rect,
        iteration: usize,
    ) -> Canvas<'_, impl Fn(&mut ratatui::widgets::canvas::Context<'_>)> {
        Canvas::default()
            .marker(Marker::Block)
            .block(Block::bordered().title("GoL"))
            .x_bounds([-0.0, self.width() as f64])
            .y_bounds([-0.0, self.height() as f64])
            .paint(move |ctx| {
                self.cells.iter().enumerate().for_each(|(j, row)| {
                    row.iter().enumerate().for_each(|(i, cell)| {
                        if let Some(true) = cell {
                            let iter_offset: usize = (iteration * 2
                                + usize::try_from(
                                    iteration as i32 * if iteration > 128 { 1 } else { -1 },
                                )
                                .unwrap_or(0))
                                % 255;
                            // let offset = (i as f64, j as f64);
                            let offset = |x: usize| (x * 3 + iter_offset) as u8 % 255;
                            ctx.print(0.0, 0.0, format!("{iter_offset}"));
                            ctx.draw(&Rectangle {
                                x: i as f64 + 0.5,
                                y: j as f64 + 0.5,
                                width: 0.1,
                                height: 0.1,
                                color: Color::Rgb(
                                    offset(i + j),
                                    offset(i * j),
                                    offset(1),
                                    // (i * 2 + iteration) as u8 % 255,
                                    // (j * 2 + iteration) as u8 % 255,
                                    // (i + j * 3 + iteration) as u8 % 255,
                                ),
                            });
                        }
                    })
                });
                ctx.layer();
            })
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
