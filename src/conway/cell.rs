use std::fmt;

#[derive(Clone, Debug)]
pub struct Coordinate {
    pub x: i32,
    pub y: i32,
}

impl Coordinate {
    pub fn new(x: i32, y: i32) -> Self {
        Coordinate { x: x, y: y }
    }
    pub fn calculate_adjecents(&self) -> Vec<Coordinate> {
        let x_sub1 = self.x - 1;
        let y_sub1 = self.y - 1;
        let mut adj: Vec<Coordinate> = Vec::new();
        for i in x_sub1..=x_sub1 + 2 {
            for j in y_sub1..=y_sub1 + 2 {
                if !(i == self.x && j == self.y) && i >= 0 && j >= 0 {
                    adj.push(Coordinate { x: i, y: j });
                }
            }
        }
        adj
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CellState {
    Dead,
    Alive,
}

pub trait Cell {
    fn new(location: Coordinate) -> Self;
    fn is_alive(&self) -> bool;
    fn set_state(&mut self, state: CellState);
    fn get_adjecent(&self) -> Vec<Coordinate>;
}

#[derive(Clone, Debug)]
pub struct SimpleCell {
    pub state: CellState,
    pub location: Coordinate,
    adjecent: Vec<Coordinate>,
}

impl SimpleCell {}

impl Cell for SimpleCell {
    fn new(location: Coordinate) -> Self {
        let adjecent = location.calculate_adjecents();
        Self {
            state: CellState::Dead,
            location: location,
            adjecent: adjecent,
        }
    }
    fn is_alive(&self) -> bool {
        self.state == CellState::Alive
    }
    fn set_state(&mut self, state: CellState) {
        self.state = state;
    }
    fn get_adjecent(&self) -> Vec<Coordinate> {
        self.adjecent.clone()
    }
}

impl fmt::Display for SimpleCell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Cell ({}, {}): {:?}",
            self.location.x, self.location.y, self.state
        )
    }
}
