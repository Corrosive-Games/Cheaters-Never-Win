use std::time::Duration;

use crate::enemies::Enemy;
use crate::{
    effects,
    game_states::GameStates,
    physics, platforms,
    player::{feet, interact, movement, spawn},
};
use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioChannel};
use bevy_parallax::*;
use bevy_rapier2d::prelude::*;
use std::collections::HashMap;

use super::CollectedChars;
use crate::audio::{GameAudioOptions, GameAudioState};
use crate::cheat_codes::{CheatCodeKind, CheatCodeResource};
use crate::interactables::{CharTextComponent, InteractableComponent, InteractableType};
use crate::toast::ShowToast;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        let mut collected_chars_list = CollectedChars {
            values: Vec::new(),
            values_map: HashMap::new(),
        };
        collected_chars_list.initialize_map();
        app.insert_resource(collected_chars_list)
            .insert_resource(ParallaxResource {
                layer_data: vec![
                    LayerData {
                        speed: 0.98,
                        path: "cyberpunk_back.png".to_string(),
                        tile_size: Vec2::new(96.0, 160.0),
                        cols: 1,
                        rows: 1,
                        scale: 4.5,
                        z: 0.0,
                        ..Default::default()
                    },
                    LayerData {
                        speed: 0.92,
                        path: "cyberpunk_middle.png".to_string(),
                        tile_size: Vec2::new(144.0, 160.0),
                        cols: 1,
                        rows: 1,
                        scale: 4.5,
                        z: 1.0,
                        ..Default::default()
                    },
                    LayerData {
                        speed: 0.82,
                        path: "cyberpunk_front.png".to_string(),
                        tile_size: Vec2::new(272.0, 160.0),
                        cols: 1,
                        rows: 1,
                        scale: 4.5,
                        z: 2.0,
                        ..Default::default()
                    },
                ],
                ..Default::default()
            })
            .add_plugin(ParallaxPlugin)
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
            })
            .add_system_set(
                SystemSet::on_enter(GameStates::Main)
                    .with_system(spawn::spawn_player.after("setup_physics")),
            )
            .add_event::<movement::GameOverEvent>()
            .add_system_set(
                SystemSet::on_update(GameStates::Main)
                    .with_system(feet::player_feet_system)
                    .label("player_feet"),
            )
            .add_system_set(SystemSet::on_exit(GameStates::Main).with_system(spawn::despawn_player))
            .add_system_set(
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
