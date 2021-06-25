
use crate::player::{Player, Employee};
use crate::deck::{Cow, Objective, Card};
use crate::buildings::{Tepee, Hazard, Building};

pub enum Action<'a> {
    PayCoins(i32),
    BuyCow(Cow),
    Buy2Cows(Cow, Cow),
    SellCow(Cow),
    Sell2Cows(Cow, Cow),
    SellHand,
    FillHand,
    DrawCards(u32),
    DiscardCards(u32),
    TrashCard(&'a Card<'a>),
    MoveCattleman(i32),
    MoveEngine(i32),
    MoveCertificate(i32), // Prestige, Pedigree, whatever it's called...
    MaxCertificate,
    TakeTepee(Tepee),
    TakeHazard(Hazard),
    TakeObjective(&'a Objective<'a>),
    TakeStationmaster(Employee),
    RemoveDisc(u32),
    PlaceDisc(u32),
    HireEmployee(Employee),
    PlaceBuilding(u32, &'a Building<'a>),
}

impl<'a> Copy for Action<'a> {}
impl<'a> Clone for Action<'a> {
    fn clone(&self) -> Self { todo!() }
}
