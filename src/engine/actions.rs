
use crate::player::{Player, Employee};
use crate::deck::{Cow, Objective};
use crate::buildings::{Tepee, Hazard, Building};

pub enum Action {
    PayCoins(i32),
    BuyCow(&Cow),
    Buy2Cows(&Cow, &Cow),
    SellCow(&Cow),
    Sell2Cows(&Cow, &Cow),
    SellHand,
    MoveCattleman(i32),
    MoveEngine(i32),
    MoveCertificate(i32), // Prestige, Pedigree, whatever it's called...
    MaxCertificate,
    TakeTepee(&Tepee),
    TakeHazard(&Hazard),
    TakeObjective(&Objective),
    PlaceDisc(u32),
    HireEmployee(&Employee),
    PlaceBuilding(i32, &Building),
}
