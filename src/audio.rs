use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioChannel, AudioSource};
use std::collections::HashMap;

use crate::game_states::GameStates;

pub struct GameAudioPlugin;

pub struct AudioChannelsMap(HashMap<String, GameAudio>);

pub struct GameAudioState {
    looped_playing: Option<AudioChannel>,
    single_playing: Option<(String, GameAudioOptions)>,
}

#[derive(Default, Debug, PartialEq)]
pub struct GameAudioOptions {
    pub volume_multiplier: Option<f32>,
    pub handle_idx: Option<usize>,
}

impl GameAudioState {
    pub fn queue_sound(&mut self, channel_name: String, options: GameAudioOptions) {
        self.single_playing = Some((channel_name, options));
    }
}

struct GameAudio {
    channel: AudioChannel,
    handles: Vec<Handle<AudioSource>>,
    current_handle: usize,
    volume: f32,
}

impl Plugin for GameAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(init_audio);
        app.add_system_set(
            SystemSet::on_enter(GameStates::MainMenu).with_system(play_looped_audio),
        );
        app.add_system_set(SystemSet::on_enter(GameStates::Main).with_system(play_looped_audio));
        app.add_system(play_single_sounds);
    }
}

fn init_audio(mut commands: Commands, asset_server: ResMut<AssetServer>) {
    let mut channels = HashMap::new();

    // Looped audio
    channels.insert(
        "main-menu-bg".to_owned(),
        GameAudio {
            channel: AudioChannel::new("main-menu-bg".to_owned()),
            handles: vec![asset_server.load("automation.ogg")],
            volume: 0.1,
            current_handle: 0,
        },
    );
    channels.insert(
        "gameplay-bg".to_owned(),
        GameAudio {
            channel: AudioChannel::new("gameplay-bg".to_owned()),
            handles: vec![asset_server.load("cyberpunk_moonlight_sonata.ogg")],
            volume: 0.1,
            current_handle: 0,
        },
    );

    // Single sounds
    channels.insert(
        "jump-sound".to_owned(),
        GameAudio {
            channel: AudioChannel::new("jump-sound".to_owned()),
            handles: vec![asset_server.load("jump.ogg")],
            volume: 0.5,
            current_handle: 0,
        },
    );
    channels.insert(
        "pickup-sound".to_owned(),
        GameAudio {
            channel: AudioChannel::new("pickup-sound".to_owned()),
            handles: vec![asset_server.load("pickup.ogg")],
            volume: 0.1,
            current_handle: 0,
        },
    );
    channels.insert(
        "powerup-sound".to_owned(),
        GameAudio {
            channel: AudioChannel::new("powerup-sound".to_owned()),
            handles: vec![asset_server.load("powerup.ogg")],
            volume: 0.1,
            current_handle: 0,
        },
    );
    channels.insert(
        "crt-sound".to_owned(),
        GameAudio {
            channel: AudioChannel::new("crt-sound".to_owned()),
            handles: vec![asset_server.load("crt.ogg")],
            volume: 0.8,
            current_handle: 0,
        },
    );
    channels.insert(
        "explosion-sound".to_owned(),
        GameAudio {
            channel: AudioChannel::new("explosion-sound".to_owned()),
            handles: vec![asset_server.load("explosion.ogg")],
            volume: 0.1,
            current_handle: 0,
        },
    );
    channels.insert(
        "dash-sound".to_owned(),
        GameAudio {
            channel: AudioChannel::new("dash-sound".to_owned()),
            handles: vec![asset_server.load("dash.ogg")],
            volume: 0.5,
            current_handle: 0,
        },
    );
    channels.insert(
        "button-sound".to_owned(),
        GameAudio {
            channel: AudioChannel::new("dash-sound".to_owned()),
            handles: vec![asset_server.load("button.ogg")],
            volume: 1.0,
            current_handle: 0,
        },
    );
    channels.insert(
        "footsteps-sound".to_owned(),
        GameAudio {
            channel: AudioChannel::new("footsteps-sound".to_owned()),
            handles: vec![
                asset_server.load("footsteps/0.ogg"),
                asset_server.load("footsteps/1.ogg"),
                asset_server.load("footsteps/2.ogg"),
                asset_server.load("footsteps/3.ogg"),
                asset_server.load("footsteps/4.ogg"),
                asset_server.load("footsteps/5.ogg"),
                asset_server.load("footsteps/6.ogg"),
                asset_server.load("footsteps/7.ogg"),
                asset_server.load("footsteps/8.ogg"),
                asset_server.load("footsteps/9.ogg"),
            ],
            volume: 1.5,
            current_handle: 0,
        },
    );

    commands.insert_resource(GameAudioState {
        looped_playing: None,
        single_playing: None,
    });
    commands.insert_resource(AudioChannelsMap(channels));
}

fn play_looped_audio(
    audio: Res<Audio>,
    game_state: Res<State<GameStates>>,
    game_audio_channels: Res<AudioChannelsMap>,
    mut audio_state: ResMut<GameAudioState>,
) {
    if let Some(channel) = &audio_state.looped_playing {
        audio.stop_channel(&channel);
    }

    let mut channel_name = "";

    match game_state.current() {
        &GameStates::MainMenu => {
            channel_name = "main-menu-bg";
        }
        &GameStates::Main => {
            channel_name = "gameplay-bg";
        }
        _ => {}
    }

    if let Some(game_audio) = game_audio_channels.0.get(&channel_name.to_owned()) {
        let channel = &game_audio.channel;
        let handle = &game_audio.handles[game_audio.current_handle];
        audio.set_volume_in_channel(0.1, &channel);
        audio.play_looped_in_channel(handle.clone(), &channel);
        audio_state.looped_playing = Some(channel.clone());
    }
}

fn play_single_sounds(
    audio: Res<Audio>,
    game_audio_channels: Res<AudioChannelsMap>,
    mut audio_state: ResMut<GameAudioState>,
) {
    if let Some(sound) = &audio_state.single_playing {
        let channel_name = &sound.0;
        let options = &sound.1;

        if let Some(game_audio) = game_audio_channels.0.get(&channel_name.to_owned()) {
            let mut handle_idx = 0;
            let mut volume_multiplier = 1.0;

            if let Some(idx) = options.handle_idx {
                handle_idx = idx;
            }

            if let Some(vol) = options.volume_multiplier {
                volume_multiplier = vol;
            }

            let handle = &game_audio.handles[handle_idx];
            audio.set_volume_in_channel(game_audio.volume * volume_multiplier, &game_audio.channel);
            audio.play_in_channel(handle.clone(), &game_audio.channel);
        }
    }

    audio_state.single_playing = None;
}
