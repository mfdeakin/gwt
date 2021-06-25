use crate::actions::Action;
use crate::player::Player;

pub enum Card<'a> {
    Cow(Cow),
    Objective(&'a Objective<'a>),
}

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

#[derive(Copy, Clone)]
pub struct Cow {
    color: Color,
    points: u32,
}

impl Cow {
    pub fn value(&self) -> u32 {
        match self.color {
            Color::Jersey => 1,
            Color::Dutch => 2,
            Color::Angus => 2,
            Color::Guernsey => 2,
            Color::Holstein => 3,
            Color::Swiss => 3,
            Color::Ayrshire => 3,
            Color::Highland => 4,
            Color::Longhorn => 5,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Objective<'a> {
    immediate: Action<'a>,
    success_pts: u32,
    fail_pts: u32,
}

impl<'a> Objective<'a> {
    fn meets_requirements(&self, player: Player) -> bool {
        panic!("Not implemented");
    }
}
