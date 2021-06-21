
use crate::actions::Action;
use crate::logical::{And, Or, XOr};
use std::fs::File;

pub enum TepeeColor {
    Green,
    Blue,
}

impl Copy for TepeeColor {}
impl Clone for TepeeColor {
    fn clone(&self) -> Self { todo!() }
}

pub struct Tepee {
    color: TepeeColor,
}

impl Copy for Tepee {}
impl Clone for Tepee {
    fn clone(&self) -> Self { todo!() }
}

pub enum HazardType {
    Flood,
    Drought,
    Rockfall,
}

impl Copy for HazardType {}
impl Clone for HazardType {
    fn clone(&self) -> Self { todo!() }
}

pub struct Hazard {
    area: HazardType,
    toll: u32,
    points: u32,
}

impl Copy for Hazard {}
impl Clone for Hazard {
    fn clone(&self) -> Self { todo!() }
}

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
