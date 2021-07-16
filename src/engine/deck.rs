
use rand::{thread_rng, SeedableRng};
use serde::{Serialize, Deserialize};
use crate::actions::{ActionTag, ActionValues};
use crate::player::Player;
use crate::logical::And;
use std::mem::swap;
use rand_pcg::Pcg64;
use rand::prelude::SliceRandom;
use crate::deck::Card::{CowCard, ObjectiveCard};

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
        swap(&mut self.draw,&mut self.discard);
        self.draw.shuffle(&mut self.rng);
    }

    pub fn addCard(&mut self, card: Card) {
        self.discard.push(card);
    }

    pub fn trashCard(&mut self, card: Card) -> Result<(), String> {
        match self.hand.iter().position(|c| { *c == card }) {
            Some(idx) => { self.hand.remove(idx); Ok(()) }
            None => Err("Card isn't in hand".to_string())
        }
    }

    pub fn playCard(&mut self, card: Card) -> Result<(), String> {
        match self.trashCard(card) {
            Ok(()) => {
                self.discard.push(card);
                Ok(())
            },
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
        let cows : Vec<Cow> = Deck::deckCowCards(&self.hand).iter()
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
        cows.sort();
        let mut prev = cows.iter();
        let mut dup_cow = Vec::<CowColor>::new();
        for cur in prev.next() {
            if *cur == prev[0] && !dup_cow.contains(*cur){
                dup_cow.push(*cur);
            }
            prev = prev.next().unwrap();
        }
        dup_cow
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

    pub fn defaultCowDeck() -> Vec<Cow> {
        vec![
            vec![Cow::new(CowColor::Holstein, 1); 7],
            vec![Cow::new(CowColor::Swiss, 2); 7],
            vec![Cow::new(CowColor::Ayrshire, 3); 7],
            vec![Cow::new(CowColor::Highland, 3); 3],
            vec![Cow::new(CowColor::Highland, 4); 3],
            vec![Cow::new(CowColor::Highland, 5); 3],
            vec![Cow::new(CowColor::Longhorn, 5); 2],
            vec![Cow::new(CowColor::Longhorn, 6); 2],
            vec![Cow::new(CowColor::Longhorn, 7); 2],
        ].into_iter().flatten().collect::<Vec<Cow>>()
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

    pub fn playerStartingDeck() -> Vec<Cow> {
        let mut deck = Vec::from([Cow::new(CowColor::Jersey, 0); 5]);
        deck.append(&mut Vec::from([Cow::new(CowColor::Dutch, 0); 3]));
        deck.append(&mut Vec::from([Cow::new(CowColor::Guernsey, 0); 3]));
        deck.append(&mut Vec::from([Cow::new(CowColor::Angus, 0); 3]));

        return deck;
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

    pub fn meetsRequirements(&self, player: &Player) -> bool {
        panic!("Not implemented");
    }

    pub fn baseObjectives() -> Vec<Objective> {
        vec![Objective::new(None, 3, 0,
                            &[ObjectiveRequirements::Cow(3), ObjectiveRequirements::Cow(4), ObjectiveRequirements::Building, ]),
             Objective::new(None, 3, 0,
                            &[ObjectiveRequirements::BlueTepee, ObjectiveRequirements::Hazard, ObjectiveRequirements::Hazard, ]),
             Objective::new(None, 3, 0,
                            &[ObjectiveRequirements::StationDisc, ObjectiveRequirements::StationDisc, ObjectiveRequirements::GreenTepee, ]),
             Objective::new(None, 3, 0,
                            &[ObjectiveRequirements::Building, ObjectiveRequirements::Building, ObjectiveRequirements::Hazard, ]),
        ]
    }

    pub fn objectives() -> Vec<Objective> {
        let teleport = Some(ActionTag::TeleportCattleman(ActionValues::Exact(3)));
        let move_eng_2 = Some(ActionTag::MoveEngine(ActionValues::Exact(2)));
        let move_eng_3 = Some(ActionTag::MoveEngine(ActionValues::Exact(3)));
        let dbl_aux = Some(ActionTag::DoubleAuxiliary);
        let take_coins = Some(ActionTag::TakeCoins(ActionValues::Exact(2)));
        let draw_cards = Some(ActionTag::DrawCards(ActionValues::Exact(3)));

        let cow_3 = ObjectiveRequirements::Cow(3);
        let cow_4 = ObjectiveRequirements::Cow(4);
        let cow_5 = ObjectiveRequirements::Cow(5);
        use ObjectiveRequirements::StationDisc;
        use ObjectiveRequirements::GreenTepee;
        use ObjectiveRequirements::BlueTepee;
        use ObjectiveRequirements::Hazard;
        use ObjectiveRequirements::Building;
        use ObjectiveRequirements::SanFran;
        vec![
            Objective::new(dbl_aux, 5, 3, &[SanFran,]),
            Objective::new(dbl_aux, 5, 3, &[SanFran,]),
            Objective::new(dbl_aux, 5, 3, &[SanFran,]),
            Objective::new(dbl_aux, 5, 3, &[SanFran,]),
            Objective::new(draw_cards, 4, 2, &[cow_3, cow_3, cow_3, StationDisc,]),
            Objective::new(take_coins, 4, 2, &[cow_3, cow_3, cow_3, Building,]),
            Objective::new(move_eng_2, 5, 3, &[cow_3, cow_4, cow_5,]),
            Objective::new(teleport, 5, 2, &[cow_3, cow_4, cow_5,]),
            Objective::new(teleport, 5, 2, &[cow_3, cow_4, Hazard, Hazard,]),
            Objective::new(take_coins, 3, 2, &[cow_4, Hazard, Hazard,]),
            Objective::new(move_eng_2, 5, 3, &[cow_4, cow_4, StationDisc, GreenTepee,]),
            Objective::new(draw_cards, 3, 2, &[cow_5, Hazard,]),
            Objective::new(take_coins, 3, 2, &[StationDisc, StationDisc, Hazard,]),
            Objective::new(move_eng_3, 5, 3, &[StationDisc, StationDisc, Hazard, Hazard,]),
            Objective::new(teleport, 5, 2, &[StationDisc, StationDisc, Building, Building,]),
            Objective::new(teleport, 5, 2, &[StationDisc, StationDisc, BlueTepee, BlueTepee,]),
            Objective::new(draw_cards, 3, 2, &[StationDisc, GreenTepee, GreenTepee,]),
            Objective::new(draw_cards, 3, 2, &[StationDisc, GreenTepee, BlueTepee,]),
            Objective::new(draw_cards, 3, 2, &[Building, Building, Hazard,]),
            Objective::new(teleport, 5, 2, &[Building, Building, Hazard, Hazard,]),
            Objective::new(move_eng_2, 5, 3, &[Building, Building, GreenTepee, GreenTepee,]),
            Objective::new(move_eng_3, 5, 3, &[Building, BlueTepee, Hazard, Hazard,]),
            Objective::new(take_coins, 3, 2, &[Building, BlueTepee, BlueTepee,]),
            Objective::new(take_coins, 3, 2, &[Building, GreenTepee, BlueTepee,]),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;
    use std::fs;
    use crate::deck::Card::CowCard;

    #[test]
    fn testDeck() {
        let path = Path::new("./data/player_starting_deck.json");
        let serialized = fs::read_to_string(path).unwrap();
        let starting_cows : Vec<Cow> = serde_json::from_str(&serialized).unwrap();
        let starting_deck = starting_cows.iter().map(|c| { CowCard(*c) }).collect();
        let mut d = Deck::new_unshuffled(4, starting_deck);
        assert_eq!(d.hand.len(), 0);
        assert_eq!(d.draw.len(), starting_cows.len());
        assert_eq!(d.discard.len(), 0);
        d.refillHand();
        // The initial hand is 3 angus cows and a guernsey;
        // ie the last 4 listed in the starting deck file
        assert_eq!(d.hand.len(), d.hand_size);
        assert_eq!(d.draw.len(), starting_cows.len() - d.hand_size);
        assert_eq!(d.discard.len(), 0);
        assert_eq!(d.cowPoints(), 0);
        assert_eq!(d.handValue(), 4);

        assert_ne!(d.trashCard(Card::CowCard(Cow::new(CowColor::Jersey, 0))), Ok(()));
        assert_eq!(d.hand.len(), d.hand_size);
        assert_eq!(d.draw.len(), starting_cows.len() - d.hand_size);
        assert_eq!(d.discard.len(), 0);
        assert_eq!(d.cowPoints(), 0);
        assert_eq!(d.handValue(), 4);

        assert_eq!(d.trashCard(Card::CowCard(Cow::new(CowColor::Angus, 0))), Ok(()));
        assert_eq!(d.hand.len(), d.hand_size - 1);
        assert_eq!(d.draw.len(), starting_cows.len() - d.hand_size);
        assert_eq!(d.discard.len(), 0);
        assert_eq!(d.cowPoints(), 0);
        assert_eq!(d.handValue(), 4);

        assert_eq!(d.playCard(Card::CowCard(Cow::new(CowColor::Angus, 0))), Ok(()));
        assert_eq!(d.hand.len(), d.hand_size - 2);
        assert_eq!(d.draw.len(), starting_cows.len() - d.hand_size);
        assert_eq!(d.discard.len(), 1);
        assert_eq!(d.cowPoints(), 0);
        assert_eq!(d.handValue(), 4);

        assert_eq!(d.playCard(Card::CowCard(Cow::new(CowColor::Angus, 0))), Ok(()));
        assert_eq!(d.hand.len(), d.hand_size - 3);
        assert_eq!(d.draw.len(), starting_cows.len() - d.hand_size);
        assert_eq!(d.discard.len(), 2);
        assert_eq!(d.cowPoints(), 0);
        assert_eq!(d.handValue(), 2);

        d.addCard(Card::CowCard(Cow::new(CowColor::Longhorn, 7)));
        assert_eq!(d.hand.len(), d.hand_size - 3);
        assert_eq!(d.draw.len(), starting_cows.len() - d.hand_size);
        assert_eq!(d.discard.len(), 3);
        assert_eq!(d.cowPoints(), 7);
        assert_eq!(d.handValue(), 2);

        assert_eq!(d.objectiveCards(), Vec::<Objective>::new());
        let obj = Objective::new(None, 5, 5, &[]);
        d.addCard(Card::ObjectiveCard(obj));
        assert_eq!(d.hand.len(), d.hand_size - 3);
        assert_eq!(d.draw.len(), starting_cows.len() - d.hand_size);
        assert_eq!(d.discard.len(), 4);
        assert_eq!(d.cowPoints(), 7);
        assert_eq!(d.handValue(), 2);
        assert_eq!(d.objectiveCards(), vec![obj]);
    }
}
