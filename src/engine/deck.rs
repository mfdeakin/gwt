
use serde::{Serialize, Deserialize};
use crate::actions::{Action, ActionTag};
use crate::player::Player;
use crate::logical::And;

pub enum Card {
    Cow(Cow),
    Objective(Objective),
}

#[derive(Copy, Clone, Serialize, Deserialize, Debug)]
pub enum CowColor {
    Jersey,
    Dutch,
    Angus,
    Guernsey,
    Holstein,
    Swiss,
    Ayrshire,
    Highland,
    Longhorn,
}

#[derive(Copy, Clone, Serialize, Deserialize, Debug)]
pub struct Cow {
    color: CowColor,
    points: u32,
}

impl Cow {
    pub fn value(&self) -> u32 {
        match self.color {
            CowColor::Jersey => 1,
            CowColor::Dutch => 2,
            CowColor::Angus => 2,
            CowColor::Guernsey => 2,
            CowColor::Holstein => 3,
            CowColor::Swiss => 3,
            CowColor::Ayrshire => 3,
            CowColor::Highland => 4,
            CowColor::Longhorn => 5,
        }
    }
}

pub enum ObjectiveRequirements {
    Building,
    Hazard,
    SanFran,
    GreenTepee,
    BlueTepee,
    PlacedDisc,
    Cow(u32),
}

#[derive(Copy, Clone, Serialize, Deserialize, Debug)]
pub struct Objective {
    immediate: ActionTag,
    success_pts: u32,
    fail_pts: u32,
}

impl Objective {
    fn meets_requirements(&self, player: Player) -> bool {
        panic!("Not implemented");
    }
}
