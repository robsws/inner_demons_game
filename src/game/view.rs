use std::collections::HashMap;

use bevy::prelude::*;

use super::{
    dungeon_model,
    model::{self, CardKind},
};

#[derive(Resource, Clone)]
pub struct ImageHandles {
    end_turn_btn: ButtonImageHandles,
    cards: HashMap<model::CardKind, CardImageHandles>,
    card_back: Handle<Image>,
}

#[derive(Clone)]
struct ButtonImageHandles {
    normal: Handle<Image>,
    hover: Handle<Image>,
}

#[derive(Resource, Clone)]
pub struct FontHandles {
    regular: Handle<Font>,
}

#[derive(Component, Clone)]
pub struct Card {
    model: model::Card,
    image_handles: CardImageHandles,
}

#[derive(Bundle)]
struct CardBundle {
    card: Card,
    button: ButtonBundle,
}

#[derive(Clone)]
struct CardImageHandles {
    face_up: Handle<Image>,
    hover: Handle<Image>,
}

#[derive(Component)]
pub struct DeckArea;

#[derive(Component)]
pub struct DeckTop;

#[derive(Component)]
pub struct HandArea;

#[derive(Component)]
pub struct DiscardArea;

#[derive(Component)]
pub struct DiscardTop;

#[derive(Component)]
pub struct PlayArea;

#[derive(Component)]
pub struct HudArea1;

#[derive(Component)]
pub struct BraveryValue;

#[derive(Component)]
pub struct HopeValue;

#[derive(Component)]
pub struct ConfidenceValue;

#[derive(Component)]
pub struct HudArea2;

#[derive(Component)]
pub struct MovesLeftValue;

#[derive(Component)]
pub struct EndTurnBtn;

#[derive(Component)]
pub struct HudArea3;

#[derive(Component)]
pub struct FearPower;

#[derive(Component)]
pub struct DespairPower;

#[derive(Component)]
pub struct DoubtPower;

#[derive(Component)]
pub struct AimValue;

#[derive(Component)]
pub struct DodgeValue;

#[derive(Component)]
pub struct ResolveValue;

#[derive(Resource)]
pub struct CardText {
    lookup: HashMap<CardKind, String>,
}

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Load images
    let img_card_back = asset_server.load("images/Card Back.png");
    let img_card_inspired = asset_server.load("images/Inspired.png");
    let img_card_inspired_hover = asset_server.load("images/InspiredHover.png");
    let img_card_peaceful = asset_server.load("images/Peaceful.png");
    let img_card_peaceful_hover = asset_server.load("images/PeacefulHover.png");
    let img_card_dizzy = asset_server.load("images/Dizzy.png");
    let img_card_dizzy_hover = asset_server.load("images/DizzyHover.png");
    let img_card_determined = asset_server.load("images/Determined.png");
    let img_card_determined_hover = asset_server.load("images/DeterminedHover.png");
    let img_card_proud = asset_server.load("images/Proud.png");
    let img_card_proud_hover = asset_server.load("images/ProudHover.png");
    let img_card_satisfied = asset_server.load("images/Satisfied.png");
    let img_card_satisfied_hover = asset_server.load("images/SatisfiedHover.png");
    let img_card_angry = asset_server.load("images/Anger.png");
    let img_card_angry_hover = asset_server.load("images/AngerHover.png");
    let img_card_stressed = asset_server.load("images/Stressed.png");
    let img_card_stressed_hover = asset_server.load("images/StressedHover.png");
    let img_card_tired = asset_server.load("images/Tired.png");
    let img_card_tired_hover = asset_server.load("images/TiredHover.png");
    let img_card_hopeless = asset_server.load("images/Hopeless.png");
    let img_card_hopeless_hover = asset_server.load("images/HopelessHover.png");
    let img_card_scared = asset_server.load("images/Scared.png");
    let img_card_scared_hover = asset_server.load("images/ScaredHover.png");
    let img_card_shameful = asset_server.load("images/Shameful.png");
    let img_card_shameful_hover = asset_server.load("images/ShamefulHover.png");
    let img_btn_end_turn = asset_server.load("images/end_turn_btn.png");
    let img_btn_end_turn_hover = asset_server.load("images/end_turn_btn_hover.png");

    // Add card text
    let mut card_text = HashMap::new();
    card_text.insert(
        CardKind::Peaceful,
        "+10 to either Bravery, Hope, or Confidence".to_string(),
    );
    card_text.insert(
        CardKind::Tired,
        "+2 power to either Fear, Despair, or Doubt inner demons".to_string(),
    );
    card_text.insert(CardKind::Stressed, "Gain a Tired card".to_string());
    card_text.insert(
        CardKind::Angry,
        "+30 to hit. -30 to dodge. Gain a Tired card.".to_string(),
    );
    card_text.insert(CardKind::Inspired, "+1 Action".to_string());
    card_text.insert(
        CardKind::Satisfied,
        "Discard a random card from your hand".to_string(),
    );
    card_text.insert(
        CardKind::Proud,
        "+10 to either Bravery, Hope, or Confidence, 3 times".to_string(),
    );
    card_text.insert(
        CardKind::Determined,
        "Stun a random demon for 3 turns".to_string(),
    );
    card_text.insert(CardKind::Dizzy, "Discard your deck".to_string());
    card_text.insert(
        CardKind::Scared,
        "-5 Resolve. +1 to all inner demons' power.".to_string(),
    );
    card_text.insert(CardKind::Hopeless, "-10 Resolve".to_string());
    card_text.insert(
        CardKind::Shameful,
        "-1 Resolve. Gain a Shameful card".to_string(),
    );

    commands.insert_resource(CardText { lookup: card_text });

    // Initialise card image handles
    let mut card_image_handles = HashMap::new();
    card_image_handles.insert(
        model::CardKind::Inspired,
        CardImageHandles {
            face_up: img_card_inspired.clone(),
            hover: img_card_inspired_hover.clone(),
        },
    );
    card_image_handles.insert(
        model::CardKind::Peaceful,
        CardImageHandles {
            face_up: img_card_peaceful.clone(),
            hover: img_card_peaceful_hover.clone(),
        },
    );
    card_image_handles.insert(
        model::CardKind::Dizzy,
        CardImageHandles {
            face_up: img_card_dizzy.clone(),
            hover: img_card_dizzy_hover.clone(),
        },
    );
    card_image_handles.insert(
        model::CardKind::Determined,
        CardImageHandles {
            face_up: img_card_determined.clone(),
            hover: img_card_determined_hover.clone(),
        },
    );
    card_image_handles.insert(
        model::CardKind::Proud,
        CardImageHandles {
            face_up: img_card_proud.clone(),
            hover: img_card_proud_hover.clone(),
        },
    );
    card_image_handles.insert(
        model::CardKind::Satisfied,
        CardImageHandles {
            face_up: img_card_satisfied.clone(),
            hover: img_card_satisfied_hover.clone(),
        },
    );
    card_image_handles.insert(
        model::CardKind::Angry,
        CardImageHandles {
            face_up: img_card_angry.clone(),
            hover: img_card_angry_hover.clone(),
        },
    );
    card_image_handles.insert(
        model::CardKind::Stressed,
        CardImageHandles {
            face_up: img_card_stressed.clone(),
            hover: img_card_stressed_hover.clone(),
        },
    );
    card_image_handles.insert(
        model::CardKind::Tired,
        CardImageHandles {
            face_up: img_card_tired.clone(),
            hover: img_card_tired_hover.clone(),
        },
    );
    card_image_handles.insert(
        model::CardKind::Hopeless,
        CardImageHandles {
            face_up: img_card_hopeless.clone(),
            hover: img_card_hopeless_hover.clone(),
        },
    );
    card_image_handles.insert(
        model::CardKind::Scared,
        CardImageHandles {
            face_up: img_card_scared.clone(),
            hover: img_card_scared_hover.clone(),
        },
    );
    card_image_handles.insert(
        model::CardKind::Shameful,
        CardImageHandles {
            face_up: img_card_shameful.clone(),
            hover: img_card_shameful_hover.clone(),
        },
    );

    // Add image handles as resources
    let image_handles = ImageHandles {
        end_turn_btn: ButtonImageHandles {
            normal: img_btn_end_turn.clone(),
            hover: img_btn_end_turn_hover.clone(),
        },
        cards: card_image_handles,
        card_back: img_card_back.clone(),
    };
    commands.insert_resource(image_handles.clone());

    // Load fonts
    let font_handles = FontHandles {
        regular: asset_server.load("fonts/Kenney High.ttf"),
    };
    commands.insert_resource(font_handles.clone());
    // Root node of layout
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::width(Val::Percent(100.0)),
                flex_direction: FlexDirection::ColumnReverse,
                justify_content: JustifyContent::End,
                ..default()
            },
            ..default()
        })
        .with_children(|root| {
            // Hand, deck and discard pile area
            root.spawn(NodeBundle {
                style: Style {
                    size: Size::height(Val::Px(154.0)),
                    justify_content: JustifyContent::SpaceBetween,
                    ..default()
                },
                background_color: Color::rgb(0.4, 0.4, 0.4).into(),
                ..default()
            })
            .with_children(|dock| {
                // Deck area
                dock.spawn(NodeBundle {
                    style: Style {
                        size: Size::width(Val::Px(138.0)),
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    background_color: Color::rgb(0.3, 0.3, 0.3).into(),
                    ..default()
                })
                .with_children(|deck_area| {
                    deck_area
                        .spawn(ImageBundle {
                            style: Style {
                                size: Size::new(Val::Px(128.0), Val::Px(144.0)),
                                ..default()
                            },
                            image: UiImage {
                                texture: image_handles.card_back.clone(),
                                ..default()
                            },
                            ..default()
                        })
                        .insert(DeckTop);
                })
                .insert(DeckArea);
                // Hand area
                dock.spawn(NodeBundle {
                    style: Style {
                        size: Size::width(Val::Percent(100.0)),
                        justify_content: JustifyContent::Start,
                        ..default()
                    },
                    background_color: Color::rgb(0.3, 0.0, 0.3).into(),
                    ..default()
                })
                .insert(HandArea);
                // Discard pile area
                dock.spawn(NodeBundle {
                    style: Style {
                        size: Size::width(Val::Px(138.0)),
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    background_color: Color::rgb(0.3, 0.3, 0.3).into(),
                    ..default()
                })
                .insert(DiscardArea)
                .with_children(|discard_area| {
                    discard_area
                        .spawn(ImageBundle {
                            style: Style {
                                size: Size::new(Val::Px(128.0), Val::Px(144.0)),
                                ..default()
                            },
                            image: UiImage {
                                texture: image_handles.card_back.clone(),
                                ..default()
                            },
                            visibility: Visibility::Hidden,
                            ..default()
                        })
                        .insert(DiscardTop);
                });
            });
            // HUD area 1
            root.spawn(NodeBundle {
                style: Style {
                    size: Size::height(Val::Px(30.0)),
                    justify_content: JustifyContent::SpaceBetween,
                    ..default()
                },
                background_color: Color::rgb(0.3, 0.1, 0.7).into(),
                ..default()
            })
            .with_children(|hud1| {
                hud1.spawn(TextBundle {
                    text: Text::from_section(
                        "Deck",
                        TextStyle {
                            font: font_handles.regular.clone(),
                            font_size: 20.0,
                            color: Color::WHITE,
                        },
                    ),
                    style: Style {
                        margin: UiRect {
                            left: Val::Px(10.0),
                            ..default()
                        },
                        ..default()
                    },
                    ..default()
                });
                hud1.spawn(TextBundle {
                    text: Text::from_sections([
                        TextSection::new(
                            "Bravery: ",
                            TextStyle {
                                font: font_handles.regular.clone(),
                                font_size: 20.0,
                                color: Color::WHITE,
                            },
                        ),
                        TextSection::new(
                            "0",
                            TextStyle {
                                font: font_handles.regular.clone(),
                                font_size: 20.0,
                                color: Color::CYAN,
                            },
                        ),
                    ]),
                    ..default()
                })
                .insert(BraveryValue);
                hud1.spawn(TextBundle {
                    text: Text::from_sections([
                        TextSection::new(
                            "Hope: ",
                            TextStyle {
                                font: font_handles.regular.clone(),
                                font_size: 20.0,
                                color: Color::WHITE,
                            },
                        ),
                        TextSection::new(
                            "0",
                            TextStyle {
                                font: font_handles.regular.clone(),
                                font_size: 20.0,
                                color: Color::CYAN,
                            },
                        ),
                    ]),
                    ..default()
                })
                .insert(HopeValue);
                hud1.spawn(TextBundle {
                    text: Text::from_sections([
                        TextSection::new(
                            "Confidence: ",
                            TextStyle {
                                font: font_handles.regular.clone(),
                                font_size: 20.0,
                                color: Color::WHITE,
                            },
                        ),
                        TextSection::new(
                            "0",
                            TextStyle {
                                font: font_handles.regular.clone(),
                                font_size: 20.0,
                                color: Color::CYAN,
                            },
                        ),
                    ]),
                    ..default()
                })
                .insert(ConfidenceValue);
                hud1.spawn(TextBundle {
                    text: Text::from_section(
                        "Discard",
                        TextStyle {
                            font: font_handles.regular.clone(),
                            font_size: 20.0,
                            color: Color::WHITE,
                        },
                    ),
                    style: Style {
                        margin: UiRect {
                            right: Val::Px(10.0),
                            ..default()
                        },
                        ..default()
                    },
                    ..default()
                });
            })
            .insert(HudArea1);
            // HUD area 3
            root.spawn(NodeBundle {
                style: Style {
                    size: Size::height(Val::Px(30.0)),
                    justify_content: JustifyContent::SpaceBetween,
                    ..default()
                },
                background_color: Color::rgb(0.3, 0.0, 0.0).into(),
                ..default()
            })
            .with_children(|hud1| {
                hud1.spawn(TextBundle {
                    text: Text::from_section(
                        "DEMONS",
                        TextStyle {
                            font: font_handles.regular.clone(),
                            font_size: 20.0,
                            color: Color::BLACK,
                        },
                    ),
                    style: Style {
                        margin: UiRect {
                            left: Val::Px(10.0),
                            ..default()
                        },
                        ..default()
                    },
                    ..default()
                });
                hud1.spawn(TextBundle {
                    text: Text::from_sections([
                        TextSection::new(
                            "Fear: ",
                            TextStyle {
                                font: font_handles.regular.clone(),
                                font_size: 20.0,
                                color: Color::WHITE,
                            },
                        ),
                        TextSection::new(
                            "0",
                            TextStyle {
                                font: font_handles.regular.clone(),
                                font_size: 20.0,
                                color: Color::RED,
                            },
                        ),
                    ]),
                    ..default()
                })
                .insert(FearPower);
                hud1.spawn(TextBundle {
                    text: Text::from_sections([
                        TextSection::new(
                            "Despair: ",
                            TextStyle {
                                font: font_handles.regular.clone(),
                                font_size: 20.0,
                                color: Color::WHITE,
                            },
                        ),
                        TextSection::new(
                            "0",
                            TextStyle {
                                font: font_handles.regular.clone(),
                                font_size: 20.0,
                                color: Color::RED,
                            },
                        ),
                    ]),
                    ..default()
                })
                .insert(DespairPower);
                hud1.spawn(TextBundle {
                    text: Text::from_sections([
                        TextSection::new(
                            "Doubt: ",
                            TextStyle {
                                font: font_handles.regular.clone(),
                                font_size: 20.0,
                                color: Color::WHITE,
                            },
                        ),
                        TextSection::new(
                            "0",
                            TextStyle {
                                font: font_handles.regular.clone(),
                                font_size: 20.0,
                                color: Color::RED,
                            },
                        ),
                    ]),
                    ..default()
                })
                .insert(DoubtPower);
                hud1.spawn(TextBundle {
                    text: Text::from_section(
                        "DEMONS",
                        TextStyle {
                            font: font_handles.regular.clone(),
                            font_size: 20.0,
                            color: Color::BLACK,
                        },
                    ),
                    style: Style {
                        margin: UiRect {
                            right: Val::Px(10.0),
                            ..default()
                        },
                        ..default()
                    },
                    ..default()
                });
            })
            .insert(HudArea3);
            // Play area
            root.spawn(NodeBundle {
                style: Style {
                    size: Size::height(Val::Px(154.0)),
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                background_color: Color::rgb(0.1, 0.1, 0.1).into(),
                ..default()
            })
            .insert(PlayArea);
            // HUD area 2
            root.spawn(NodeBundle {
                style: Style {
                    size: Size::height(Val::Px(30.0)),
                    justify_content: JustifyContent::SpaceBetween,
                    ..default()
                },
                background_color: Color::rgb(0.3, 0.1, 0.7).into(),
                ..default()
            })
            .with_children(|hud2| {
                hud2.spawn(TextBundle {
                    text: Text::from_sections([
                        TextSection::new(
                            "Moves left: ",
                            TextStyle {
                                font: font_handles.regular.clone(),
                                font_size: 20.0,
                                color: Color::WHITE,
                            },
                        ),
                        TextSection::new(
                            "0",
                            TextStyle {
                                font: font_handles.regular.clone(),
                                font_size: 20.0,
                                color: Color::CYAN,
                            },
                        ),
                    ]),
                    ..default()
                })
                .insert(MovesLeftValue);
                hud2.spawn(TextBundle {
                    text: Text::from_sections([
                        TextSection::new(
                            "Hit %: ",
                            TextStyle {
                                font: font_handles.regular.clone(),
                                font_size: 20.0,
                                color: Color::WHITE,
                            },
                        ),
                        TextSection::new(
                            "0",
                            TextStyle {
                                font: font_handles.regular.clone(),
                                font_size: 20.0,
                                color: Color::CYAN,
                            },
                        ),
                    ]),
                    ..default()
                })
                .insert(AimValue);
                hud2.spawn(TextBundle {
                    text: Text::from_sections([
                        TextSection::new(
                            "Dodge %: ",
                            TextStyle {
                                font: font_handles.regular.clone(),
                                font_size: 20.0,
                                color: Color::WHITE,
                            },
                        ),
                        TextSection::new(
                            "0",
                            TextStyle {
                                font: font_handles.regular.clone(),
                                font_size: 20.0,
                                color: Color::CYAN,
                            },
                        ),
                    ]),
                    ..default()
                })
                .insert(DodgeValue);
                hud2.spawn(ButtonBundle {
                    style: Style {
                        size: Size::width(Val::Px(120.0)),
                        margin: UiRect {
                            left: Val::Px(5.0),
                            ..default()
                        },
                        ..default()
                    },
                    image: UiImage {
                        texture: image_handles.end_turn_btn.normal.clone(),
                        ..default()
                    },
                    ..default()
                })
                .insert(EndTurnBtn);
            })
            .insert(HudArea2);
            // Demon area
            root.spawn(NodeBundle {
                style: Style {
                    size: Size::height(Val::Percent(100.0)),
                    ..default()
                },
                visibility: Visibility::Hidden,
                ..default()
            });
            // Resolve display
            root.spawn(NodeBundle {
                style: Style {
                    size: Size::height(Val::Px(30.0)),
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                background_color: Color::rgb(0.3, 0.1, 0.7).into(),
                ..default()
            })
            .with_children(|parent| {
                parent
                    .spawn(TextBundle {
                        text: Text::from_sections([
                            TextSection::new(
                                "Resolve: ",
                                TextStyle {
                                    font: font_handles.regular.clone(),
                                    font_size: 24.0,
                                    color: Color::WHITE,
                                },
                            ),
                            TextSection::new(
                                "0",
                                TextStyle {
                                    font: font_handles.regular.clone(),
                                    font_size: 24.0,
                                    color: Color::GREEN,
                                },
                            ),
                        ]),
                        ..default()
                    })
                    .insert(ResolveValue);
            });
        });
}

fn create_card(card_model: model::Card, image_handles: &ImageHandles) -> CardBundle {
    let kind = card_model.kind;
    let card = Card {
        model: card_model,
        image_handles: image_handles.cards.get(&kind).unwrap().clone(),
    };
    let start_texture = card.image_handles.face_up.clone();
    CardBundle {
        card,
        button: ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(128.0), Val::Px(144.0)),
                margin: UiRect {
                    left: Val::Px(5.0),
                    top: Val::Px(5.0),
                    ..default()
                },
                ..default()
            },
            image: UiImage {
                texture: start_texture,
                ..default()
            },
            ..default()
        },
    }
}

pub fn clear_hand_and_play_area(
    mut commands: Commands,
    mut q_hand_area: Query<Entity, With<HandArea>>,
    mut q_play_area: Query<Entity, With<PlayArea>>,
) {
    commands.entity(q_hand_area.single_mut()).clear_children();
    commands.entity(q_play_area.single_mut()).clear_children();
}

pub fn refresh_hand(
    mut commands: Commands,
    mut q_hand_area: Query<Entity, With<HandArea>>,
    mut q_cards: Query<(Entity, &Parent, &Card, &mut UiImage)>,
    image_handles: Res<ImageHandles>,
    game_model: Res<model::CardGameModel>,
) {
    // Catalog the card objects that already exist in the scene
    // to avoid despawning and creating them every frame
    let mut visible_cards: HashMap<u32, (Entity, Entity)> = HashMap::new();
    for (entity, parent, card, mut image) in q_cards.iter_mut() {
        visible_cards.insert(card.model.id, (entity, parent.get()));
    }

    // Update the hand
    let e_hand_area = q_hand_area.single_mut();
    let mut hand_area = commands.entity(e_hand_area);
    for card in game_model.hand.iter() {
        match visible_cards.get(&card.id) {
            Some((entity, parent)) => {
                if *parent != e_hand_area {
                    println!("setting parent");
                    // Set card's parent as the hand area
                    hand_area.push_children(&[*entity]);
                }
            }
            None => {
                // Spawn a new card object
                hand_area.with_children(|parent| {
                    let new_card_obj = create_card(card.clone(), &image_handles);
                    parent.spawn(new_card_obj);
                });
            }
        }
    }
}

pub fn refresh_play_area(
    mut commands: Commands,
    mut q_play_area: Query<Entity, With<PlayArea>>,
    q_play_area_children: Query<&Children, With<PlayArea>>,
    mut q_cards: Query<(Entity, &Card, &mut UiImage)>,
    image_handles: Res<ImageHandles>,
    game_model: Res<model::CardGameModel>,
) {
    // Catalog the card objects that already exist in the scene
    // to avoid despawning and creating them every frame
    let mut visible_cards: HashMap<u32, Entity> = HashMap::new();
    for (entity, card, _) in q_cards.iter_mut() {
        visible_cards.insert(card.model.id, entity);
    }

    let e_play_area = q_play_area.single_mut();
    commands.entity(e_play_area).clear_children();

    // Update the play area
    let mut play_area = commands.entity(e_play_area);
    for card in game_model.in_play.iter() {
        match visible_cards.get(&card.id) {
            Some(entity) => {
                // Set card's parent as the play area
                play_area.push_children(&[*entity]);
            }
            None => {
                // Spawn a new card object
                play_area.with_children(|parent| {
                    let new_card_obj = create_card(card.clone(), &image_handles);
                    parent.spawn(new_card_obj);
                });
            }
        }
    }

    // Make play area cards face up
    for children in q_play_area_children.iter() {
        for child in children.iter() {
            let (entity, card, mut image) = q_cards.get_mut(*child).unwrap();
            image.texture = card.image_handles.face_up.clone();
        }
    }
}

pub fn refresh_deck(
    mut q_deck_top_visibility: Query<&mut Visibility, With<DeckTop>>,
    game_model: Res<model::CardGameModel>,
) {
    // Update the deck
    let mut deck_top_visibility = q_deck_top_visibility.single_mut();
    if game_model.deck.is_empty() {
        *deck_top_visibility = Visibility::Hidden;
    } else {
        *deck_top_visibility = Visibility::Visible;
    }
}

pub fn refresh_discard_pile(
    mut q_disc_top: Query<(&mut Visibility, &mut UiImage), With<DiscardTop>>,
    image_handles: Res<ImageHandles>,
    game_model: Res<model::CardGameModel>,
) {
    // Update the discard pile
    let (mut disc_top_visibility, mut disc_top_image) = q_disc_top.single_mut();
    if game_model.discard_pile.is_empty() {
        *disc_top_visibility = Visibility::Hidden;
    } else {
        *disc_top_visibility = Visibility::Visible;
        // Set the discard top's image to the card on top of the discard pile
        let top_card_kind = game_model.discard_pile.last().unwrap().kind;
        disc_top_image.texture = match image_handles.cards.get(&top_card_kind) {
            Some(card_image_handles) => card_image_handles.face_up.clone(),
            None => {
                println!("{:?}", top_card_kind);
                image_handles.card_back.clone()
            }
        }
    }
}

pub fn cleanup_dead_cards(
    mut commands: Commands,
    game_model: Res<model::CardGameModel>,
    q_cards: Query<(Entity, &Card, &mut UiImage)>,
) {
    // Clean up cards in either the deck or discard pile
    for (card_entity, card, _) in q_cards.iter() {
        if game_model
            .deck
            .iter()
            .find(|c| c.id == card.model.id)
            .is_some()
            || game_model
                .discard_pile
                .iter()
                .find(|c| c.id == card.model.id)
                .is_some()
        {
            commands.entity(card_entity).despawn_recursive();
        }
    }
}

pub fn refresh_bravery(
    mut q_bravery_val: Query<&mut Text, With<BraveryValue>>,
    game_model: Res<model::CardGameModel>,
) {
    // Update the HUD
    let mut bravery_ui = q_bravery_val.single_mut();
    bravery_ui.sections[1].value = game_model.player_stats.bravery.to_string();
}

pub fn refresh_hope(
    mut q_hope_val: Query<&mut Text, With<HopeValue>>,
    game_model: Res<model::CardGameModel>,
) {
    let mut hope_ui = q_hope_val.single_mut();
    hope_ui.sections[1].value = game_model.player_stats.hope.to_string();
}

pub fn refresh_confidence(
    mut q_confidence_val: Query<&mut Text, With<ConfidenceValue>>,
    game_model: Res<model::CardGameModel>,
) {
    let mut confidence_ui = q_confidence_val.single_mut();
    confidence_ui.sections[1].value = game_model.player_stats.confidence.to_string();
}

pub fn refresh_moves_left(
    mut q_moves_left_val: Query<&mut Text, With<MovesLeftValue>>,
    game_model: Res<dungeon_model::DungeonGameModel>,
) {
    let mut moves_left_ui = q_moves_left_val.single_mut();
    moves_left_ui.sections[1].value = game_model.actions_left.to_string();
}

pub fn refresh_aim(
    mut q_aim_val: Query<&mut Text, With<AimValue>>,
    game_model: Res<dungeon_model::DungeonGameModel>,
) {
    let mut aim_ui = q_aim_val.single_mut();
    aim_ui.sections[1].value = game_model.player_aim.to_string();
}

pub fn refresh_dodge(
    mut q_dodge_val: Query<&mut Text, With<DodgeValue>>,
    game_model: Res<dungeon_model::DungeonGameModel>,
) {
    let mut dodge_ui = q_dodge_val.single_mut();
    dodge_ui.sections[1].value = game_model.player_dodge.to_string();
}

pub fn refresh_resolve(
    mut q_resolve_val: Query<&mut Text, With<ResolveValue>>,
    game_model: Res<model::CardGameModel>,
) {
    let mut resolve_ui = q_resolve_val.single_mut();
    resolve_ui.sections[1].value = game_model.player_stats.resolve.to_string();
}

pub fn refresh_fear(
    mut q_fear_val: Query<&mut Text, With<FearPower>>,
    game_model: Res<model::CardGameModel>,
) {
    let mut fear_ui = q_fear_val.single_mut();
    if game_model.demons.fear.stun_time > 0 {
        fear_ui.sections[0].value = "STUNNED: ".to_string();
        fear_ui.sections[1].value = game_model.demons.fear.stun_time.to_string();
    } else {
        fear_ui.sections[0].value = "Fear: ".to_string();
        fear_ui.sections[1].value = game_model.demons.fear.power.to_string();
    }
}

pub fn refresh_despair(
    mut q_despair_val: Query<&mut Text, With<DespairPower>>,
    game_model: Res<model::CardGameModel>,
) {
    let mut despair_ui = q_despair_val.single_mut();
    if game_model.demons.despair.stun_time > 0 {
        despair_ui.sections[0].value = "STUNNED: ".to_string();
        despair_ui.sections[1].value = game_model.demons.despair.stun_time.to_string();
    } else {
        despair_ui.sections[0].value = "Despair: ".to_string();
        despair_ui.sections[1].value = game_model.demons.despair.power.to_string();
    }
}

pub fn refresh_doubt(
    mut q_doubt_val: Query<&mut Text, With<DoubtPower>>,
    game_model: Res<model::CardGameModel>,
) {
    let mut doubt_ui = q_doubt_val.single_mut();
    if game_model.demons.doubt.stun_time > 0 {
        doubt_ui.sections[0].value = "STUNNED: ".to_string();
        doubt_ui.sections[1].value = game_model.demons.doubt.stun_time.to_string();
    } else {
        doubt_ui.sections[0].value = "Doubt: ".to_string();
        doubt_ui.sections[1].value = game_model.demons.doubt.power.to_string();
    }
}

pub fn hand_card_interaction(
    mut commands: Commands,
    q_hand_area_children: Query<&Children, With<HandArea>>,
    mut q_interaction: Query<
        (&Interaction, Entity, &mut UiImage, &Card),
        (Changed<Interaction>, With<Button>),
    >,
    mut game_model: ResMut<model::CardGameModel>,
    mut dungeon_model: ResMut<dungeon_model::DungeonGameModel>,
    card_text: Res<CardText>,
    font_handles: Res<FontHandles>,
) {
    let children_result = q_hand_area_children.get_single();
    match children_result {
        Ok(children) => {
            for &child in children.iter() {
                match q_interaction.get_mut(child) {
                    Ok((interaction, mut entity, mut image, card)) => match *interaction {
                        Interaction::Clicked => {
                            game_model.play(card.model.id, &mut dungeon_model);
                            // Remove hover text
                            commands.entity(entity).despawn_descendants();
                        }
                        Interaction::Hovered => {
                            image.texture = card.image_handles.hover.clone();
                            // Add hover text
                            commands.entity(entity).with_children(|parent| {
                                parent.spawn(TextBundle {
                                    text: Text::from_section(
                                        card_text.lookup.get(&card.model.kind).unwrap(),
                                        TextStyle {
                                            font: font_handles.regular.clone(),
                                            font_size: 20.0,
                                            color: Color::BLACK,
                                        },
                                    ),
                                    style: Style {
                                        margin: UiRect {
                                            left: Val::Px(2.0),
                                            right: Val::Px(2.0),
                                            top: Val::Px(2.0),
                                            bottom: Val::Px(2.0),
                                        },
                                        max_size: Size::new(Val::Px(124.0), Val::Px(140.0)),
                                        ..default()
                                    },
                                    ..default()
                                });
                            });
                        }
                        Interaction::None => {
                            image.texture = card.image_handles.face_up.clone();
                            // Remove hover text
                            commands.entity(entity).despawn_descendants();
                        }
                    },
                    _ => (),
                }
            }
        }
        _ => (),
    }
}

pub fn end_turn_btn_interaction(
    mut q_interaction: Query<
        (&Interaction, &mut UiImage),
        (Changed<Interaction>, With<Button>, With<EndTurnBtn>),
    >,
    mut game_model: ResMut<model::CardGameModel>,
    mut dungeon_model: ResMut<dungeon_model::DungeonGameModel>,
    image_handles: Res<ImageHandles>,
) {
    for (interaction, mut image) in &mut q_interaction {
        match *interaction {
            Interaction::Clicked => {
                game_model.end_turn();
                dungeon_model.end_turn();
            }
            Interaction::Hovered => image.texture = image_handles.end_turn_btn.hover.clone(),
            Interaction::None => image.texture = image_handles.end_turn_btn.normal.clone(),
        }
    }
}
