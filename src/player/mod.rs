use bevy::prelude::*;
use bevy_parallax::*;
use ron::de::from_bytes;
mod feet;
mod interact;
mod lives_counter;
mod movement;
mod spawn;

use crate::game_states::GameStates;

pub use self::spawn::{Inventory, Player};

pub struct RunnerPlugin;

impl Plugin for RunnerPlugin {
    fn build(&self, app: &mut App) {
        // add plugins
        app.add_plugin(ParallaxPlugin);

        // insert resources
        app.insert_resource(ParallaxResource {
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
                //.with_system(interact::detect_cheat_code_activation)
                .with_system(interact::show_terminal_toaster_notification),
        );
    }
}
