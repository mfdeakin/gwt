
use crate::actions::Action;
use crate::player::Player;

#[derive(Copy, Clone)]
pub enum Color {
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

pub struct Cow {
    color: Color,
    points: u32,
}

impl Cow {
    pub fn value(&self) -> u32 {
        match self.color {
            Jersey => 1,
            Dutch => 2,
            Angus => 2,
            Guernsey => 2,
            Holstein => 3,
            Swiss => 3,
            Ayrshire => 3,
            Highland => 4,
            Longhorn => 5,
        }
    }
}

impl Copy for Cow {}
impl Clone for Cow {
    fn clone(&self) -> Self { todo!() }
}

pub struct Objective<'a> {
    immediate: Action<'a>,
    success_pts: u32,
    fail_pts: u32,
}

impl<'a> Objective<'a> {
    fn meets_requirements(&self, player: Player) -> bool {
        panic!("Not implemented");
        false
    }
}
