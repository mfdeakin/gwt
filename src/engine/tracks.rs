use serde::{Deserialize, Serialize};

use crate::actions::{ActionTag, ActionValues};
use crate::buildings::{Building, Hazard, HazardType, Tepee};
use crate::logical::And;

#[derive(Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
pub enum SpaceOccupant {
    Building { risk_action: And<ActionTag, 3>, building: Option<Building>, neutral: bool, forest: bool },
    Hazard(HazardType, Option<Hazard>),
    Tepee(i32, Option<Tepee>),
    KansasCity,
    Start,
}

#[derive(Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
pub struct Space {
    occupant: SpaceOccupant,
    nextSpace: [Option<usize>; 2],
}

impl Space {
    pub fn new(occupant: SpaceOccupant, nextSpace: [Option<usize>; 2]) -> Space {
        return Space { occupant, nextSpace };
    }
}

#[derive(Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
pub struct City {
    location: u32,
    points: i32,
    coins: u32,
    advanced_disc: bool,
    limited: bool,
    placed_discs: [u32; 4],
    name: String,
}

#[derive(Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
pub struct PairAction {
    take_obj: bool,
    points: i32,
}

#[derive(Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
pub struct CityTrack {
    cities: Vec<City>,
    pair_action: Vec<PairAction>,
}

#[derive(Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
pub struct StationMaster {
    points: ActionValues,
    bonus: crate::logical::XOr<ActionTag, 2>,
    perm_cert: bool,
}

#[derive(Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
pub struct Station {
    location: u32,
    price: i32,
    points: u32,
    advanced_disc: bool,
    placed_discs: [bool; 4],
    station_master: Option<StationMaster>,
}

#[derive(Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
pub enum EngineSpace {
    TurnoutTrack(usize),
    MainTrack(usize),
}

#[derive(Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
pub struct StationTrack {
    end: u32,
    engine_loc: [EngineSpace; 4],
    final_station: Station,
    stations: Vec<Station>,
    crossings: Vec<u32>,
    cities: CityTrack,
}

impl StationTrack {
    pub fn moveEngine(&mut self, player: u32, space: EngineSpace) -> Option<(ActionTag, ActionTag)> {
        self.engine_loc[player as usize] = space;
        if space == EngineSpace::MainTrack(self.end as usize) {
            Some((ActionTag::MoveEngine(ActionValues::AtMost(-1)), ActionTag::TakeCoins(ActionValues::Exact(3))))
        } else {
            None
        }
    }

    pub fn makeDelivery(&mut self, player: u32, city: u32) {

    }
}
