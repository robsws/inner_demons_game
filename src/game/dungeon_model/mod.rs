use bevy::prelude::*;
use rand::prelude::*;

mod map;

use map::Map;

use super::model::{self, CardGameModel};

#[derive(Resource)]
pub struct DungeonGameModel {
    pub map: Map,
    pub player_pos: Coord,
    pub player_aim: u8,
    pub player_dodge: u8,
    pub got_hit: bool,
    pub turn_actions: u8,
    pub actions_left: u8,
    pub items: Vec<Item>,
    pub enemies: Vec<Enemy>,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct Coord(pub i32, pub i32);

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct Item {
    pub id: u32,
    pub kind: ItemKind,
    pub pos: Coord,
    pub collected: bool,
}

pub enum ItemKind {
    Book,
    Chest,
    Food,
}

pub struct Enemy {
    pub id: u32,
    pub dead: bool,
    pub pos: Coord,
}

pub fn setup(mut commands: Commands) {
    commands.insert_resource(DungeonGameModel::new());
}

impl DungeonGameModel {
    pub fn new() -> Self {
        let m = Map::new();
        let player_start = m.player_start;
        let items = Self::init_items(&m);
        let enemies = Self::init_enemies(&m);
        DungeonGameModel {
            map: m,
            player_pos: player_start,
            player_aim: 40,
            player_dodge: 40,
            got_hit: false,
            turn_actions: 5,
            actions_left: 5,
            items,
            enemies,
        }
    }

    fn init_items(m: &Map) -> Vec<Item> {
        let mut items = Vec::new();
        let mut item_id = 0;
        for book_pos in m.book_positions.iter() {
            items.push(Item {
                id: item_id,
                kind: ItemKind::Book,
                pos: *book_pos,
                collected: false,
            });
            item_id += 1;
        }
        for chest_pos in m.chest_positions.iter() {
            items.push(Item {
                id: item_id,
                kind: ItemKind::Chest,
                pos: *chest_pos,
                collected: false,
            });
            item_id += 1;
        }
        for food_pos in m.food_positions.iter() {
            items.push(Item {
                id: item_id,
                kind: ItemKind::Food,
                pos: *food_pos,
                collected: false,
            });
            item_id += 1;
        }
        items
    }

    fn init_enemies(m: &Map) -> Vec<Enemy> {
        let mut enemies = Vec::new();
        let mut enemy_id = 0;
        for pos in m.enemy_positions.iter() {
            enemies.push(Enemy {
                id: enemy_id,
                pos: *pos,
                dead: false,
            });
            enemy_id += 1;
        }
        enemies
    }

    pub fn move_player(&mut self, dir: Direction, cmodel: &mut CardGameModel) {
        if self.actions_left == 0 {
            return;
        }
        let move_vector = match dir {
            Direction::Up => (0, 1),
            Direction::Down => (0, -1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        };
        let next_pos = Coord(
            self.player_pos.0 + move_vector.0,
            self.player_pos.1 + move_vector.1,
        );
        // Check for walls
        if !self.map.floor_coords.contains(&next_pos) {
            return;
        }
        self.actions_left -= 1;
        // Check for enemies
        match self.enemies.iter().find(|e| !e.dead && e.pos == next_pos) {
            Some(enemy) => {
                self.combat_enemy(enemy.id, cmodel);
                return;
            }
            None => (),
        }
        // Check for items
        match self
            .items
            .iter()
            .find(|i| !i.collected && i.pos == next_pos)
        {
            Some(item) => self.get_item(item.id, cmodel),
            None => (),
        }
        // Move player
        self.player_pos = next_pos;
    }

    fn combat_enemy(&mut self, enemy_id: u32, cmodel: &mut CardGameModel) {
        let player_hit_roll = rand::thread_rng().gen_range(0..100);
        if player_hit_roll < self.player_aim {
            match self.enemies.iter_mut().find(|e| e.id == enemy_id) {
                Some(enemy) => {
                    enemy.dead = true;
                    cmodel.gain(model::CardKind::Determined);
                }
                None => println!("Hit non existent enemy"),
            }
        }

        let enemy_hit_roll = rand::thread_rng().gen_range(0..100);
        if enemy_hit_roll > self.player_dodge {
            self.got_hit = true;
            // Gain a random negative card
            let roll = rand::thread_rng().gen_range(0..3);
            match roll {
                0 => cmodel.gain(model::CardKind::Dizzy),
                1 => cmodel.gain(model::CardKind::Stressed),
                2 => cmodel.gain(model::CardKind::Angry),
                _ => println!("Combat rng broke."),
            }
        }
    }

    fn get_item(&mut self, item_id: u32, cmodel: &mut model::CardGameModel) {
        match self.items.iter_mut().find(|i| i.id == item_id) {
            Some(item) => {
                item.collected = true;
                match item.kind {
                    ItemKind::Book => {
                        for _ in 0..3 {
                            cmodel.gain(model::CardKind::Inspired);
                        }
                    }
                    ItemKind::Chest => {
                        for _ in 0..3 {
                            cmodel.gain(model::CardKind::Proud);
                        }
                    }
                    ItemKind::Food => {
                        for _ in 0..3 {
                            cmodel.gain(model::CardKind::Satisfied);
                        }
                    }
                }
            }
            None => println!("Collected non existent item"),
        }
    }

    pub fn end_turn(&mut self) {
        self.actions_left = self.turn_actions;
        self.got_hit = false;
        self.player_aim = 40;
        self.player_dodge = 40;
    }

    pub fn find_item_at_pos(&self, pos: &Coord) -> Option<&Item> {
        self.items.iter().find(|i| i.pos == *pos && !i.collected)
    }

    pub fn find_enemy_at_pos(&self, pos: &Coord) -> Option<&Enemy> {
        self.enemies.iter().find(|e| e.pos == *pos && !e.dead)
    }
}
