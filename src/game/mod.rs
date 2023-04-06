use bevy::prelude::*;

mod dungeon_model;
mod dungeon_view;
mod model;
mod view;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(model::setup)
            .add_startup_system(dungeon_model::setup)
            .add_startup_system(view::setup)
            .add_startup_system(dungeon_view::setup)
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
                    view::refresh_fear,
                    view::refresh_despair,
                    view::refresh_doubt,
                    view::refresh_aim,
                    view::refresh_dodge,
                    view::refresh_moves_left,
                    view::cleanup_dead_cards,
                )
                    .after(view::clear_hand_and_play_area),
            )
            .add_system(dungeon_view::refresh_view)
            .add_system(dungeon_view::keyboard_input);
    }
}
