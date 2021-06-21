
use crate::actions::Action;

pub enum Employee {
    Cowboy = 0,
    Craftsman = 1,
    Engineer = 2,
}

impl Copy for Employee {}
impl Clone for Employee {
    fn clone(&self) -> Self { todo!() }
}

pub struct Player {
    coins: i32,
    hired: [u32; 3],

}
