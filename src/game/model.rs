use bevy::prelude::*;
use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Resource)]
pub struct CardGameModel {
    pub demons: Demons,
    pub player_stats: PlayerStats,
    pub deck: Vec<Card>,
    pub discard_pile: Vec<Card>,
    pub hand: Vec<Card>,
    pub in_play: Vec<Card>,
    next_card_id: u32,
}

impl CardGameModel {
    pub fn new(starter_cards: Vec<CardKind>) -> Self {
        let mut card_game_model = CardGameModel {
            demons: Demons {
                fear: Demon {
                    power: 5,
                    stun_time: 0,
                },
                despair: Demon {
                    power: 5,
                    stun_time: 0,
                },
                doubt: Demon {
                    power: 5,
                    stun_time: 0,
                },
            },
            player_stats: PlayerStats {
                resolve: 100,
                bravery: 20,
                hope: 20,
                confidence: 20,
            },
            deck: starter_cards
                .iter()
                .zip(0..)
                .map(|(kind, id)| Card { id, kind: *kind })
                .collect(),
            discard_pile: Vec::new(),
            hand: Vec::new(),
            in_play: Vec::new(),
            next_card_id: starter_cards.len() as u32,
        };
        card_game_model.deck.shuffle(&mut thread_rng());
        for _ in 0..5 {
            card_game_model.draw();
        }
        card_game_model
    }

    pub fn draw(&mut self) {
        // If there are no cards to draw, shuffle discard pile into deck
        if self.deck.is_empty() {
            self.deck.append(&mut self.discard_pile);
            self.deck.shuffle(&mut thread_rng());
        }
        let card = self.deck.pop().unwrap();
        self.hand.push(card);
    }

    fn find_card_in_hand(&self, card_id: u32) -> usize {
        // Find the index of the card with the given card_id
        self.hand
            .iter()
            .zip(0..)
            .find(|(c, _)| c.id == card_id)
            .unwrap()
            .1
    }

    pub fn discard(&mut self, card_id: u32) {
        let card_index = self.find_card_in_hand(card_id);
        let card = self.hand.remove(card_index);
        self.discard_pile.push(card);
    }

    pub fn gain(&mut self, kind: CardKind) {
        let card = Card {
            id: self.next_card_id,
            kind,
        };
        self.discard_pile.push(card);
        self.next_card_id += 1;
    }

    pub fn play(&mut self, card_id: u32) {
        let card_index = self.find_card_in_hand(card_id);
        let card = self.hand.remove(card_index);
        let card_kind = card.kind;
        self.in_play.push(card);
        match card_kind {
            CardKind::Angry => self.angry(),
            CardKind::Inspired => self.inspired(),
            CardKind::Tired => self.tired(),
            CardKind::Stressed => self.stressed(),
            CardKind::Satisfied => self.satisfied(),
            CardKind::Proud => self.proud(),
            CardKind::Determined => self.determined(),
            CardKind::Peaceful => self.peaceful(),
            CardKind::Dizzy => self.dizzy(),
            CardKind::Scared => self.scared(),
            CardKind::Hopeless => self.hopeless(),
            CardKind::Shameful => self.shameful(),
        }
    }

    fn inspired(&mut self) {
        // Add a new random card to your deck
    }

    fn peaceful(&mut self) {
        // +5 to bravery, hope, or confidence
    }

    fn tired(&mut self) {
        // +1 to a demon's power
    }

    fn stressed(&mut self) {
        // Gain Tired
    }

    fn angry(&mut self) {
        // Gain one when hit by an enemy
        // Stun a demon.
    }

    fn satisfied(&mut self) {
        // Gain three when picking up food
        // Discard a card and draw back to 5
    }

    fn proud(&mut self) {
        // Multiply next card action by 3.
    }

    fn determined(&mut self) {
        // Draw back up to 5 cards
    }

    fn dizzy(&mut self) {
        // Discard your deck
    }

    fn scared(&mut self) {
        // -10 Resolve, +1 to all demons' power
        self.player_stats.resolve -= 10;
        self.demons.fear.power += 1;
        self.demons.despair.power += 1;
        self.demons.doubt.power += 1;
    }

    fn hopeless(&mut self) {
        // -20 Resolve
        self.player_stats.resolve -= 20;
    }

    fn shameful(&mut self) {
        // -5 Resolve, gain Shameful
        self.player_stats.resolve -= 5;
        self.gain(CardKind::Shameful);
    }

    pub fn end_turn(&mut self) {
        self.cleanup();
        self.demon_attack();
        // Draw up to 5
        for _ in 0..(5 - self.hand.len()) {
            self.draw();
        }
    }

    fn cleanup(&mut self) {
        // All cards in play are discarded
        self.discard_pile.append(&mut self.in_play);
    }

    fn demon_attack(&mut self) {
        // Fear
        let fear_demon = &mut self.demons.fear;
        if fear_demon.stun_time > 0 {
            fear_demon.stun_time -= 1;
        } else if self.player_stats.bravery > 0 {
            self.player_stats.bravery -= fear_demon.power;
        } else {
            self.gain(CardKind::Scared)
        }
        // Despair
        let despair_demon = &mut self.demons.despair;
        if despair_demon.stun_time > 0 {
            despair_demon.stun_time -= 1;
        } else if self.player_stats.hope > 0 {
            self.player_stats.hope -= despair_demon.power;
        } else {
            self.gain(CardKind::Hopeless)
        }
        // Doubt
        let doubt_demon = &mut self.demons.doubt;
        if doubt_demon.stun_time > 0 {
            doubt_demon.stun_time -= 1;
        } else if self.player_stats.confidence > 0 {
            self.player_stats.confidence -= doubt_demon.power;
        } else {
            self.gain(CardKind::Shameful)
        }
    }
}

pub fn setup(mut commands: Commands) {
    let card_game_model = CardGameModel::new(vec![
        CardKind::Inspired,
        CardKind::Inspired,
        CardKind::Inspired,
        CardKind::Peaceful,
        CardKind::Peaceful,
        CardKind::Peaceful,
        CardKind::Peaceful,
        CardKind::Peaceful,
        CardKind::Peaceful,
        CardKind::Peaceful,
    ]);
    commands.insert_resource(card_game_model);
}

pub struct Demon {
    // How much damage the demon does to Resolve at the end of the turn
    pub power: u32,
    // How many turns the demon is stunned for
    pub stun_time: u32,
}

pub struct Demons {
    pub fear: Demon,
    pub despair: Demon,
    pub doubt: Demon,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum CardKind {
    Angry,
    Inspired,
    Tired,
    Stressed,
    Satisfied,
    Proud,
    Determined,
    Peaceful,
    Dizzy,
    Scared,
    Hopeless,
    Shameful,
}

#[derive(Clone)]
pub struct Card {
    pub id: u32,
    pub kind: CardKind,
}

pub struct PlayerStats {
    pub resolve: u32,
    pub bravery: u32,
    pub hope: u32,
    pub confidence: u32,
}
