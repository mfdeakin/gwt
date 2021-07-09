#![allow(non_snake_case)]

pub mod actions;
pub mod player;
pub mod buildings;
pub mod tracks;
pub mod logical;
pub mod deck;

use crate::buildings::Building;
use crate::player::Player;
use crate::deck::{Objective, CowMarket};
use crate::tracks::SpaceOccupant;
use std::fs::read_to_string;
use std::path::Path;

struct Engine {
    starting_player: u32,
    players: Vec<Player>,
    cows: CowMarket,
    objectives: Vec<Objective>,
    track: Vec<SpaceOccupant>,
}

impl Engine {
    pub fn startDefaultGame(numPlayers: u32) -> Engine {
        let buildings = Vec::<Building>::with_capacity(10);
        let mut players = Vec::<Player>::with_capacity(numPlayers as usize);
        for i in 0..numPlayers {
            players[i as usize] = Player::new(i, buildings.clone());
        }
        let track_path = Path::new("data/default_track.json");
        let track_str = read_to_string(track_path).unwrap();
        let track : Vec<SpaceOccupant> = serde_json::from_str(&track_str).unwrap();
        return Engine { starting_player: 0, players: players, cows: CowMarket::new(), objectives: Vec::<Objective>::new(), track: track };
    }
}
