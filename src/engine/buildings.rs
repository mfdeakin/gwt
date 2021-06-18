
use crate::actions::Action;
use crate::logical::{Or, XOr};
use std::fs::File;

pub enum TepeeColor {
    Green,
    Blue,
}

pub struct Tepee {
    color: TepeeColor,
}

pub enum HazardType {
    Flood,
    Drought,
    Rockfall,
}

pub struct Hazard {
    area: HazardType,
    toll: u32,
    points: u32,
}

pub struct Building {
    // action_options: Or<XOr<u32, 2>, 3>,
    owner: u32,
    toll: u32,
}

impl Building {
    pub fn readBuilding(mut input: &File) -> Building {
        return Building {owner: 0, toll: 0};
    }
}
