
mod actions;
mod player;
mod buildings;
mod tracks;
mod logical;
mod deck;

use crate::buildings::Building;
use crate::player::Player;

struct Engine {
    starting_player: u8,
    players: Vec<Player>,
}
