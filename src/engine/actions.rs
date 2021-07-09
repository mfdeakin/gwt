
use serde::{Serialize, Deserialize};
use crate::player::{Employee};
use crate::deck::{Cow, Objective, Card, CowColor};
use crate::buildings::{Tepee, Hazard, Building};

#[derive(Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
pub enum Action {
    PayCoins(i32),
    BuyCow(Cow),
    Buy2Cows(Cow, Cow),
    SellCow(Cow),
    SellCowPair(Cow, Cow),
    SellHand,
    FillHand,
    DrawCards(Option<u32>),
    DiscardCards(Option<u32>),
    TrashCard(Option<Card>),
    MoveCattleman(Option<i32>),
    MoveEngine(Option<i32>),
    MoveCertificate(Option<i32>), // Prestige, Pedigree, whatever it's called...
    MaxCertificate,
    TakeTepee(Option<Tepee>),
    TakeHazard(Option<Hazard>),
    TakeObjective(Option<Objective>),
    TakeStationmaster(Option<Employee>),
    RemoveDisc(Option<u32>),
    PlaceDisc(Option<u32>),
    HireEmployee(Option<Employee>),
    PlaceBuilding(Option<u32>, Option<Building>),
    Auxiliary1,
    Auxiliary2
}

#[derive(Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
pub enum DiscardCardOpts {
    AnyCow,
    Color(CowColor),
    PairCow,
    Objective,
}

#[derive(Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
pub enum ActionValues {
    Exact(i32),
    EmployeeMult(Employee, i32), // Multiplier based on the number of a type of hired employee
    AllEmployeesMult(i32), // Multiplier based on the number of any type of employee
    ForestMult(i32), // Multiplier based on the number of the players buildings in the forest
    TepeePairMult(i32), // Multiplier based on the number of blue-green tepee pairs
    HazardPairMult(i32), // Multiplier based on the number of blue-green tepee pairs
    Max,
}

#[derive(Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
pub enum ActionTag {
    TakeCoins(ActionValues),
    BuyCows,
    DiscardCard(DiscardCardOpts),
    DrawCards(ActionValues),
    TrashCard,
    MoveCattleman(ActionValues),
    TeleportCattleman(ActionValues),
    MoveEngine(ActionValues),
    MoveCertificate(ActionValues),
    TakeTepee,
    TakeHazard,
    TakeObjective,
    PlaceDisc(ActionValues),
    HireEmployee(ActionValues),
    PlaceBuilding(ActionValues),
    CityDiscMoveTrain,
    StationDiscBehindTrain,
    AdjacentBuilding,
    Auxiliary,
    DoubleAuxiliary,
}
