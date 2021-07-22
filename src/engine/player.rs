
use serde::{Serialize, Deserialize};
use crate::deck::{Objective, Deck, Cow, Card};
use crate::buildings::{Building, Hazard};
use std::cmp::min;
use crate::deck::Card::CowCard;
use crate::actions::ActionTag;

#[derive(Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
pub enum Employee {
    Cowboy = 0,
    Craftsman = 1,
    Engineer = 2,
}

#[derive(Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
pub struct AuxiliaryAction {
    actions: [ActionTag; 2],
    unlocked: u32,
}

impl AuxiliaryAction {
    pub fn availableActions(&self) -> (Option<ActionTag>, Option<ActionTag>) {
        if self.unlocked == 0 {
            (None, None)
        } else if self.unlocked == 1 {
            (Some(self.actions[0]), None)
        } else {
            (Some(self.actions[0]), Some(self.actions[1]))
        }
    }

    pub fn hasDisc(&self) -> bool {
        self.unlocked < 2
    }

    pub fn unlock(&mut self) {
        assert!(self.unlocked <= 1);
        self.unlocked += 1;
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
pub struct PlayerBoard {

}

#[derive(Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
pub struct Player {
    id: u32,
    dollars: u32,
    cattleManPos: usize,
    hired: [u32; 3],
    deck: Deck,
    playedObjectives: Vec<Objective>,
    playedBuildings: Vec<usize>, // Contains the locations of played buildings on the track
    buildings: Vec<Building>,
    hazards: Vec<Hazard>,
    green_tepees: u32,
    blue_tepees: u32,
}

#[derive(Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
pub struct ObjectiveResources {
    pub buildings: u32,
    pub hazards: u32,
    pub san_fran: u32,
    pub green_tepees: u32,
    pub blue_tepees: u32,
    pub station_discs: u32,
    pub ryb_cows: u32,
    pub brown_cows: u32,
    pub purple_cows: u32,
}

impl Player {
    pub fn new(turnPos: u32, startingDeck: Vec<Cow>, playerBuildings: Vec<Building>) -> Player {
        let startingDeck: Vec<Card> = startingDeck.iter()
            .map(|cow| { CowCard(*cow) })
            .collect();
        return Player {
            id: turnPos,
            dollars: turnPos + 6,
            cattleManPos: 0,
            hired: [0; 3],
            deck: Deck::new(4, startingDeck),
            playedObjectives: Vec::<Objective>::with_capacity(4),
            playedBuildings: Vec::with_capacity(playerBuildings.len()),
            buildings: playerBuildings,
            hazards: Vec::<Hazard>::new(),
            green_tepees: 0,
            blue_tepees: 0,
        };
    }

    pub fn id(&self) -> u32 { self.id }

    pub fn tepeePairs(&self) -> u32 {
        min(self.green_tepees, self.blue_tepees)
    }

}
