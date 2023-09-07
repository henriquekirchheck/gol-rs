#[repr(u8)]
#[derive(Debug, Clone, Copy, Default)]
pub enum Cell {
    #[default]
    Dead,
    Alive,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Coords {
    pub x: usize,
    pub y: usize
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
    fn get_next_cell(&self, coords: Coords) -> Result<Cell, InvalidCoordsError>;
    fn get_state(&self) -> &Self::Grid;
    fn set_state(&mut self, state: Self::Grid) -> Result<(), InvalidSizeError>;
    fn get_next_state(&self) -> Self::Grid;
    fn get_population(&self) -> u128;
    fn step(&self);
    fn render<T>(&self, renderer: T)
    where
        T: Fn(Coords, bool);
}
