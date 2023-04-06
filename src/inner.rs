use bevy::prelude::*;

pub struct CardGamePlugin;

impl Plugin for CardGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(model::setup)
            .add_startup_system(view::setup)
            .add_system(view::hand_card_interaction)
            .add_system(view::end_turn_btn_interaction)
            .add_system(
                view::clear_hand_and_play_area
                    .after(view::hand_card_interaction)
                    .after(view::end_turn_btn_interaction),
            )
            .add_systems(
                (
                    view::refresh_hand,
                    view::refresh_deck,
                    view::refresh_play_area,
                    view::refresh_discard_pile,
                    view::refresh_bravery,
                    view::refresh_hope,
                    view::refresh_confidence,
                    view::cleanup_dead_cards,
                )
                    .after(view::clear_hand_and_play_area),
            );
    }
}

mod view {

    use std::collections::HashMap;

    use bevy::prelude::*;

    use super::model;

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
    pub struct EndTurnBtn;

    pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
        // Load images
        let img_card_back = asset_server.load("images/Card Back.png");
        let img_card_inspired = asset_server.load("images/Inspired.png");
        let img_card_inspired_hover = asset_server.load("images/Card Back.png");
        let img_card_peaceful = asset_server.load("images/Peaceful.png");
        let img_card_peaceful_hover = asset_server.load("images/Card Back.png");
        let img_card_dizzy = asset_server.load("images/Dizzy.png");
        let img_card_dizzy_hover = asset_server.load("images/Card Back.png");
        let img_card_determined = asset_server.load("images/Determined.png");
        let img_card_determined_hover = asset_server.load("images/Card Back.png");
        let img_card_proud = asset_server.load("images/Proud.png");
        let img_card_proud_hover = asset_server.load("images/Card Back.png");
        let img_card_satisfied = asset_server.load("images/Satisfied.png");
        let img_card_satisfied_hover = asset_server.load("images/Card Back.png");
        let img_card_angry = asset_server.load("images/Anger.png");
        let img_card_angry_hover = asset_server.load("images/Card Back.png");
        let img_card_stressed = asset_server.load("images/Stressed.png");
        let img_card_stressed_hover = asset_server.load("images/Card Back.png");
        let img_card_tired = asset_server.load("images/Tired.png");
        let img_card_tired_hover = asset_server.load("images/Card Back.png");
        let img_btn_end_turn = asset_server.load("images/end_turn_btn.png");
        let img_btn_end_turn_hover = asset_server.load("images/end_turn_btn_hover.png");

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
        commands.spawn(Camera2dBundle::default());
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
                                    size: Size::new(Val::Px(64.0), Val::Px(72.0)),
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
                        background_color: Color::rgb(0.3, 0.3, 0.0).into(),
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
                                    size: Size::new(Val::Px(64.0), Val::Px(72.0)),
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
                })
                .insert(HudArea1);
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
                        size: Size::height(Val::Px(60.0)),
                        justify_content: JustifyContent::End,
                        ..default()
                    },
                    background_color: Color::rgb(0.3, 0.1, 0.7).into(),
                    ..default()
                })
                .with_children(|hud2| {
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
                        size: Size::height(Val::Px(400.0)),
                        ..default()
                    },
                    background_color: Color::rgb(0.8, 0.8, 0.8).into(),
                    ..default()
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
        mut q_cards: Query<(Entity, &Card, &mut UiImage)>,
        image_handles: Res<ImageHandles>,
        game_model: Res<model::CardGameModel>,
    ) {
        // Catalog the card objects that already exist in the scene
        // to avoid despawning and creating them every frame
        let mut visible_cards: HashMap<u32, Entity> = HashMap::new();
        for (entity, card, mut image) in q_cards.iter_mut() {
            visible_cards.insert(card.model.id, entity);
            // Also make sure all cards are face up
            image.texture = card.image_handles.face_up.clone();
        }

        // Update the hand
        let mut hand_area = commands.entity(q_hand_area.single_mut());
        for card in game_model.hand.iter() {
            match visible_cards.get(&card.id) {
                Some(entity) => {
                    // Set card's parent as the hand area
                    hand_area.push_children(&[*entity]);
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
        mut q_cards: Query<(Entity, &Card, &mut UiImage)>,
        image_handles: Res<ImageHandles>,
        game_model: Res<model::CardGameModel>,
    ) {
        // Catalog the card objects that already exist in the scene
        // to avoid despawning and creating them every frame
        let mut visible_cards: HashMap<u32, Entity> = HashMap::new();
        for (entity, card, mut image) in q_cards.iter_mut() {
            visible_cards.insert(card.model.id, entity);
            // Also make sure all cards are face up
            image.texture = card.image_handles.face_up.clone();
        }

        commands.entity(q_play_area.single_mut()).clear_children();
        // Update the play area
        let mut play_area = commands.entity(q_play_area.single_mut());
        for card in game_model.in_play.iter() {
            match visible_cards.get(&card.id) {
                Some(entity) => {
                    // Set card's parent as the hand area
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
    }

    pub fn refresh_deck(
        mut q_deck_top_visibility: Query<&mut Visibility, (With<DeckTop>, Without<DiscardTop>)>,
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
        mut q_disc_top: Query<
            (&mut Visibility, &mut UiImage),
            (Without<DeckTop>, With<DiscardTop>),
        >,
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

    pub fn hand_card_interaction(
        q_hand_area_children: Query<&Children, With<HandArea>>,
        mut q_interaction: Query<
            (&Interaction, &mut UiImage, &Card),
            (Changed<Interaction>, With<Button>),
        >,
        mut game_model: ResMut<model::CardGameModel>,
    ) {
        let children_result = q_hand_area_children.get_single();
        match children_result {
            Ok(children) => {
                for &child in children.iter() {
                    match q_interaction.get_mut(child) {
                        Ok((interaction, mut image, card)) => match *interaction {
                            Interaction::Clicked => {
                                game_model.play(card.model.id);
                            }
                            Interaction::Hovered => {
                                image.texture = card.image_handles.hover.clone()
                            }
                            Interaction::None => image.texture = card.image_handles.face_up.clone(),
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
        image_handles: Res<ImageHandles>,
    ) {
        for (interaction, mut image) in &mut q_interaction {
            match *interaction {
                Interaction::Clicked => {
                    game_model.end_turn();
                }
                Interaction::Hovered => image.texture = image_handles.end_turn_btn.hover.clone(),
                Interaction::None => image.texture = image_handles.end_turn_btn.normal.clone(),
            }
        }
    }
}

// The card game can be represented entirely by this state object.
// Bevy will manage the actual objects on screen and interactions,
// but this model stores the actual state of the game and performs
// mutations on it.

// The model and the view will communicate via bevy events, which
// will be queued up by the model and then displayed at a sensible
// speed for the player.

mod model {

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
            // Pick a new card to add to your deck
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
            // Stun a demon.
        }

        fn satisfied(&mut self) {
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
}
