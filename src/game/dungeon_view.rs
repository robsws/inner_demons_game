use bevy::{prelude::*, render::camera::ScalingMode, utils::Duration};

use super::dungeon_model;
use super::dungeon_model::Coord;

#[derive(Component)]
pub struct Canvas;

#[derive(Resource)]
pub struct InputCooldown {
    timer: Timer,
}

#[derive(Resource)]
pub struct ImageHandles {
    items: ItemImageHandles,
}

struct ItemImageHandles {
    book: Handle<Image>,
    chest: Handle<Image>,
    food: Handle<Image>,
}

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let img_item_book = asset_server.load("images/book.png");
    let img_item_chest = asset_server.load("images/chest.png");
    let img_item_food = asset_server.load("images/food.png");
    let image_handles = ImageHandles {
        items: ItemImageHandles {
            book: img_item_book,
            chest: img_item_chest,
            food: img_item_food,
        },
    };
    commands.insert_resource(image_handles);
    let camera_bundle = Camera2dBundle {
        projection: OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical(20.0),
            ..default()
        },
        ..default()
    };
    commands.spawn(camera_bundle);
    commands
        .spawn(SpatialBundle {
            transform: Transform {
                translation: Vec3::new(0.0, 4.0, 0.0),
                ..default()
            },
            visibility: Visibility::Visible,
            ..default()
        })
        .insert(Canvas);
    commands.insert_resource(InputCooldown {
        timer: Timer::new(Duration::from_millis(250), TimerMode::Once),
    })
}

pub fn refresh_view(
    mut commands: Commands,
    model: Res<dungeon_model::DungeonGameModel>,
    q_canvas: Query<Entity, With<Canvas>>,
    image_handles: Res<ImageHandles>,
) {
    let canvas = q_canvas.single();
    commands.entity(canvas).despawn_descendants();
    // Render the free tiles in a 20x10 square around the player
    for x in -10..11 {
        for y in -5..6 {
            let pos = Coord(x + model.player_pos.0, y + model.player_pos.1);
            if model.map.floor_coords.contains(&pos) {
                // Draw a grey square
                commands.entity(canvas).with_children(|parent| {
                    parent.spawn(SpriteBundle {
                        sprite: Sprite {
                            color: Color::rgb(0.3, 0.3, 0.3),
                            custom_size: Some(Vec2::new(0.8, 0.8)),
                            ..default()
                        },
                        transform: Transform {
                            translation: Vec3::new(x as f32, y as f32, 0.0),
                            ..default()
                        },
                        ..default()
                    });
                });
            }
            // Check for items
            match model.find_item_at_pos(&pos) {
                Some(item) => {
                    // Draw the item
                    let texture = match item.kind {
                        dungeon_model::ItemKind::Book => image_handles.items.book.clone(),
                        dungeon_model::ItemKind::Chest => image_handles.items.chest.clone(),
                        dungeon_model::ItemKind::Food => image_handles.items.food.clone(),
                    };
                    commands.entity(canvas).with_children(|parent| {
                        parent.spawn(SpriteBundle {
                            sprite: Sprite {
                                custom_size: Some(Vec2::new(0.9, 0.9)),
                                ..default()
                            },
                            transform: Transform {
                                translation: Vec3::new(x as f32, y as f32, 0.0),
                                ..default()
                            },
                            texture,
                            ..default()
                        });
                    });
                }
                None => (),
            }
            // Check for enemies
            match model.find_enemy_at_pos(&pos) {
                Some(_) => {
                    commands.entity(canvas).with_children(|parent| {
                        parent.spawn(SpriteBundle {
                            sprite: Sprite {
                                color: Color::rgb(0.4, 0.0, 0.0),
                                custom_size: Some(Vec2::new(0.9, 0.9)),
                                ..default()
                            },
                            transform: Transform {
                                translation: Vec3::new(x as f32, y as f32, 0.0),
                                ..default()
                            },
                            ..default()
                        });
                    });
                }
                None => (),
            }
        }
    }
    commands.entity(canvas).with_children(|parent| {
        parent.spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.0, 0.5, 0.5),
                custom_size: Some(Vec2::new(1.0, 1.0)),
                ..default()
            },
            ..default()
        });
    });
}

pub fn keyboard_input(
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut cooldown: ResMut<InputCooldown>,
    mut dungeon_model: ResMut<dungeon_model::DungeonGameModel>,
) {
    cooldown.timer.tick(time.delta());
    if !cooldown.timer.finished() {
        return;
    }
    if input.any_pressed([KeyCode::Up, KeyCode::W]) {
        dungeon_model.move_player(dungeon_model::Direction::Up);
        cooldown.timer.reset();
    }
    if input.any_pressed([KeyCode::Down, KeyCode::S]) {
        dungeon_model.move_player(dungeon_model::Direction::Down);
        cooldown.timer.reset();
    }
    if input.any_pressed([KeyCode::Right, KeyCode::D]) {
        dungeon_model.move_player(dungeon_model::Direction::Right);
        cooldown.timer.reset();
    }
    if input.any_pressed([KeyCode::Left, KeyCode::A]) {
        dungeon_model.move_player(dungeon_model::Direction::Left);
        cooldown.timer.reset();
    }
}
