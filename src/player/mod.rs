use bevy::prelude::*;
mod feet;
mod interact;
mod lives_counter;
mod movement;
mod player;
mod spawn;

use std::collections::HashMap;

use crate::game_states::GameStates;

pub use self::spawn::Player;

pub struct RunnerPlugin;

impl Plugin for RunnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(player::PlayerPlugin);

        app.add_system_set(
            SystemSet::on_enter(GameStates::Main)
                .with_system(lives_counter::build_ui)
                .label("lives_counter_build"),
        );
        app.add_system_set(
            SystemSet::on_update(GameStates::Main)
                .with_system(lives_counter::update_counter)
                .after("lives_counter_build"),
        );
    }
}

pub const LETTERS: [char; 36] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
];
pub struct CollectedChars {
    pub values: Vec<char>,
    pub values_map: HashMap<char, u32>,
}

impl CollectedChars {
    pub fn initialize_map(&mut self) {
        for c in LETTERS {
            self.values_map.insert(c, 0);
        }
    }
}
