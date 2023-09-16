use std::fmt::{Debug, Display};

use itertools::Itertools;
use rayon::prelude::{ParallelBridge, ParallelIterator};

use crate::lifealgo::{Cell, Coords, InvalidCoordsError, InvalidSizeError, LifeAlgo};

pub struct GameOfLife {
    width: usize,
    height: usize,
    grid: Vec<Cell>,
}

impl GameOfLife {
    fn get_index_from_coords(&self, coords: Coords) -> Result<usize, InvalidCoordsError> {
        if coords.x >= self.width || coords.y >= self.height {
            Err(InvalidCoordsError)
        } else {
            Ok(coords.x + coords.y * self.width)
        }
    }

    fn display_terminal(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                let cell = self.grid[self
                    .get_index_from_coords(Coords { x, y })
                    .expect("what the fuck?")];
                let symbol = match cell {
                    Cell::Alive => "#",
                    Cell::Dead => " ",
                };
                write!(f, "{}", symbol)?;
                if x != self.width - 1 {
                    write!(f, " ")?;
                }
            }
            if y != self.height - 1 {
                write!(f, "\n")?;
            }
        }

        Ok(())
    }
}

impl LifeAlgo for GameOfLife {
    fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            grid: vec![Cell::default(); width * height],
        }
    }

    fn get_size(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    fn get_cell(&self, coords: Coords) -> Result<&Cell, InvalidCoordsError> {
        self.grid
            .get(self.get_index_from_coords(coords)?)
            // .map(ToOwned::to_owned)
            .ok_or(InvalidCoordsError)
    }

    fn set_cell(&mut self, coords: Coords, new_state: Cell) -> Result<(), InvalidCoordsError> {
        let index = self.get_index_from_coords(coords)?;
        let cell = self.grid.get_mut(index).ok_or(InvalidCoordsError)?;
        *cell = new_state;
        Ok(())
    }

    fn get_cell_number_neighbours(&self, coords: Coords) -> Result<u8, InvalidCoordsError> {
        (-1..=1)
            .cartesian_product(-1..=1)
            .par_bridge()
            .map(|(offset_x, offset_y)| -> Result<u8, InvalidCoordsError> {
                if offset_x == 0 && offset_y == 0 {
                    Ok(0)
                } else {
                    let neighbour_coords = Coords {
                        x: wrap_around(offset_x + coords.x as isize, self.width as isize),
                        y: wrap_around(offset_y + coords.y as isize, self.height as isize),
                    };
                    let neighbour = self
                        .grid
                        .get(self.get_index_from_coords(neighbour_coords)?)
                        .ok_or(InvalidCoordsError)?;

                    Ok(*neighbour as u8)
                }
            })
            .sum::<Result<u8, InvalidCoordsError>>()
    }

    fn get_next_cell(&self, coords: Coords) -> Result<Cell, InvalidCoordsError> {
        let cell = self
            .grid
            .get(self.get_index_from_coords(coords)?)
            .ok_or(InvalidCoordsError)?;
        let neighbours = self.get_cell_number_neighbours(coords)?;

        match (neighbours == 3) || (neighbours == 2 && *cell == Cell::Alive) {
            true => Ok(Cell::Alive),
            false => Ok(Cell::Dead),
        }
    }

    fn get_state(&self) -> impl Iterator<Item = (Coords, &Cell)> {
        (0..self.width)
            .cartesian_product(0..self.height)
            .map(|(x, y)| Coords { x, y })
            .map(|coords| (coords, self.get_cell(coords).expect("what the fuck?")))
    }

    fn set_state(
        &mut self,
        state: impl Iterator<Item = (Coords, Cell)> + Clone,
    ) -> Result<(), InvalidSizeError> {
        if state.clone().count() != self.width * self.height {
            Err(InvalidSizeError)
        } else {
            for (coords, cell) in state {
                self.set_cell(coords, cell).expect("what the fuck?");
            }
            Ok(())
        }
    }

    fn get_next_state(&self) -> impl Iterator<Item = (Coords, Cell)> {
        (0..self.width)
            .cartesian_product(0..self.height)
            .par_bridge()
            .map(|(x, y)| Coords { x, y })
            .map(|coords| (coords, self.get_next_cell(coords).expect("what the fuck?")))
            .collect::<Vec<_>>()
            .into_iter()
    }

    fn get_population(&self) -> u128 {
        self.grid.iter().map(|x| *x as u8 as u128).sum()
    }

    fn step(&mut self) {
        (0..self.width)
            .cartesian_product(0..self.height)
            .par_bridge()
            .map(|(x, y)| Coords { x, y })
            .map(|coords| {
                (
                    self.get_index_from_coords(coords).expect("what the fuck?"),
                    self.get_next_cell(coords).expect("what the fuck?"),
                )
            })
            .collect::<Vec<_>>()
            .into_iter()
            .for_each(|(index, cell)| {
                self.grid[index] = cell;
            });
    }

    fn set_state_with(&mut self, state: Cell) {
        for index in 0..self.grid.len() {
            self.grid[index] = state
        }
    }

    fn set_state_fn<F>(&mut self, func: F)
    where
        F: Fn(Coords) -> Cell,
    {
        let coordinates = (0..self.width)
            .cartesian_product(0..self.height)
            .par_bridge()
            .map(|(x, y)| Coords { x, y })
            .map(|coords| {
                (
                    self.get_index_from_coords(coords).expect("what the fuck?"),
                    coords,
                )
            })
            .collect::<Vec<_>>();

        for (index, coords) in coordinates {
            self.grid[index] = func(coords);
        }
    }
}

impl Debug for GameOfLife {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Dimentions: {}x{}", self.width, self.height)?;
        self.display_terminal(f)?;
        Ok(())
    }
}
impl Display for GameOfLife {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.display_terminal(f)?;
        Ok(())
    }
}

fn wrap_around(index: isize, num: isize) -> usize {
    ((index % num + num) % num) as usize
}
