
use serde::{Serialize, Deserialize};
use crate::buildings::{Building, Hazard, Tepee, HazardType};
use crate::logical::And;
use crate::actions::{ActionTag, SellCowOpts, ActionValues};
use crate::deck::CowColor;

#[derive(Copy, Clone, Serialize, Deserialize, Debug)]
pub enum SpaceOccupant {
    Building(And<ActionTag, 3>, Option<Building>),
    Hazard(HazardType, Option<Hazard>),
    Tepee(i32, Option<Tepee>),
    KansasCity,
    Start,
}

#[derive(Copy, Clone, Serialize, Deserialize, Debug)]
pub struct Space {
    occupant: SpaceOccupant,
    nextSpace: [Option<usize>; 2],
}

impl Space {
    pub fn new(occupant: SpaceOccupant, nextSpace: [Option<usize>; 2]) -> Space {
        return Space { occupant, nextSpace };
    }
}

pub fn defaultTrack() -> Vec<Space> {
     let special1 : And<ActionTag, 3> = And::new(&[ActionTag::SellCow(SellCowOpts::Color(CowColor::Jersey)), ActionTag::MoveCertificate(ActionValues::Exact(1)), ActionTag::PayCoins(ActionValues::Exact(2)),]);
     let special2 : And<ActionTag, 3> = And::new(&[ActionTag::SellCow(SellCowOpts::Any), ActionTag::MoveCertificate(ActionValues::Exact(1)),]);
    // 51 space track
    vec![Space::new(SpaceOccupant::Start, [Some(1), None]),                                                               //  0
         Space::new(SpaceOccupant::Building(And::empty(), Some(Building::basicBuilding(0))), [Some(2), Some(5)]),   //  1 A
         Space::new(SpaceOccupant::Building(And::empty(), None), [Some(3), None]),                                        //  2
         Space::new(SpaceOccupant::Building(And::empty(), None), [Some(4), None]),                                        //  3
         Space::new(SpaceOccupant::Building(And::empty(), None), [Some(11), None]),                                       //  4
         Space::new(SpaceOccupant::Hazard(HazardType::Flood, None), [Some(6), None]),                                     //  5
         Space::new(SpaceOccupant::Hazard(HazardType::Flood, None), [Some(7), None]),                                     //  6
         Space::new(SpaceOccupant::Hazard(HazardType::Flood, None), [Some(8), None]),                                     //  7
         Space::new(SpaceOccupant::Hazard(HazardType::Flood, None), [Some(9), None]),                                     //  8
         Space::new(SpaceOccupant::Building(special1, None), [Some(10), None]),                                           //  9   Special 1
         Space::new(SpaceOccupant::Building(special1, None), [Some(11), None]),                                           // 10   Special 1
         Space::new(SpaceOccupant::Building(And::empty(), Some(Building::basicBuilding(1))), [Some(12), Some(15)]), // 11 B
         Space::new(SpaceOccupant::Building(And::empty(), None), [Some(13), None]),                                       // 12
         Space::new(SpaceOccupant::Building(And::empty(), None), [Some(14), None]),                                       // 13
         Space::new(SpaceOccupant::Building(And::empty(), None), [Some(20), None]),                                       // 14
         Space::new(SpaceOccupant::Hazard(HazardType::Drought, None), [Some(16), None]),                                  // 15
         Space::new(SpaceOccupant::Hazard(HazardType::Drought, None), [Some(17), None]),                                  // 16
         Space::new(SpaceOccupant::Hazard(HazardType::Drought, None), [Some(18), None]),                                  // 17
         Space::new(SpaceOccupant::Hazard(HazardType::Drought, None), [Some(19), None]),                                  // 18
         Space::new(SpaceOccupant::Building(special1, None), [Some(20), None]),                                           // 19   Special 2
         Space::new(SpaceOccupant::Building(And::empty(), Some(Building::basicBuilding(2))), [Some(21), Some(23)]), // 20 C
         Space::new(SpaceOccupant::Building(And::empty(), None), [Some(22), None]),                                       // 21
         Space::new(SpaceOccupant::Building(And::empty(), None), [Some(33), None]),                                       // 22
         Space::new(SpaceOccupant::Building(And::empty(), None), [Some(24), Some(25)]),                                   // 23
         Space::new(SpaceOccupant::Building(And::empty(), Some(Building::basicBuilding(3))), [Some(33), None]),     // 24 D
         Space::new(SpaceOccupant::Tepee(1, None), [Some(26), None]),                                                     // 25
         Space::new(SpaceOccupant::Tepee(2, None), [Some(27), None]),                                                     // 26
         Space::new(SpaceOccupant::Tepee(4, None), [Some(28), None]),                                                     // 27
         Space::new(SpaceOccupant::Tepee(6, None), [Some(29), None]),                                                     // 28
         Space::new(SpaceOccupant::Tepee(8, None), [Some(30), None]),                                                     // 29
         Space::new(SpaceOccupant::Tepee(10, None), [Some(31), None]),                                                    // 30
         Space::new(SpaceOccupant::Building(special2, None), [Some(32), None]),                                           // 31   Special 2
         Space::new(SpaceOccupant::Building(special1, None), [Some(33), None]),                                           // 32   Special 1
         Space::new(SpaceOccupant::Building(And::empty(), Some(Building::basicBuilding(4))), [Some(34), Some(36)]), // 33 E
         Space::new(SpaceOccupant::Building(And::empty(), None), [Some(35), None]),                                       // 34
         Space::new(SpaceOccupant::Building(And::empty(), None), [Some(42), None]),                                       // 35
         Space::new(SpaceOccupant::Hazard(HazardType::Rockfall, None), [Some(37), None]),                                 // 36
         Space::new(SpaceOccupant::Hazard(HazardType::Rockfall, None), [Some(38), None]),                                 // 37
         Space::new(SpaceOccupant::Hazard(HazardType::Rockfall, None), [Some(39), None]),                                 // 38
         Space::new(SpaceOccupant::Hazard(HazardType::Rockfall, None), [Some(40), None]),                                 // 39
         Space::new(SpaceOccupant::Building(special2, None), [Some(41), None]),                                           // 40   Special 2
         Space::new(SpaceOccupant::Building(special1, None), [Some(42), None]),                                           // 41   Special 1
         Space::new(SpaceOccupant::Building(And::empty(), Some(Building::basicBuilding(5))), [Some(43), Some(44)]), // 42 F
         Space::new(SpaceOccupant::Building(And::empty(), None), [Some(45), None]),                                       // 43
         Space::new(SpaceOccupant::Building(And::empty(), None), [Some(45), None]),                                       // 44
         Space::new(SpaceOccupant::Building(And::empty(), Some(Building::basicBuilding(6))), [Some(46), Some(47)]), // 45 G
         Space::new(SpaceOccupant::Building(And::empty(), None), [Some(48), None]),                                       // 46
         Space::new(SpaceOccupant::Building(And::empty(), None), [Some(48), None]),                                       // 47
         Space::new(SpaceOccupant::KansasCity, [None, None]),                                                             // 48
         Space::new(SpaceOccupant::Tepee(-3, None), [None, None]),                                                        // 49
         Space::new(SpaceOccupant::Tepee(-2, None), [None, None]),                                                        // 50
         Space::new(SpaceOccupant::Tepee(-1, None), [None, None]),                                                        // 51
    ]

}
