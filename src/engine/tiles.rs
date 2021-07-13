use serde::{Serialize, Deserialize};
use crate::buildings::{HazardType, Hazard, Tepee, Toll};
use crate::player::Employee;
use std::collections::vec_deque::Iter;

// https://github.com/rust-lang/rust/issues/83574
// use std::iter::zip;

#[derive(Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
pub enum Tile {
    EmployeeTile(Employee),
    HazardTile(Hazard),
    TepeeTile(Tepee),
    BlankTile,
}

#[derive(Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
pub struct Foresight {
    current: [Tile; Foresight::FORESIGHT_SIZE],
    next: [Tile; Foresight::FORESIGHT_SIZE],
    piles: [Vec<Tile>; Foresight::FORESIGHT_SIZE],
}

impl Foresight {
    const FORESIGHT_SIZE: usize = 3;

    pub fn new() -> Foresight {
        Foresight { current: [Tile::BlankTile; 3], next: [Tile::BlankTile; 3], piles: Foresight::defaultTilePiles() }
    }

    pub fn nextTiles(&mut self) -> [Tile; 3] {
        let removed = self.current;
        self.current = self.next;
        // Use zip here when available
        for i in 0..self.piles.len() {
            self.next[i] = self.piles[i].pop().unwrap_or(Tile::BlankTile);
        }
        removed
    }

    pub fn defaultTilePiles() -> [Vec<Tile>; 3] {
        let mut tolls = vec![
            vec![Tile::TepeeTile(Tepee::Blue); 8],
            vec![Tile::TepeeTile(Tepee::Green); 9],
        ].into_iter().flatten().collect::<Vec<Tile>>();
        for haz in [HazardType::Flood, HazardType::Drought, HazardType::Rockfall] {
            tolls.append(&mut vec![Tile::HazardTile(Hazard::new(haz, Toll::Green, 4)); 2]);
            for toll in [Toll::Green, Toll::Black] {
                for i in 2..3 {
                    tolls.push(Tile::HazardTile(Hazard::new(haz, toll, i)));
                }
            }
        }
        let employees = vec![
            vec![Tile::EmployeeTile(Employee::Engineer); 11],
            vec![Tile::EmployeeTile(Employee::Cowboy); 11],
            vec![Tile::EmployeeTile(Employee::Craftsman); 11],
        ].into_iter().flatten().collect::<Vec<Tile>>();
        let mix = vec![
            vec![Tile::EmployeeTile(Employee::Engineer); 7],
            vec![Tile::EmployeeTile(Employee::Cowboy); 7],
            vec![Tile::EmployeeTile(Employee::Craftsman); 7],
            vec![Tile::TepeeTile(Tepee::Green); 2],
            vec![Tile::TepeeTile(Tepee::Blue); 3],
        ].into_iter().flatten().collect::<Vec<Tile>>();
        [tolls, employees, mix]
    }
}


#[derive(Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
pub enum JobMarketEvent {
    NoEvent,
    RefillCowMarket,
    FinalRound,
}

#[derive(Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
pub struct JobMarket {
    employees: Vec<Employee>,
    cost: [i32; JobMarket::NUM_ROWS],
    refresh_cow_market: Vec<usize>,
    game_end: usize,
    num_cols: usize,
}

impl JobMarket {
    const NUM_ROWS: usize = 12;

    fn new(num_players: usize) -> JobMarket {
        JobMarket { employees: Vec::<Employee>::with_capacity(JobMarket::NUM_ROWS * num_players), cost: JobMarket::defaultRowSalary(), refresh_cow_market: vec![6, 9], game_end: 12, num_cols: num_players }
    }

    fn addEmployee(&mut self, emp: Employee) -> JobMarketEvent {
        self.employees.push(emp);
        for row in self.refresh_cow_market.iter() {
            if self.employees.len() / self.num_cols == *row && self.employees.len() % self.num_cols == 0 {
                return JobMarketEvent::RefillCowMarket;
            }
        }
        if self.employees.len() / self.num_cols == self.game_end && self.employees.len() % self.num_cols == 0 {
            return JobMarketEvent::FinalRound;
        }
        let i = self.refresh_cow_market.iter();
        return JobMarketEvent::NoEvent;
    }

    fn salary(&self, emp_idx: usize) -> i32 {
        let idx = emp_idx / self.num_cols;
        self.cost[idx]
    }

    pub fn findEmployees(&self, emp: Employee) -> Vec<(i32, &Employee)> {
        let mut salaries : Vec<(i32, &Employee)> = self.employees.iter()
            .filter(|check_emp| { **check_emp == emp })
            .enumerate()
            .map(|v| { (self.salary(v.0), v.1) })
            .collect();
        salaries.sort_unstable_by_key(|a| a.0);
        salaries.dedup();
        salaries
    }

    fn defaultRowSalary() -> [i32; JobMarket::NUM_ROWS] {
        [6, 6, 7, 5, 7, 9, 6, 8, 10, 6, 5, 4]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::player::Employee;
}
