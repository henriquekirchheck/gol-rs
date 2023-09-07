use crate::lifealgo::{Cell, Coords, LifeAlgo};

pub struct GameOfLife {
    width: usize,
    height: usize,
    grid: Vec<Cell>,
}

impl GameOfLife {
    fn get_index_from_coords(&self, coords: Coords) -> usize {
        coords.x + coords.y * self.width
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

    fn get_cell(&self, coords: Coords) -> Cell {
        todo!()
    }

    fn set_cell(&mut self, coords: Coords, new_state: Cell) {
        todo!()
    }

    fn get_next_cell(&self, coords: Coords) -> Cell {
        todo!()
    }

    fn get_current_state(&self) -> Self::Grid {
        todo!()
    }

    fn set_current_state(&mut self, state: Self::Grid) {
        todo!()
    }

    fn get_next_state(&self) -> Self::Grid {
        todo!()
    }

    fn get_population(&self) -> u128 {
        todo!()
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
