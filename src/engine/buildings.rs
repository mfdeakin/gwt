
use crate::actions::Action;
use crate::logical::{And, Or, XOr};
use std::fs::File;

#[derive(Copy, Clone)]
pub enum TepeeColor {
    Green,
    Blue,
}

#[derive(Copy, Clone)]
pub struct Tepee {
    color: TepeeColor,
}

#[derive(Copy, Clone)]
pub enum HazardType {
    Flood,
    Drought,
    Rockfall,
}

#[derive(Copy, Clone)]
pub struct Hazard {
    area: HazardType,
    toll: u32,
    points: u32,
}

#[derive(Copy, Clone)]
pub struct Building<'a> {
    // action_options: Or<XOr<u32, 2>, 3>,
    owner: u32,
    toll: u32,
    actions: Or<XOr<And<Action<'a>, 3>, 2>, 3>,
}

impl<'a> Building<'a> {
    pub fn readBuilding(mut input: &File) -> Building<'a> {
        return Building {owner: 0, toll: 0, actions: Or::empty()};
    }
}
