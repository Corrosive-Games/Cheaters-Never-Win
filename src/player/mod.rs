use bevy::prelude::*;
use bevy_parallax::*;
use ron::de::from_bytes;
mod feet;
mod interact;
mod lives_counter;
mod movement;
mod spawn;

use std::collections::HashMap;

use crate::game_states::GameStates;

pub use self::spawn::Player;

pub struct RunnerPlugin;

impl Plugin for RunnerPlugin {
    fn build(&self, app: &mut App) {
        // add plugins
        app.add_plugin(ParallaxPlugin);

        // initialize list of collected characters
        let mut collected_chars_list = CollectedChars {
            values: Vec::new(),
            values_map: HashMap::new(),
        };
        collected_chars_list.initialize_map();

        // insert resources
        app.insert_resource(collected_chars_list)
            .insert_resource(ParallaxResource {
                layer_data: from_bytes::<Vec<LayerData>>(include_bytes!(
                    "../../data/parallax_layers.ron"
                ))
                .unwrap(),
                ..Default::default()
            })
            .insert_resource(movement::PlayerAnimationResource {
                run_right: movement::AnimationData {
                    length: 8,
                    offset: 0,
                },
                jump: movement::AnimationData {
                    length: 4,
                    offset: 8,
                },
                idle: movement::AnimationData {
                    length: 4,
                    offset: 16,
                },
                run_left: movement::AnimationData {
                    length: 8,
                    offset: 24,
                },
                dash_attack: movement::AnimationData {
                    length: 8,
                    offset: 32,
                },
                run_step_counter: 0,
            });

        // add events
        app.add_event::<movement::GameOverEvent>();

        // add systems
        app.add_system_set(
            SystemSet::on_enter(GameStates::Main)
                .with_system(lives_counter::build_ui)
                .label("lives_counter_build")
                .with_system(spawn::spawn_player.after("setup_physics")),
        );
        app.add_system_set(
            SystemSet::on_update(GameStates::Main)
                .with_system(lives_counter::update_counter)
                .after("lives_counter_build")
                .with_system(feet::player_feet_system)
                .label("player_feet"),
        );

        app.add_system_set(SystemSet::on_exit(GameStates::Main).with_system(spawn::despawn_player));

        app.add_system_set(
            SystemSet::on_update(GameStates::Main)
                .with_system(movement::follow_player_camera)
                .with_system(movement::animate_sprite)
                .with_system(movement::move_player_system)
                .after("player_feet")
                .with_system(interact::detect_char_interactable)
                .with_system(movement::player_collision_system)
                .with_system(movement::player_fall_damage)
                .with_system(interact::detect_cheat_code_activation)
                .with_system(interact::show_terminal_toaster_notification),
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
