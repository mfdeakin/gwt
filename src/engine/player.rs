
use serde::{Serialize, Deserialize};
use crate::deck::Objective;
use crate::buildings::{Building, Hazard};

#[derive(Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
pub enum Employee {
    Cowboy = 0,
    Craftsman = 1,
    Engineer = 2,
}

#[derive(Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
pub struct Player {
    dollars: u32,
    cattleManPos: usize,
    hired: [u32; 3],
    buildings: Vec<Building>,
    playedObjectives: Vec<Objective>,
    hazards: Vec<Hazard>,
    green_tepees: u32,
    blue_tepees: u32,
}

impl Player {
    pub fn new(turnPos: u32, playerBuildings: Vec<Building>) -> Player {
        return Player {
            dollars: turnPos + 6,
            cattleManPos: 0,
            hired: [0; 3],
            buildings: playerBuildings,
            playedObjectives: Vec::<Objective>::with_capacity(4),
            hazards: Vec::<Hazard>::new(),
            green_tepees: 0,
            blue_tepees: 0,
        };
    }
}
