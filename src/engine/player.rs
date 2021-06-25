use crate::actions::Action;
use crate::deck::Objective;
use crate::buildings::Building;

#[derive(Copy, Clone)]
pub enum Employee {
    Cowboy = 0,
    Craftsman = 1,
    Engineer = 2,
}

#[derive(Clone)]
pub struct Player<'a> {
    dollars: u32,
    hired: [u32; 3],
    buildings: Vec<Building<'a>>,
    playedObjectives: Vec<Objective<'a>>,
}

impl<'a> Player<'a> {
    pub fn new(turnPos: u32, playerBuildings: Vec<Building<'a>>) -> Player<'a> {
        return Player {
            dollars: turnPos + 6,
            hired: [0; 3],
            buildings: playerBuildings,
            playedObjectives: Vec::<Objective>::with_capacity(4),
        };
    }
}
