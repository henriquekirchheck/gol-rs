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
}

impl LifeAlgo for GameOfLife {
    type Grid = Vec<Cell>;

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

    fn get_cell(&self, coords: Coords) -> Result<Cell, InvalidCoordsError> {
        self.grid
            .get(self.get_index_from_coords(coords)?)
            .map(ToOwned::to_owned)
            .ok_or(InvalidCoordsError)
    }

    fn set_cell(&mut self, coords: Coords, new_state: Cell) -> Result<(), InvalidCoordsError> {
        let index = self.get_index_from_coords(coords)?;
        let cell = self.grid.get_mut(index).ok_or(InvalidCoordsError)?;
        *cell = new_state;
        Ok(())
    }

    fn get_next_cell(&self, coords: Coords) -> Result<Cell, InvalidCoordsError> {
        todo!()
    }

    fn get_state(&self) -> &Self::Grid {
        &self.grid
    }

    fn set_state(&mut self, state: Self::Grid) -> Result<(), InvalidSizeError> {
        if state.len() != self.width * self.height {
            Err(InvalidSizeError)
        } else {
            // self.grid;
            Ok(())
        }
    }

    fn get_next_state(&self) -> Self::Grid {
        todo!()
    }

    fn get_population(&self) -> u128 {
        self.grid.iter().map(|x| *x as u8 as u128).sum()
    }

    fn step(&self) {
        todo!()
    }

    fn render<T>(&self, renderer: T)
    where
        T: Fn(Coords, bool),
    {
        todo!()
    }
}
