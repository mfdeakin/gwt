mod actions;
mod player;
mod buildings;
mod tracks;
mod logical;
mod deck;

use crate::buildings::Building;
use crate::player::Player;

struct Engine {
    starting_player: u32,
    players: Vec<Player>,
}

impl Engine {
    pub fn startDefaultGame(numPlayers: u32) -> Engine {
        let mut buildings = Vec::<Building>::with_capacity(10);
        let mut players = Vec::<Player>::with_capacity(numPlayers as usize);
        for i in 0..numPlayers {
            players[i as usize] = Player::new(i, buildings.clone());
        }
        return Engine { starting_player: 0, players };
    }
}
