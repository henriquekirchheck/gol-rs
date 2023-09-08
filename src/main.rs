use life_rs::{algorithms::gol::GameOfLife, lifealgo::LifeAlgo};

fn main() {
    let mut gol = GameOfLife::new(318, 135);
    gol.set_state_fn(|_| rand::random());
    loop {
        println!("{gol:?}");
        gol.step();
    }
}
