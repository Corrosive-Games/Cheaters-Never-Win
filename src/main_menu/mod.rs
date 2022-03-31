use bevy::{app::AppExit, prelude::*};

use crate::{
    audio::{GameAudioOptions, GameAudioState},
    game_states::GameStates,
    pause_menu::button::{UIButton, HOVERED_BUTTON, NORMAL_BUTTON, PRESSED_BUTTON},
};
mod ui;

#[derive(Component)]
pub struct MainMenuEntity;

#[derive(Component)]
pub struct MainBackgroundLayer;

pub struct MainMenuPlugin;
impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        // on enter
        app.add_system_set(
            SystemSet::on_enter(GameStates::MainMenu)
                .with_system(ui::build_ui)
                .with_system(spawn_main_menu_background),
        );
        // on update
        app.add_system_set(SystemSet::on_update(GameStates::MainMenu).with_system(button_handler));
        // on exit
        app.add_system_set(
            SystemSet::on_exit(GameStates::MainMenu)
                .with_system(destroy_menu)
                .with_system(despawn_main_menu_background),
        );
    }
}

fn destroy_menu(mut commands: Commands, query: Query<Entity, With<MainMenuEntity>>) {
    info!("[MainMenuPlugin] Destroying state entities before exiting...");
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    info!("[MainMenuPlugin] Exiting state");
}

pub fn button_handler(
    mut interaction_query: Query<(&Interaction, &mut UiColor, &UIButton), Changed<Interaction>>,
    mut game_state: ResMut<State<GameStates>>,
    mut exit: EventWriter<AppExit>,
    mut game_audio_state: ResMut<GameAudioState>,
) {
    for (interaction, mut color, button) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                game_audio_state.queue_sound(
                    "button-sound".to_owned(),
                    GameAudioOptions {
                        ..Default::default()
                    },
                );
                *color = PRESSED_BUTTON.into();
                match button.name.as_str() {
                    "play" => {
                        game_state.set(GameStates::Main).unwrap();
                    }
                    "quit" => {
                        exit.send(AppExit);
                    }
                    _ => {}
                }
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

fn spawn_main_menu_background(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("cyberpunk-street.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(608., 192.), 1, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle.clone(),
            transform: Transform {
                scale: Vec3::new(3.0, 4.0, 1.0),
                translation: Vec3::new(0.0, 0.0, 3.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(MainBackgroundLayer);
}

fn despawn_main_menu_background(
    mut commands: Commands,
    background_query: Query<Entity, With<MainBackgroundLayer>>,
) {
    for entity in background_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
