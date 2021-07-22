#[cfg(test)]
mod tests {
    use serde::{Serialize, Deserialize};
    use std::env;
    use std::fs;

    use engine::deck::{CowColor, Cow, Objective};
    use engine::buildings::Building;
    use std::path::Path;
    use engine::tiles::Foresight;

    #[test]
    pub fn saveDefaultTrack() {
        assert_eq!(1, 1);
        // let track = defaultStationTrack(defaultCityTrack(), vec![]);
        // let serialized = serde_json::to_string(&track).unwrap();
        // let path = Path::new("./data/default_station_track.json");
        // fs::write(path, serialized);
        //
        // let station_masters = defaultStationMasters();
        // let serialized = serde_json::to_string(&station_masters).unwrap();
        // let path = Path::new("./data/default_station_masters.json");
        // fs::write(path, serialized);
        //
        // let track = defaultTrack();
        // let serialized = serde_json::to_string(&track).unwrap();
        // let path = Path::new("./data/default_track.json");
        // fs::write(path, serialized);
        //
        // let mut neutral_buildings = Vec::<Building>::with_capacity(7);
        // for i in 0..7 {
        //     neutral_buildings.push(Building::basicBuilding(i));
        // }
        // let path = Path::new("./data/neutral_buildings.json");
        // let serialized = fs::read_to_string(path).unwrap();
        // let neutral_buildings_de : Vec<Building> = serde_json::from_str(&serialized).unwrap();
        // for i in 0 .. 7 {
        //     assert_eq!(neutral_buildings_de[i], neutral_buildings[i]);
        // }
        // let serialized = serde_json::to_string(&neutral_buildings).unwrap();
        // fs::write(path, serialized);
        //
        //
        // let mut player_buildings = Vec::<Building>::with_capacity(10);
        // for i in 0..7 {
        //     player_buildings.push(Building::playerBuilding(i, false));
        // }
        // let path = Path::new("./data/player_buildings_a.json");
        // let serialized = serde_json::to_string(&player_buildings).unwrap();
        // fs::write(path, serialized);
        //
        // let mut player_buildings = Vec::<Building>::with_capacity(10);
        // for i in 0..7 {
        //     player_buildings.push(Building::playerBuilding(i, true));
        // }
        // let path = Path::new("./data/player_buildings_b.json");
        // let serialized = serde_json::to_string(&player_buildings).unwrap();
        // fs::write(path, serialized);
        //
        // let mut player_starting_deck = Cow::playerStartingDeck();
        // let path = Path::new("./data/player_starting_deck.json");
        // let serialized = serde_json::to_string(&player_starting_deck).unwrap();
        // fs::write(path, serialized);
        //
        // let mut cow_deck = Cow::defaultCowDeck();
        // let path = Path::new("./data/cow_deck.json");
        // let serialized = serde_json::to_string(&cow_deck).unwrap();
        // fs::write(path, serialized);
        //
        // let mut player_starting_obj = Objective::baseObjectives();
        // let path = Path::new("./data/player_starting_objectives.json");
        // let serialized = serde_json::to_string(&player_starting_obj).unwrap();
        // fs::write(path, serialized);
        //
        // let mut obj = Objective::objectives();
        // let path = Path::new("./data/objectives_deck.json");
        // let serialized = serde_json::to_string(&obj).unwrap();
        // fs::write(path, serialized);
        //
        // let mut foresight = Foresight::new();
        // let path = Path::new("./data/foresight_piles.json");
        // let serialized = serde_json::to_string(&obj).unwrap();
        // fs::write(path, serialized);
    }
}