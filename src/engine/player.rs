
use serde::{Serialize, Deserialize};
use crate::deck::Objective;
use crate::buildings::Building;

#[derive(Copy, Clone, Serialize, Deserialize, Debug)]
pub enum Employee {
    Cowboy = 0,
    Craftsman = 1,
    Engineer = 2,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Player {
    dollars: u32,
    hired: [u32; 3],
    buildings: Vec<Building>,
    playedObjectives: Vec<Objective>,
}

impl Player {
    pub fn new(turnPos: u32, playerBuildings: Vec<Building>) -> Player {
        return Player {
            dollars: turnPos + 6,
            hired: [0; 3],
            buildings: playerBuildings,
            playedObjectives: Vec::<Objective>::with_capacity(4),
        };
    }
}
