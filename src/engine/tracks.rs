
use crate::buildings::Building;
use std::fs::File;
use std::io::Read;
use std::mem::size_of;

#[derive(Copy, Clone)]
pub struct Space<'a> {
    building: Option<Building<'a>>,
    nextBuilding: usize,
    nextSpace: usize,
}

impl<'a> Space<'a> {
    pub fn empty(pos: usize) -> Space<'a> {
        return Space { building: None, nextBuilding: 0, nextSpace: pos + 1 };
    }

    pub fn readSpace(mut input: &File, pos: usize) -> Space {
        let mut hasBuilding: [u8; size_of::<usize>()] = [0; size_of::<usize>()];
        input.read(&mut hasBuilding[..]).unwrap();
        let mut building: Option<Building> = None;
        if hasBuilding[0] > 0 {
            building = Some(Building::readBuilding(input));
        }
        return Space { building: building, nextBuilding: 0, nextSpace: pos + 1 };
    }
}

fn readTrack(mut input: &File) -> Vec<Space> {
    let mut numSpaces: [u8; size_of::<usize>()] = [0; size_of::<usize>()];
    input.read(&mut numSpaces[..]).unwrap();
    let numSpaces: usize = numSpaces[0].into();
    let mut track: Vec<Space> = Vec::with_capacity(numSpaces);
    for i in 0..numSpaces {
        track[i] = Space::empty(i)
    }
    return track;
}