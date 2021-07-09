
use serde::{Serialize, Deserialize};
use crate::actions::ActionTag;
use crate::player::Player;

#[derive(Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
pub enum Card {
    Cow(Cow),
    Objective(Objective),
}

#[derive(Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
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

#[derive(Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
pub struct Cow {
    color: CowColor,
    points: u32,
}

impl Cow {
    pub fn new(color: CowColor, points: u32) -> Cow {
        Cow {color, points}
    }

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

#[derive(Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
pub struct CowMarket {
    cow_deck: Vec<Cow>,
    ryb_market: Vec<Cow>,
    brown_market: Vec<Cow>,
    purple_market: Vec<Cow>,
}

impl CowMarket {
    pub fn new() -> CowMarket {
        let cow_deck = vec![Cow::new(CowColor::Holstein, 3)];
        let ryb_market = Vec::<Cow>::new();
        let brown_market = Vec::<Cow>::new();
        let purple_market = Vec::<Cow>::new();
        CowMarket { cow_deck: cow_deck, ryb_market: ryb_market, brown_market: brown_market, purple_market: purple_market }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
pub enum ObjectiveRequirements {
    Building,
    Hazard,
    SanFran,
    GreenTepee,
    BlueTepee,
    PlacedDisc,
    Cow(u32),
}

#[derive(Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
pub struct Objective {
    immediate: ActionTag,
    success_pts: u32,
    fail_pts: u32,
    requirements: [Option<ObjectiveRequirements>; 4],
}

impl Objective {
    fn meets_requirements(&self, player: Player) -> bool {
        panic!("Not implemented");
    }
}
