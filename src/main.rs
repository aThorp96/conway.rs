use std::{thread, time};

mod conway;

use conway::board::Board;
use conway::conway::CellState;

fn sleep(time_ms: usize) {
    let sleep_time = time::Duration::from_millis(time_ms as u64);
    thread::sleep(sleep_time);
}

fn main() {
    let mut b = Board::new(10, 10);

    b.set(0, 0, CellState::Alive);
    b.set(1, 1, CellState::Alive);
    b.set(1, 2, CellState::Alive);
    b.set(2, 0, CellState::Alive);
    b.set(2, 1, CellState::Alive);

    for i in 0..50 {
        println!("Board:\n{}", b);
        b.step();
        sleep(100);
    }
}
