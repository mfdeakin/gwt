
use crate::actions::Action;

pub enum Employee {
    Cowboy = 0,
    Craftsman = 1,
    Engineer = 2,
}

pub struct Player {
    coins: i32,
    hired: [u32; 3],
    // utility: Vec<dyn Action>,
}
