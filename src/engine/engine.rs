#![allow(non_snake_case)]

pub mod actions;
pub mod player;
pub mod buildings;
pub mod tracks;
pub mod logical;
pub mod deck;
pub mod tiles;

use serde::{Serialize, Deserialize};
use crate::buildings::Building;
use crate::player::Player;
use crate::deck::{Objective, CowMarket, Cow};
use crate::tracks::SpaceOccupant;
use std::path::Path;
use std::fs::read_to_string;

#[derive(Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
pub struct Engine {
    starting_player: usize,
    current_player: usize,
    players: Vec<Player>,
    cows: CowMarket,
    objectives: Vec<Objective>,
    track: Vec<SpaceOccupant>,
}

impl Engine {
    pub fn startDefaultGame(numPlayers: u32) -> Engine {
        let buildings = Vec::<Building>::with_capacity(10);
        let mut players = Vec::<Player>::with_capacity(numPlayers as usize);
        let starting_deck_path = Path::new("data/default_track.json");
        let starting_deck_str = read_to_string(starting_deck_path).unwrap();
        let starting_deck: Vec<Cow> = serde_json::from_str(&starting_deck_str).unwrap();
        for i in 0..numPlayers {
            players[i as usize] = Player::new(i, starting_deck.clone(), buildings.clone());
        }
        let track_path = Path::new("data/default_track.json");
        let track_str = read_to_string(track_path).unwrap();
        let track : Vec<SpaceOccupant> = serde_json::from_str(&track_str).unwrap();
        return Engine { starting_player: 0, current_player: 0, players, cows: CowMarket::new(), objectives: Vec::<Objective>::new(), track };
    }
}
