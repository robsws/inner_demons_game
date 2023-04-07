use bevy::prelude::*;
use rand::prelude::*;

use super::dungeon_model;

#[derive(Resource)]
pub struct CardGameModel {
    pub demons: Demons,
    pub player_stats: PlayerStats,
    pub deck: Vec<Card>,
    pub discard_pile: Vec<Card>,
    pub hand: Vec<Card>,
    pub in_play: Vec<Card>,
    next_card_id: u32,
    pub game_over: bool,
    pub win: bool,
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
            game_over: false,
            win: false,
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

    pub fn play(&mut self, card_id: u32, dmodel: &mut dungeon_model::DungeonGameModel) {
        let card_index = self.find_card_in_hand(card_id);
        let card = self.hand.remove(card_index);
        let card_kind = card.kind;
        self.in_play.push(card);
        match card_kind {
            CardKind::Angry => self.angry(dmodel),
            CardKind::Inspired => self.inspired(dmodel),
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

    fn peaceful(&mut self) {
        // +10 to random defensive trait
        let roll = rand::thread_rng().gen_range(0..3);
        match roll {
            0 => self.player_stats.bravery += 10,
            1 => self.player_stats.confidence += 10,
            2 => self.player_stats.hope += 10,
            _ => println!("Peaceful card rng broke."),
        };
    }

    fn tired(&mut self) {
        // +2 to a demon's power
        let roll = rand::thread_rng().gen_range(0..3);
        match roll {
            0 => self.demons.fear.power += 2,
            1 => self.demons.despair.power += 2,
            2 => self.demons.doubt.power += 2,
            _ => println!("Tired card rng broke."),
        };
    }

    fn stressed(&mut self) {
        // Gain one when hit by an enemy
        // Gain Tired
        self.gain(CardKind::Tired);
    }

    fn angry(&mut self, dmodel: &mut dungeon_model::DungeonGameModel) {
        // Gain one when hit by an enemy
        // +30 to hit
        // -30 to dodge
        // Gain Tired
        dmodel.player_aim += 30;
        dmodel.player_dodge -= 30;
        self.gain(CardKind::Tired);
    }

    fn inspired(&mut self, dmodel: &mut dungeon_model::DungeonGameModel) {
        // Gain three when picking up book
        // +1 Action
        dmodel.actions_left += 1;
    }

    fn satisfied(&mut self) {
        // Gain three when picking up food
        // Discard a random card
        let card_index = rand::thread_rng().gen_range(0..self.hand.len());
        self.discard(self.hand[card_index].id);
    }

    fn proud(&mut self) {
        // Gain three when picking up treasure
        // Acts like 3 peaceful.
        for _ in 0..3 {
            self.peaceful();
        }
    }

    fn determined(&mut self) {
        // Gain when defeating an enemy
        // Stun a demon
        let roll = rand::thread_rng().gen_range(0..3);
        match roll {
            0 => self.demons.fear.stun_time = 3,
            1 => self.demons.despair.stun_time = 3,
            2 => self.demons.despair.stun_time = 3,
            _ => println!("Determined card rng broke."),
        };
    }

    fn dizzy(&mut self) {
        // Gain when getting hit (random)
        // Discard your deck
        self.discard_pile.append(&mut self.deck);
    }

    fn scared(&mut self) {
        // -5 Resolve, +1 to all demons' power
        self.player_stats.resolve = self.player_stats.resolve.saturating_sub(5);
        self.demons.fear.power += 1;
        self.demons.despair.power += 1;
        self.demons.doubt.power += 1;
        self.check_game_over();
    }

    fn hopeless(&mut self) {
        // -10 Resolve
        self.player_stats.resolve = self.player_stats.resolve.saturating_sub(10);
        self.check_game_over();
    }

    fn shameful(&mut self) {
        // -1 Resolve, gain Shameful
        self.player_stats.resolve = self.player_stats.resolve.saturating_sub(1);
        self.gain(CardKind::Shameful);
        self.check_game_over();
    }

    fn check_game_over(&mut self) {
        if self.player_stats.resolve == 0 {
            self.game_over = true;
        }
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
            self.player_stats.bravery = self.player_stats.bravery.saturating_sub(fear_demon.power);
        } else {
            self.gain(CardKind::Scared)
        }
        // Despair
        let despair_demon = &mut self.demons.despair;
        if despair_demon.stun_time > 0 {
            despair_demon.stun_time -= 1;
        } else if self.player_stats.hope > 0 {
            self.player_stats.hope = self.player_stats.hope.saturating_sub(despair_demon.power);
        } else {
            self.gain(CardKind::Hopeless)
        }
        // Doubt
        let doubt_demon = &mut self.demons.doubt;
        if doubt_demon.stun_time > 0 {
            doubt_demon.stun_time -= 1;
        } else if self.player_stats.confidence > 0 {
            self.player_stats.confidence = self
                .player_stats
                .confidence
                .saturating_sub(doubt_demon.power);
        } else {
            self.gain(CardKind::Shameful)
        }
    }
}

pub fn setup(mut commands: Commands) {
    let card_game_model = CardGameModel::new(vec![
        CardKind::Peaceful,
        CardKind::Peaceful,
        CardKind::Peaceful,
        CardKind::Peaceful,
        CardKind::Peaceful,
        CardKind::Peaceful,
        CardKind::Peaceful,
        CardKind::Tired,
        CardKind::Tired,
        CardKind::Tired,
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
