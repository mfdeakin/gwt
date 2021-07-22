use serde::{Serialize, Deserialize};
use crate::buildings::{HazardType, Hazard, Tepee, Toll};
use crate::player::Employee;

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
    employees: Vec<Option<Employee>>,
    cost: [i32; JobMarket::NUM_ROWS],
    refresh_cow_market: Vec<usize>,
    game_end: usize,
    num_cols: usize,
}

impl JobMarket {
    const NUM_ROWS: usize = 12;

    pub fn new(num_players: usize) -> JobMarket {
        JobMarket { employees: Vec::<Option<Employee>>::with_capacity(JobMarket::NUM_ROWS * num_players), cost: JobMarket::defaultRowSalary(), refresh_cow_market: vec![6, 9], game_end: 12, num_cols: num_players }
    }

    pub fn addEmployee(&mut self, emp: Employee) -> JobMarketEvent {
        self.employees.push(Some(emp));
        for row in self.refresh_cow_market.iter() {
            if self.employees.len() / self.num_cols == *row && self.employees.len() % self.num_cols == 0 {
                return JobMarketEvent::RefillCowMarket;
            }
        }
        if self.employees.len() / self.num_cols == self.game_end && self.employees.len() % self.num_cols == 0 {
            return JobMarketEvent::FinalRound;
        }
        return JobMarketEvent::NoEvent;
    }

    pub fn hireEmployee(&mut self, emp: Employee, salary: i32) -> Result<(), String> {
        for (i, e) in self.employees.iter().enumerate() {
            if *e == Some(emp) && self.salary(i) == salary {
                self.employees[i] = None;
                return Ok(());
            }
        }
        return Err("No employee found".to_string());
    }

    fn salary(&self, emp_idx: usize) -> i32 {
        let idx = emp_idx / self.num_cols;
        if idx < self.cost.len() {
            self.cost[idx]
        } else {
            1_000_000_000 // An amount of money no player will ever have...
        }
    }

    pub fn findEmployees(&self, emp: Employee) -> Vec<(i32, usize)> {
        let mut salaries: Vec<(i32, usize)> = self.employees.iter()
            .enumerate()
            .filter(|check_emp| {
                // #62358 <https://github.com/rust-lang/rust/issues/62358>
                // *(*check_emp).1.contains(emp)
                match *(*check_emp).1 {
                    Some(check) => check == emp,
                    _ => false
                }
            })
            .map(|v| { (self.salary(v.0), v.0) })
            .collect();
        salaries.sort_unstable_by_key(|a| a.0);
        salaries.dedup_by_key(|a| a.0);
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

    #[test]
    fn testJobMarket1() {
        let mut market = JobMarket::new(1);
        for _ in 0..5 {
            assert_eq!(market.addEmployee(Employee::Engineer), JobMarketEvent::NoEvent);
        }
        assert_eq!(market.addEmployee(Employee::Engineer), JobMarketEvent::RefillCowMarket);
        for _ in 0..2 {
            assert_eq!(market.addEmployee(Employee::Cowboy), JobMarketEvent::NoEvent);
        }
        assert_eq!(market.addEmployee(Employee::Cowboy), JobMarketEvent::RefillCowMarket);
        for _ in 0..2 {
            assert_eq!(market.addEmployee(Employee::Cowboy), JobMarketEvent::NoEvent);
        }
        assert_eq!(market.addEmployee(Employee::Cowboy), JobMarketEvent::FinalRound);

        let eng_salaries = vec![5, 6, 7, 9];
        let eng_found = market.findEmployees(Employee::Engineer);
        let cowboy_salaries = vec![4, 5, 6, 8, 10];
        let cowboy_found = market.findEmployees(Employee::Cowboy);
        let craft_salaries = Vec::<i32>::new();
        let craft_found = market.findEmployees(Employee::Craftsman);
        for (truth, check) in [
            (eng_salaries, eng_found), (cowboy_salaries, cowboy_found), (craft_salaries, craft_found), ] {
            assert_eq!(truth.len(), check.len());
            for i in 0..truth.len() {
                assert_eq!(truth[i], check[i].0);
            }
        }
        assert_eq!(market.hireEmployee(Employee::Engineer, 5), Ok(()));
        assert_ne!(market.hireEmployee(Employee::Engineer, 5), Ok(()));
    }

    #[test]
    fn testJobMarket3() {
        let mut market = JobMarket::new(3);
        for _ in 0..17 {
            assert_eq!(market.addEmployee(Employee::Engineer), JobMarketEvent::NoEvent);
        }
        assert_eq!(market.addEmployee(Employee::Engineer), JobMarketEvent::RefillCowMarket);
        for _ in 0..8 {
            assert_eq!(market.addEmployee(Employee::Cowboy), JobMarketEvent::NoEvent);
        }
        assert_eq!(market.addEmployee(Employee::Cowboy), JobMarketEvent::RefillCowMarket);
        for _ in 0..8 {
            assert_eq!(market.addEmployee(Employee::Cowboy), JobMarketEvent::NoEvent);
        }
        assert_eq!(market.addEmployee(Employee::Cowboy), JobMarketEvent::FinalRound);

        let eng_salaries = vec![5, 6, 7, 9];
        let eng_found = market.findEmployees(Employee::Engineer);
        let cowboy_salaries = vec![4, 5, 6, 8, 10];
        let cowboy_found = market.findEmployees(Employee::Cowboy);
        let craft_salaries = Vec::<i32>::new();
        let craft_found = market.findEmployees(Employee::Craftsman);
        for (truth, check) in [
            (eng_salaries, eng_found), (cowboy_salaries, cowboy_found), (craft_salaries, craft_found), ] {
            assert_eq!(truth.len(), check.len());
            for i in 0..truth.len() {
                assert_eq!(truth[i], check[i].0);
            }
        }
        for _ in 0..3 {
            assert_eq!(market.hireEmployee(Employee::Engineer, 5), Ok(()));
        }
        assert_ne!(market.hireEmployee(Employee::Engineer, 5), Ok(()));
    }
}
