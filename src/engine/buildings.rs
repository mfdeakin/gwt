
use serde::{Serialize, Deserialize};
use crate::actions::{ActionTag, DiscardCardOpts, ActionValues};
use crate::logical::{And, Or, XOr};
use crate::deck::CowColor;
use crate::player::Employee;

#[derive(Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
pub enum Toll {
    NoToll,
    Green,
    Black,
    GreenBlack,
}

#[derive(Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
pub enum Tepee {
    Green,
    Blue,
}

#[derive(Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
pub enum HazardType {
    Flood,
    Drought,
    Rockfall,
}

#[derive(Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
pub struct Hazard {
    area: HazardType,
    toll: Toll,
    points: u32,
}

#[derive(Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
pub struct Building {
    owner: u32,
    laborers: u32,
    points: u32,
    toll: Toll,
    actions: Or<XOr<And<ActionTag, 3>, 2>, 3>,
}

impl Building {
    pub fn basicBuilding(num: u32) -> Building {
        let actions = {
            match num {
                0 => Or::new(&[
                    XOr::new(&[And::new(&[
                        ActionTag::DiscardCard(DiscardCardOpts::Color(CowColor::Guernsey)),
                        ActionTag::TakeCoins(ActionValues::Exact(2)),
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
                        ActionTag::DiscardCard(DiscardCardOpts::Color(CowColor::Dutch)),
                        ActionTag::TakeCoins(ActionValues::Exact(2)),
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
                            ActionTag::TakeCoins(ActionValues::Exact(-2)),
                            ActionTag::MoveEngine(ActionValues::Exact(2))
                        ]),
                    ]),
                    XOr::new(&[And::new(&[ActionTag::DoubleAuxiliary,
                    ]), ]),
                ]),
                4 => Or::new(&[
                    XOr::new(&[And::new(&[
                        ActionTag::DiscardCard(DiscardCardOpts::Color(CowColor::Angus)),
                        ActionTag::TakeCoins(ActionValues::Exact(2)),
                    ]), ]),
                    XOr::new(&[And::new(&[
                        ActionTag::BuyCows,
                    ]), ]),
                ]),
                5 => Or::new(&[
                    XOr::new(&[And::new(&[
                        ActionTag::DiscardCard(DiscardCardOpts::PairCow),
                        ActionTag::TakeCoins(ActionValues::Exact(4)),
                    ]), ]),
                    XOr::new(&[And::new(&[
                        ActionTag::TakeCoins(ActionValues::Exact(-7)),
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
        Building { owner: 0, laborers: 0, points: 0, toll: Toll::NoToll, actions }
    }

    pub fn playerBuilding(num: u32, side_b: bool) -> Building {
        let actions =
            {
                if !side_b {
                    match num {
                        0 => Or::new(&[
                            XOr::new(&[And::new(&[
                                ActionTag::TakeCoins(ActionValues::ForestMult(2)),
                            ]), ]),
                        ]),
                        1 => Or::new(&[
                            XOr::new(&[And::new(&[
                                ActionTag::DiscardCard(DiscardCardOpts::Color(CowColor::Guernsey)),
                                ActionTag::TakeCoins(ActionValues::Exact(4)),
                            ]), ]),
                            XOr::new(&[And::new(&[
                                ActionTag::BuyCows,
                            ]), ]),
                        ]),
                        2 => Or::new(&[
                            XOr::new(&[And::new(&[
                                ActionTag::DiscardCard(DiscardCardOpts::PairCow),
                                ActionTag::TakeCoins(ActionValues::Exact(3)),
                            ]), ]),
                            XOr::new(&[And::new(&[
                                ActionTag::MoveCattleman(ActionValues::Exact(1)),
                            ]), ]),
                        ]),
                        3 => Or::new(&[
                            XOr::new(&[And::new(&[
                                ActionTag::TakeCoins(ActionValues::Exact(-5)),
                                ActionTag::TakeHazard,
                            ]), ]),
                            XOr::new(&[And::new(&[
                                ActionTag::MoveCattleman(ActionValues::Exact(2)),
                            ]), ]),
                        ]),
                        4 => Or::new(&[
                            XOr::new(&[And::new(&[
                                ActionTag::HireEmployee(ActionValues::Exact(1)),
                            ]), ]),
                            XOr::new(&[And::new(&[
                                ActionTag::MoveEngine(ActionValues::EmployeeMult(Employee::Engineer, 1)),
                            ]), ]),
                        ]),
                        5 => Or::new(&[
                            XOr::new(&[And::new(&[
                                ActionTag::DiscardCard(DiscardCardOpts::Color(CowColor::Holstein)),
                                ActionTag::TakeCoins(ActionValues::Exact(10)),
                            ]), ]),
                            XOr::new(&[And::new(&[
                                ActionTag::DoubleAuxiliary,
                            ]), ]),
                        ]),
                        6 => Or::new(&[
                            XOr::new(&[And::new(&[
                                ActionTag::MoveCertificate(ActionValues::TepeePairMult(2)),
                                ActionTag::TakeCoins(ActionValues::TepeePairMult(2)),
                            ]), ]),
                        ]),
                        7 => Or::new(&[
                            XOr::new(&[And::new(&[
                                ActionTag::TakeTepee,
                            ]), And::new(&[
                                ActionTag::DoubleAuxiliary,
                            ]), ]),
                            XOr::new(&[And::new(&[
                                ActionTag::MoveEngine(ActionValues::Exact(2)),
                            ]), ]),
                        ]),
                        8 => Or::new(&[
                            XOr::new(&[And::new(&[
                                ActionTag::MoveEngine(ActionValues::Exact(3)),
                            ]), ]),
                            XOr::new(&[And::new(&[
                                ActionTag::CityDiscMoveTrain,
                            ]), ]),
                        ]),
                        9 => Or::new(&[
                            XOr::new(&[And::new(&[
                                ActionTag::MoveCertificate(ActionValues::Max),
                            ]), ]),
                            XOr::new(&[And::new(&[
                                ActionTag::MoveCattleman(ActionValues::Exact(5)),
                            ]), ]),
                        ]),
                        _ => panic!("Invalid building specified")
                    }
                }
                else {
                    match num {
                        0 => Or::new(&[
                            XOr::new(&[And::new(&[
                                ActionTag::DiscardCard(DiscardCardOpts::Objective),
                                ActionTag::MoveCertificate(ActionValues::Exact(2)),
                            ]), ]),
                            XOr::new(&[And::new(&[
                                ActionTag::MoveEngine(ActionValues::Exact(-1)),
                                ActionTag::TakeCoins(ActionValues::Exact(3)),
                            ]), ]),
                        ]),
                        1 => Or::new(&[
                            XOr::new(&[And::new(&[
                                ActionTag::DiscardCard(DiscardCardOpts::Color(CowColor::Jersey)),
                                ActionTag::MoveEngine(ActionValues::Exact(1)),
                            ]), ]),
                            XOr::new(&[And::new(&[
                                ActionTag::DiscardCard(DiscardCardOpts::Color(CowColor::Dutch)),
                                ActionTag::TakeCoins(ActionValues::Exact(3)),
                            ]), ]),
                        ]),
                        2 => Or::new(&[
                            XOr::new(&[And::new(&[
                                ActionTag::DoubleAuxiliary,
                            ]), ]),
                            XOr::new(&[And::new(&[
                                ActionTag::MoveCattleman(ActionValues::Exact(1)),
                            ]), ]),
                        ]),
                        3 => Or::new(&[
                            XOr::new(&[And::new(&[
                                ActionTag::DrawCards(ActionValues::EmployeeMult(Employee::Cowboy, 1)),
                            ]), ]),
                            XOr::new(&[And::new(&[
                                ActionTag::MoveCattleman(ActionValues::Exact(3)),
                            ]), ]),
                        ]),
                        4 => Or::new(&[
                            XOr::new(&[And::new(&[
                                ActionTag::DiscardCard(DiscardCardOpts::Color(CowColor::Angus)),
                                ActionTag::MoveCertificate(ActionValues::Exact(2)),
                            ]), ]),
                            XOr::new(&[And::new(&[
                                ActionTag::TakeCoins(ActionValues::EmployeeMult(Employee::Engineer, 1)),
                            ]), ]),
                        ]),
                        5 => Or::new(&[
                            XOr::new(&[And::new(&[
                                ActionTag::DiscardCard(DiscardCardOpts::AnyCow),
                                ActionTag::TakeCoins(ActionValues::Exact(3)),
                                ActionTag::TakeObjective,
                            ]), ]),
                        ]),
                        6 => Or::new(&[
                            XOr::new(&[And::new(&[
                                ActionTag::MoveEngine(ActionValues::ForestMult(1)),
                            ]), ]),
                        ]),
                        7 => Or::new(&[
                            XOr::new(&[And::new(&[
                                ActionTag::AdjacentBuilding,
                            ]), ]),
                        ]),
                        8 => Or::new(&[
                            XOr::new(&[And::new(&[
                                ActionTag::StationDiscBehindTrain,
                            ]), ]),
                        ]),
                        9 => Or::new(&[
                            XOr::new(&[And::new(&[
                                ActionTag::TakeCoins(ActionValues::Exact(4)),
                            ]), ]),
                            XOr::new(&[And::new(&[
                                ActionTag::MoveEngine(ActionValues::Exact(4)),
                            ]), ]),
                            XOr::new(&[And::new(&[
                                ActionTag::MoveCattleman(ActionValues::Exact(4)),
                            ]), ]),
                        ]),
                        _ => panic!("Invalid building specified")
                    }
                }
            };
        let toll = {
            match num {
                0 => Toll::Green,
                1 => Toll::NoToll,
                2 => Toll::NoToll,
                3 => Toll::Black,
                4 => Toll::NoToll,
                5 => Toll::NoToll,
                6 => Toll::GreenBlack,
                7 => if !side_b { Toll::Green } else { Toll::NoToll },
                8 => Toll::NoToll,
                9 => Toll::Black,
                _ => panic!("Invalid building specified")
            }
        };
        let laborers = match num {
            0 => 1,
            1 => 1,
            2 => if !side_b { 1 } else { 2 },
            3 => 2,
            4 => 3,
            5 => 4,
            6 => 5,
            7 => if !side_b { 5 } else { 6 },
            8 => if !side_b { 7 } else { 6 },
            9 => if !side_b { 9 } else { 8 },
            _ => panic!("Invalid building specified")
        };
        let points = match num {
            0 => 1,
            1 => 1,
            2 => if !side_b { 1 } else { 3 },
            3 => 3,
            4 => 4,
            5 => 5,
            6 => 6,
            7 => if !side_b { 6 } else { 8 },
            8 => if !side_b { 9 } else { 8 },
            9 => if !side_b { 13 } else { 11 },
            _ => panic!("Invalid building specified")
        };
        Building { owner: 0, laborers, points, toll, actions }
    }
}
