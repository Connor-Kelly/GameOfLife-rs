use crate::grid;

pub struct GameOfLife {
    grid: grid::Grid,
}

impl IntoIterator for GameOfLife {
    type Item = grid::Grid;

    type IntoIter = GameOfLifeIterator;

    fn into_iter(self) -> Self::IntoIter {
        GameOfLifeIterator {
            iteration: 0,
            grid: self.grid,
        }
    }
}

#[derive(Debug, Default)]
pub struct GameOfLifeIterator {
    pub iteration: usize,
    pub grid: grid::Grid,
}
impl Iterator for GameOfLifeIterator {
    type Item = grid::Grid;

    fn next(&mut self) -> Option<Self::Item> {
        let mut next = grid::Grid::new(self.grid.height(), self.grid.width());

        for i in 0..next.width() {
            for j in 0..next.height() {
                next.cells[j][i] = self.apply_rules((i, j))
            }
        }
        self.grid = next.clone();
        self.iteration += 1;

        Some(next)

        // todo!()
    }
}

enum LifeState {
    Live,
    Dead,
}

impl GameOfLifeIterator {
    fn apply_rules(&self, coord: (usize, usize)) -> Option<bool> {
        let live_neighbors = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ]
        .iter()
        .filter_map(|rel_coord: &(i32, i32)| {
            Some((
                match usize::try_from(rel_coord.0 + coord.0 as i32).ok() {
                    // match coord.0.checked_add(rel_coord.0) {
                    Some(x) => x,
                    None => return None,
                },
                match usize::try_from(rel_coord.1 + coord.1 as i32).ok() {
                    // match coord.1.checked_add(rel_coord.1) {
                    Some(x) => x,
                    None => return None,
                },
            ))
        })
        .filter(|coord| coord.0 < self.grid.width() && coord.1 < self.grid.height())
        .filter(|coord| match self.get_livestate(*coord) {
            LifeState::Live => true,
            LifeState::Dead => false,
        })
        .count();

        match self.get_livestate(coord) {
            LifeState::Live => match live_neighbors {
                0 | 1 => Some(false),
                2 | 3 => Some(true),
                4..=8 => Some(false),
                _ => panic!("unknown rule"),
            },
            LifeState::Dead => match live_neighbors {
                3 => Some(true),
                n if (0..=8).contains(&n) => Some(false),
                _ => panic!("unknown rule"),
            },
        }
    }
    fn get_livestate(&self, coord: (usize, usize)) -> LifeState {
        match self.grid[coord] {
            Some(life) => match life {
                true => LifeState::Live,
                false => LifeState::Dead,
            },
            None => panic!("Found uninited cell, {coord:?}"),
        }
    }

    pub(crate) fn init_with_grid(grid: &grid::Grid) -> GameOfLifeIterator {
        let mut init_grid = grid.clone();
        init_grid.cells = init_grid
            .cells
            .iter()
            .enumerate()
            .map(|(j, row)| {
                row.iter()
                    .enumerate()
                    .map(|(i, cell)| match cell {
                        Some(live) => Some(*live),
                        None => Some(rand::random_bool(0.3)),
                    })
                    .collect()
            })
            .collect();

        GameOfLifeIterator {
            iteration: 0,
            grid: init_grid.clone(),
        }
    }
}
