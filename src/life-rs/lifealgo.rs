use rand::{distributions::Standard, prelude::Distribution};

#[repr(u8)]
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum Cell {
    #[default]
    Dead,
    Alive,
}

impl Distribution<Cell> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Cell {
        match rng.gen_range(0..=1) {
            0 => Cell::Dead,
            _ => Cell::Alive,
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Coords {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug, Clone)]
pub struct InvalidCoordsError;
#[derive(Debug, Clone)]
pub struct InvalidSizeError;

pub trait LifeAlgo {
    type Grid;

    fn new(width: usize, height: usize) -> Self;
    fn get_size(&self) -> (usize, usize);
    fn get_cell(&self, coords: Coords) -> Result<Cell, InvalidCoordsError>;
    fn set_cell(&mut self, coords: Coords, new_state: Cell) -> Result<(), InvalidCoordsError>;
    fn get_cell_number_neighbours(&self, coords: Coords) -> Result<u8, InvalidCoordsError>;
    fn get_next_cell(&self, coords: Coords) -> Result<Cell, InvalidCoordsError>;
    fn get_state(&self) -> &Self::Grid;
    fn set_state(&mut self, state: Self::Grid) -> Result<(), InvalidSizeError>;
    fn set_state_with(&mut self, state: Cell);
    fn set_state_fn<F>(&mut self, func: F)
    where
        F: Fn(Coords) -> Cell;
    fn get_next_state(&self) -> Self::Grid;
    fn get_population(&self) -> u128;
    fn step(&mut self);
}
