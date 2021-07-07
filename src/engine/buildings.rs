
use serde::{Serialize, Deserialize};
use crate::actions::{ActionTag, SellCowOpts, ActionValues};
use crate::logical::{And, Or, XOr};
use crate::deck::CowColor;
use crate::player::Employee;

#[derive(Copy, Clone, Serialize, Deserialize, Debug)]
pub enum Tepee {
    Green,
    Blue,
}

#[derive(Copy, Clone, Serialize, Deserialize, Debug)]
pub enum HazardType {
    Flood,
    Drought,
    Rockfall,
}

#[derive(Copy, Clone, Serialize, Deserialize, Debug)]
pub struct Hazard {
    area: HazardType,
    toll: u32,
    points: u32,
}

#[derive(Copy, Clone, Serialize, Deserialize, Debug)]
pub struct Building {
    owner: u32,
    toll: u32,
    actions: Or<XOr<And<ActionTag, 3>, 2>, 3>,
}

impl Building {
    pub fn basicBuilding(num: u32) -> Building {
        let actions = {
            match num {
                0 => Or::new(&[
                    XOr::new(&[And::new(&[
                        ActionTag::SellCow(SellCowOpts::Color(CowColor::Guernsey)),
                        ActionTag::PayCoins(ActionValues::Exact(2)),
                    ]), ]),
                    XOr::new(&[And::new(&[
                        ActionTag::HireEmployee(ActionValues::Exact(0)),
                    ]), ]),
                    XOr::new(&[And::new(&[
                        ActionTag::HireEmployee(ActionValues::Exact(-2)),
                    ]), ]),
                ]),
                1 => Or::new(&[
                    XOr::new(&[And::new(&[
                        ActionTag::SellCow(SellCowOpts::Color(CowColor::Dutch)),
                        ActionTag::PayCoins(ActionValues::Exact(2)),
                    ]), ]),
                    XOr::new(&[And::new(&[
                        ActionTag::PlaceBuilding(ActionValues::EmployeeMult(Employee::Craftsman, -2)),
                    ]), ]),
                ]),
                2 => Or::new(&[
                    XOr::new(&[
                        And::new(&[ActionTag::MoveCertificate(ActionValues::Exact(1)), ]),
                        And::new(&[ActionTag::TakeObjective, ]),
                    ]),
                    XOr::new(&[And::new(&[
                        ActionTag::MoveEngine(ActionValues::EmployeeMult(Employee::Engineer, 1)),
                    ]), ]),
                ]),
                3 => Or::new(&[
                    XOr::new(&[
                        And::new(&[ActionTag::TakeTepee, ]),
                        And::new(&[
                            ActionTag::PayCoins(ActionValues::Exact(-2)),
                            ActionTag::MoveEngine(ActionValues::Exact(2))
                        ]),
                    ]),
                    XOr::new(&[And::new(&[ActionTag::DoubleAuxiliary,
                    ]), ]),
                ]),
                4 => Or::new(&[
                    XOr::new(&[And::new(&[
                        ActionTag::SellCow(SellCowOpts::Color(CowColor::Angus)),
                        ActionTag::PayCoins(ActionValues::Exact(2)),
                    ]), ]),
                    XOr::new(&[And::new(&[
                        ActionTag::BuyCows,
                    ]), ]),
                ]),
                5 => Or::new(&[
                    XOr::new(&[And::new(&[
                        ActionTag::SellCow(SellCowOpts::Pair),
                        ActionTag::PayCoins(ActionValues::Exact(4)),
                    ]), ]),
                    XOr::new(&[And::new(&[
                        ActionTag::PayCoins(ActionValues::Exact(-7)),
                        ActionTag::TakeHazard,
                    ]), ]),
                ]),
                6 => Or::new(&[
                    XOr::new(&[And::new(&[
                        ActionTag::MoveEngine(ActionValues::EmployeeMult(Employee::Engineer, 1)),
                    ]), ]),
                    XOr::new(&[And::new(&[ActionTag::DoubleAuxiliary,
                    ]), ]),
                ]),
                _ => panic!("Invalid building specified"),
            }
        };
        Building { owner: 0, toll: 0, actions }
    }
}
