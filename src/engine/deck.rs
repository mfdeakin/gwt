use serde::{Serialize, Deserialize};
use crate::actions::{ActionTag, ActionValues};
use crate::player::Player;
use crate::logical::And;
use crate::buildings::Tepee::Blue;

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
        Cow { color, points }
    }

    pub fn defaultCowDeck() -> Vec<Cow> {
        vec![
            vec![Cow::new(CowColor::Holstein, 1); 7],
            vec![Cow::new(CowColor::Swiss, 2); 7],
            vec![Cow::new(CowColor::Ayrshire, 3); 7],
            vec![Cow::new(CowColor::Highland, 3); 3],
            vec![Cow::new(CowColor::Highland, 4); 3],
            vec![Cow::new(CowColor::Highland, 5); 3],
            vec![Cow::new(CowColor::Longhorn, 5); 2],
            vec![Cow::new(CowColor::Longhorn, 6); 2],
            vec![Cow::new(CowColor::Longhorn, 7); 2],
        ].into_iter().flatten().collect::<Vec<Cow>>()
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

    pub fn playerStartingDeck() -> Vec<Cow> {
        let mut deck = Vec::from([Cow::new(CowColor::Jersey, 0); 5]);
        deck.append(&mut Vec::from([Cow::new(CowColor::Dutch, 0); 3]));
        deck.append(&mut Vec::from([Cow::new(CowColor::Guernsey, 0); 3]));
        deck.append(&mut Vec::from([Cow::new(CowColor::Angus, 0); 3]));

        return deck;
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
    StationDisc,
    Cow(u32), // The cows value
}

#[derive(Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
pub struct Objective {
    immediate: Option<ActionTag>,
    success_pts: u32,
    fail_pts: u32,
    requirements: And<ObjectiveRequirements, 4>,
}

impl Objective {
    pub fn new(immediate: Option<ActionTag>, success_pts: u32, fail_pts: u32,
           requirements: &[ObjectiveRequirements]) -> Objective {
        Objective { immediate, success_pts, fail_pts, requirements: And::new(requirements) }
    }

    pub fn meetsRequirements(&self, player: Player) -> bool {
        panic!("Not implemented");
    }

    pub fn baseObjectives() -> Vec<Objective> {
        vec![Objective::new(None, 3, 0,
                            &[ObjectiveRequirements::Cow(3), ObjectiveRequirements::Cow(4), ObjectiveRequirements::Building, ]),
             Objective::new(None, 3, 0,
                            &[ObjectiveRequirements::BlueTepee, ObjectiveRequirements::Hazard, ObjectiveRequirements::Hazard, ]),
             Objective::new(None, 3, 0,
                            &[ObjectiveRequirements::StationDisc, ObjectiveRequirements::StationDisc, ObjectiveRequirements::GreenTepee, ]),
             Objective::new(None, 3, 0,
                            &[ObjectiveRequirements::Building, ObjectiveRequirements::Building, ObjectiveRequirements::Hazard, ]),
        ]
    }

    pub fn objectives() -> Vec<Objective> {
        let teleport = Some(ActionTag::TeleportCattleman(ActionValues::Exact(3)));
        let move_eng_2 = Some(ActionTag::MoveEngine(ActionValues::Exact(2)));
        let move_eng_3 = Some(ActionTag::MoveEngine(ActionValues::Exact(3)));
        let dbl_aux = Some(ActionTag::DoubleAuxiliary);
        let take_coins = Some(ActionTag::TakeCoins(ActionValues::Exact(2)));
        let draw_cards = Some(ActionTag::DrawCards(ActionValues::Exact(3)));

        let cow_3 = ObjectiveRequirements::Cow(3);
        let cow_4 = ObjectiveRequirements::Cow(4);
        let cow_5 = ObjectiveRequirements::Cow(5);
        use ObjectiveRequirements::StationDisc;
        use ObjectiveRequirements::GreenTepee;
        use ObjectiveRequirements::BlueTepee;
        use ObjectiveRequirements::Hazard;
        use ObjectiveRequirements::Building;
        use ObjectiveRequirements::SanFran;
        vec![
            Objective::new(dbl_aux, 5, 3, &[SanFran,]),
            Objective::new(dbl_aux, 5, 3, &[SanFran,]),
            Objective::new(dbl_aux, 5, 3, &[SanFran,]),
            Objective::new(dbl_aux, 5, 3, &[SanFran,]),
            Objective::new(draw_cards, 4, 2, &[cow_3, cow_3, cow_3, StationDisc,]),
            Objective::new(take_coins, 4, 2, &[cow_3, cow_3, cow_3, Building,]),
            Objective::new(move_eng_2, 5, 3, &[cow_3, cow_4, cow_5,]),
            Objective::new(teleport, 5, 2, &[cow_3, cow_4, cow_5,]),
            Objective::new(teleport, 5, 2, &[cow_3, cow_4, Hazard, Hazard,]),
            Objective::new(take_coins, 3, 2, &[cow_4, Hazard, Hazard,]),
            Objective::new(move_eng_2, 5, 3, &[cow_4, cow_4, StationDisc, GreenTepee,]),
            Objective::new(draw_cards, 3, 2, &[cow_5, Hazard,]),
            Objective::new(take_coins, 3, 2, &[StationDisc, StationDisc, Hazard,]),
            Objective::new(move_eng_3, 5, 3, &[StationDisc, StationDisc, Hazard, Hazard,]),
            Objective::new(teleport, 5, 2, &[StationDisc, StationDisc, Building, Building,]),
            Objective::new(teleport, 5, 2, &[StationDisc, StationDisc, BlueTepee, BlueTepee,]),
            Objective::new(draw_cards, 3, 2, &[StationDisc, GreenTepee, GreenTepee,]),
            Objective::new(draw_cards, 3, 2, &[StationDisc, GreenTepee, BlueTepee,]),
            Objective::new(draw_cards, 3, 2, &[Building, Building, Hazard,]),
            Objective::new(teleport, 5, 2, &[Building, Building, Hazard, Hazard,]),
            Objective::new(move_eng_2, 5, 3, &[Building, Building, GreenTepee, GreenTepee,]),
            Objective::new(move_eng_3, 5, 3, &[Building, BlueTepee, Hazard, Hazard,]),
            Objective::new(take_coins, 3, 2, &[Building, BlueTepee, BlueTepee,]),
            Objective::new(take_coins, 3, 2, &[Building, GreenTepee, BlueTepee,]),
        ]
    }
}
