use std::mem::swap;

use rand::{SeedableRng, thread_rng};
use rand::prelude::SliceRandom;
use rand_pcg::Pcg64;
use serde::{Deserialize, Serialize};

use crate::actions::ActionTag;
use crate::deck::Card::{CowCard, ObjectiveCard};
use crate::logical::And;
use crate::player::ObjectiveResources;

#[derive(Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
pub enum Card {
    CowCard(Cow),
    ObjectiveCard(Objective),
}

#[derive(Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
pub struct Deck {
    hand: Vec<Card>,
    draw: Vec<Card>,
    discard: Vec<Card>,
    hand_size: usize,
    rng: Pcg64,
}

impl Deck {
    pub fn new_unshuffled(hand_size: usize, draw: Vec<Card>) -> Deck {
        Deck {
            hand: Vec::with_capacity(hand_size + 2),
            draw,
            discard: Vec::with_capacity(hand_size * 4),
            hand_size,
            rng: Pcg64::from_rng(thread_rng()).unwrap(),
        }
    }

    pub fn new(hand_size: usize, pile: Vec<Card>) -> Deck {
        let mut deck = Deck::new_unshuffled(hand_size, pile);
        deck.draw.shuffle(&mut deck.rng);
        deck
    }

    // Refills either to the hand limit, or until all cards are in the hand
    pub fn refillHand(&mut self) {
        while self.hand.len() < self.hand_size {
            if self.drawCard() != Ok(()) {
                break;
            }
        }
    }

    pub fn drawCard(&mut self) -> Result<(), String> {
        if self.draw.len() == 0 && self.discard.len() > 0 {
            self.shuffleDiscard();
        }
        if self.draw.len() > 0 {
            self.hand.push(self.draw.pop().unwrap());
            Result::Ok(())
        } else {
            Result::Err("No cards left to draw".to_string())
        }
    }

    pub fn shuffleDiscard(&mut self) {
        swap(&mut self.draw, &mut self.discard);
        self.draw.shuffle(&mut self.rng);
    }

    pub fn addCard(&mut self, card: Card) {
        self.discard.push(card);
    }

    pub fn trashCard(&mut self, card: Card) -> Result<(), String> {
        match self.hand.iter().position(|c| { *c == card }) {
            Some(idx) => {
                self.hand.remove(idx);
                Ok(())
            }
            None => Err("Card isn't in hand".to_string())
        }
    }

    pub fn playCard(&mut self, card: Card) -> Result<(), String> {
        match self.trashCard(card) {
            Ok(()) => {
                self.discard.push(card);
                Ok(())
            }
            err => err
        }
    }

    pub fn handValue(&self) -> u32 {
        let mut cows = Deck::deckCowCards(&self.hand);
        if cows.len() > 0 {
            cows.sort_by_key(|cow| { cow.color });
            cows.dedup_by_key(|cow| { cow.color });
            cows.iter().map(|cow| { cow.value() })
                .reduce(|v1, v2| { v1 + v2 }).unwrap()
        } else {
            0
        }
    }

    pub fn cowInHand(&self, color: CowColor) -> Option<Cow> {
        let cows: Vec<Cow> = Deck::deckCowCards(&self.hand).iter()
            .filter(|c| { (**c).color == color })
            .map(|c| { *c })
            .collect();
        if cows.len() > 0 {
            Some(cows[0])
        } else {
            None
        }
    }

    pub fn pairInHand(&self) -> Vec<CowColor> {
        let mut cows: Vec<CowColor> = Deck::deckCowCards(&self.hand).iter()
            .map(|c| { (*c).color })
            .collect();
        if cows.len() == 0 {
            vec![]
        } else {
            cows.sort_unstable();
            let mut dup_cow = Vec::<CowColor>::with_capacity(self.hand_size);
            let mut iter = cows.iter();
            let mut prev = iter.next().unwrap();
            let mut next = iter.next();
            while next != None {
                if *prev == *next.unwrap() && !dup_cow.contains(prev) {
                    dup_cow.push(*prev);
                }
                prev = next.unwrap();
                next = iter.next();
            }
            dup_cow
        }
    }

    pub fn cowCards(&self) -> Vec<Cow> {
        Deck::deckCowCards(&self.hand).iter()
            .chain(Deck::deckCowCards(&self.draw).iter())
            .chain(Deck::deckCowCards(&self.discard).iter())
            .map(|c| { *c })
            .collect()
    }

    pub fn cowPoints(&self) -> u32 {
        self.cowCards().iter()
            .map(|c| { c.points })
            .reduce(|a, b| { a + b })
            .unwrap_or(0) // This person lost the game...
    }

    pub fn objectiveCards(&self) -> Vec<Objective> {
        Deck::deckObjectiveCards(&self.hand).iter()
            .chain(Deck::deckObjectiveCards(&self.draw).iter())
            .chain(Deck::deckObjectiveCards(&self.discard).iter())
            .map(|obj| { *obj })
            .collect()
    }

    fn deckCowCards(cards: &Vec<Card>) -> Vec<Cow> {
        cards.iter()
            .filter(|card| { if let CowCard(_) = **card { true } else { false } })
            .map(|cow_card| { if let CowCard(cow) = *cow_card { cow } else { unreachable!() } })
            .collect()
    }

    fn deckObjectiveCards(cards: &Vec<Card>) -> Vec<Objective> {
        cards.iter()
            .filter(|card| { if let ObjectiveCard(_) = **card { true } else { false } })
            .map(|obj_card| { if let ObjectiveCard(obj) = *obj_card { obj } else { unreachable!() } })
            .collect()
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize, Debug)]
pub enum CowColor {
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

#[derive(Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
pub struct Cow {
    color: CowColor,
    points: u32,
}

impl Cow {
    pub fn new(color: CowColor, points: u32) -> Cow {
        Cow { color, points }
    }

    pub fn value(&self) -> u32 {
        match self.color {
            CowColor::Jersey => 1,
            CowColor::Dutch => 2,
            CowColor::Angus => 2,
            CowColor::Guernsey => 2,
            CowColor::Holstein => 3,
            CowColor::Swiss => 3,
            CowColor::Ayrshire => 3,
            CowColor::Highland => 4,
            CowColor::Longhorn => 5,
        }
    }
}

#[derive(Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
pub struct CowMarket {
    cow_deck: Vec<Cow>,
    ryb_market: Vec<Cow>,
    brown_market: Vec<Cow>,
    purple_market: Vec<Cow>,
}

impl CowMarket {
    pub fn new() -> CowMarket {
        let cow_deck = vec![Cow::new(CowColor::Holstein, 3)];
        let ryb_market = Vec::<Cow>::new();
        let brown_market = Vec::<Cow>::new();
        let purple_market = Vec::<Cow>::new();
        CowMarket { cow_deck, ryb_market, brown_market, purple_market }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
pub enum ObjectiveRequirements {
    Building,
    Hazard,
    SanFran,
    GreenTepee,
    BlueTepee,
    StationDisc,
    Cow(u32), // The cows value
}

#[derive(Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
pub struct Objective {
    immediate: Option<ActionTag>,
    success_pts: u32,
    fail_pts: u32,
    requirements: And<ObjectiveRequirements, 4>,
}

impl Objective {
    pub fn new(immediate: Option<ActionTag>, success_pts: u32, fail_pts: u32,
               requirements: &[ObjectiveRequirements]) -> Objective {
        Objective { immediate, success_pts, fail_pts, requirements: And::new(requirements) }
    }

    pub fn meetsRequirements(&self, mut resources: ObjectiveResources) -> Option<ObjectiveResources> {
        for req_opt in self.requirements.items {
            if let Some(obj_req) = req_opt {
                let req_test =
                    match obj_req {
                        ObjectiveRequirements::Building => {
                            if resources.buildings > 0
                            {
                                resources.buildings -= 1;
                                true
                            } else { false }
                        }
                        ObjectiveRequirements::Hazard => {
                            if resources.hazards > 0
                            {
                                resources.hazards -= 1;
                                true
                            } else { false }
                        }
                        ObjectiveRequirements::SanFran => {
                            if resources.san_fran > 0
                            {
                                resources.san_fran -= 1;
                                true
                            } else { false }
                        }
                        ObjectiveRequirements::GreenTepee => {
                            if resources.green_tepees > 0
                            {
                                resources.green_tepees -= 1;
                                true
                            } else { false }
                        }
                        ObjectiveRequirements::BlueTepee => {
                            if resources.blue_tepees > 0
                            {
                                resources.blue_tepees -= 1;
                                true
                            } else { false }
                        }
                        ObjectiveRequirements::StationDisc => {
                            if resources.station_discs > 0
                            {
                                resources.station_discs -= 1;
                                true
                            } else { false }
                        }
                        ObjectiveRequirements::Cow(v) => {
                            if v == 3 {
                                if resources.ryb_cows > 0
                                {
                                    resources.ryb_cows -= 1;
                                    true
                                } else { false }
                            } else if v == 4 {
                                if resources.brown_cows > 0
                                {
                                    resources.brown_cows -= 1;
                                    true
                                } else { false }
                            } else if v == 5 {
                                if resources.purple_cows > 0
                                {
                                    resources.purple_cows -= 1;
                                    true
                                } else { false }
                            } else { unreachable!() }
                        }
                    };
                if req_test == false {
                    return None;
                }
            }
        }
        Some(resources)
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;

    use crate::deck::Card::CowCard;

    use super::*;

    #[test]
    fn testDeck() {
        let path = Path::new("./data/player_starting_deck.json");
        let serialized = fs::read_to_string(path).unwrap();
        let starting_cows: Vec<Cow> = serde_json::from_str(&serialized).unwrap();
        let starting_deck = starting_cows.iter().map(|c| { CowCard(*c) }).collect();
        let mut d = Deck::new_unshuffled(4, starting_deck);
        assert_eq!(d.hand.len(), 0);
        assert_eq!(d.draw.len(), starting_cows.len());
        assert_eq!(d.discard.len(), 0);
        d.refillHand();


        // The initial hand is 3 angus cows and a guernsey;
        // ie the last 4 listed in the starting deck file
        assert_eq!(d.pairInHand(), vec![CowColor::Angus]);
        assert_eq!(d.hand.len(), d.hand_size);
        assert_eq!(d.draw.len(), starting_cows.len() - d.hand_size);
        assert_eq!(d.discard.len(), 0);
        assert_eq!(d.cowPoints(), 0);
        assert_eq!(d.handValue(), 4);

        assert_ne!(d.trashCard(Card::CowCard(Cow::new(CowColor::Jersey, 0))), Ok(()));
        assert_eq!(d.pairInHand(), vec![CowColor::Angus]);
        assert_eq!(d.hand.len(), d.hand_size);
        assert_eq!(d.draw.len(), starting_cows.len() - d.hand_size);
        assert_eq!(d.discard.len(), 0);
        assert_eq!(d.cowPoints(), 0);
        assert_eq!(d.handValue(), 4);

        assert_eq!(d.trashCard(Card::CowCard(Cow::new(CowColor::Angus, 0))), Ok(()));
        assert_eq!(d.pairInHand(), vec![CowColor::Angus]);
        assert_eq!(d.hand.len(), d.hand_size - 1);
        assert_eq!(d.draw.len(), starting_cows.len() - d.hand_size);
        assert_eq!(d.discard.len(), 0);
        assert_eq!(d.cowPoints(), 0);
        assert_eq!(d.handValue(), 4);

        assert_eq!(d.playCard(Card::CowCard(Cow::new(CowColor::Angus, 0))), Ok(()));
        assert_eq!(d.pairInHand(), Vec::<CowColor>::new());
        assert_eq!(d.hand.len(), d.hand_size - 2);
        assert_eq!(d.draw.len(), starting_cows.len() - d.hand_size);
        assert_eq!(d.discard.len(), 1);
        assert_eq!(d.cowPoints(), 0);
        assert_eq!(d.handValue(), 4);

        assert_eq!(d.playCard(Card::CowCard(Cow::new(CowColor::Angus, 0))), Ok(()));
        assert_eq!(d.pairInHand(), Vec::<CowColor>::new());
        assert_eq!(d.hand.len(), d.hand_size - 3);
        assert_eq!(d.draw.len(), starting_cows.len() - d.hand_size);
        assert_eq!(d.discard.len(), 2);
        assert_eq!(d.cowPoints(), 0);
        assert_eq!(d.handValue(), 2);

        d.addCard(Card::CowCard(Cow::new(CowColor::Longhorn, 7)));
        assert_eq!(d.pairInHand(), Vec::<CowColor>::new());
        assert_eq!(d.hand.len(), d.hand_size - 3);
        assert_eq!(d.draw.len(), starting_cows.len() - d.hand_size);
        assert_eq!(d.discard.len(), 3);
        assert_eq!(d.cowPoints(), 7);
        assert_eq!(d.handValue(), 2);

        assert_eq!(d.objectiveCards(), Vec::<Objective>::new());
        let obj = Objective::new(None, 5, 5, &[]);
        d.addCard(Card::ObjectiveCard(obj));
        assert_eq!(d.pairInHand(), Vec::<CowColor>::new());
        assert_eq!(d.hand.len(), d.hand_size - 3);
        assert_eq!(d.draw.len(), starting_cows.len() - d.hand_size);
        assert_eq!(d.discard.len(), 4);
        assert_eq!(d.cowPoints(), 7);
        assert_eq!(d.handValue(), 2);
        assert_eq!(d.objectiveCards(), vec![obj]);
    }
}
