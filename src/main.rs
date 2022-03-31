use bevy::prelude::*;
#[cfg(debug_assertions)]
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_kira_audio::AudioPlugin;
use bevy_rapier2d::prelude::*;

mod audio;
mod camera;
mod cheat_codes;
mod console;
mod effects;
mod enemies;
mod game_over;
mod game_states;
mod interactables;
mod letter_gutter;
mod main_menu;
mod pause_menu;
mod physics;
mod platforms;
mod player;
mod stats;
mod tab_menu;
mod toast;
mod tutorial;

fn main() {
    let mut app = App::new();

    #[cfg(debug_assertions)]
    app.add_plugin(WorldInspectorPlugin::new());

    app.insert_resource(WindowDescriptor {
        resizable: false,
        height: 720.,
        width: 1280.,
        title: "Cheaters Never Win".to_string(),
        ..Default::default()
    })
    .insert_resource(cheat_codes::CheatCodeResource::new())
    .add_plugin(main_menu::MainMenuPlugin)
    .add_plugins(DefaultPlugins)
    .add_plugin(tab_menu::TabMenuPlugin)
    .add_plugin(console::ConsolePlugin)
    .add_plugin(player::RunnerPlugin)
    .add_plugin(pause_menu::PauseMenuPlugin)
    .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
    .add_plugin(physics::PhysicsPlugin)
    .add_plugin(platforms::PlatformsPlugin)
    .add_plugin(enemies::EnemiesPlugin)
    .add_plugin(toast::ToastPlugin)
    .add_plugin(game_over::GameOverPlugin)
    .add_plugin(interactables::InteractablesPlugin)
    .add_plugin(letter_gutter::LetterGutterPlugin)
    .add_plugin(AudioPlugin)
    .add_state(game_states::GameStates::MainMenu)
    .add_plugin(stats::GameStatsPlugin)
    .add_plugin(effects::EffectsPlugin)
    .add_plugin(audio::GameAudioPlugin)
    .add_startup_system(camera::add_camera)
    .add_system_set(
        SystemSet::on_enter(game_states::GameStates::Main).with_system(tutorial::prelude_text),
    )
    .run();
}
