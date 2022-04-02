use crate::{
    audio::{GameAudioOptions, GameAudioState},
    //cheat_codes::{CheatCodeKind, CheatCodeResource},
    interactables::{CharTextComponent, InteractableComponent, InteractableType},
    player::{spawn, CollectedChars},
    toast::ShowToast,
};
use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioChannel};
use std::time::Duration;

// TODO: move/remove belongs in cheat_codes
/*
pub fn detect_cheat_code_activation(
    mut query: Query<&mut spawn::Player>,
    mut cheat_codes: ResMut<CheatCodeResource>,
) {
    for mut player in query.iter_mut() {
        if cheat_codes.is_code_activated(&CheatCodeKind::ExtraLife) {
            player.lives += 1;
            cheat_codes.deactivate_code(&CheatCodeKind::ExtraLife);
        }
    }
}
*/

// TODO: generalize for all interactable entities
pub fn show_terminal_toaster_notification(
    player_query: Query<&Transform, With<spawn::Player>>,
    mut toast_writer: EventWriter<ShowToast>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    let right = keyboard_input.just_released(KeyCode::D);
    let left = keyboard_input.just_released(KeyCode::A);

    if let Some(player_transform) = player_query.iter().next() {
        if (right || left)
            && player_transform.translation.x > 1150.
            && player_transform.translation.x <= 1300.
        {
            let value = String::from("Press E to access console");
            toast_writer.send(ShowToast {
                value,
                duration: Duration::from_secs(3),
            });
        }
    }
}

// Pick up keycaps when player runs into them
pub fn detect_char_interactable(
    mut commands: Commands,
    mut collected_chars: ResMut<CollectedChars>,
    mut game_audio_state: ResMut<GameAudioState>,
    player_query: Query<&Transform, With<spawn::Player>>,
    interactable_query: Query<(
        Entity,
        &InteractableComponent,
        &Transform,
        &CharTextComponent,
    )>,
    audio: Res<Audio>,
) {
    if let Some(player_transform) = player_query.iter().next() {
        for (entity, interactable, transform, char_component) in interactable_query.iter() {
            match interactable.interactable_type {
                InteractableType::CharText => {
                    let distance_x = player_transform.translation.x - transform.translation.x;
                    let distance_y = player_transform.translation.y - transform.translation.y;
                    let range = interactable.range;

                    if distance_x <= range
                        && distance_x >= -range
                        && distance_y <= range
                        && distance_y >= -range
                    {
                        let audio_channel = AudioChannel::new("sfx-channel".to_owned());
                        audio.set_volume_in_channel(0.3, &audio_channel);
                        game_audio_state.queue_sound(
                            "pickup-sound".to_owned(),
                            GameAudioOptions {
                                ..Default::default()
                            },
                        );
                        collected_chars.values.push(char_component.value);

                        let char_entry = collected_chars.values_map.get(&char_component.value);
                        if let Some(_count) = char_entry {
                            *collected_chars
                                .values_map
                                .get_mut(&char_component.value)
                                .unwrap() += 1;
                        }

                        commands.entity(entity).despawn();
                    }
                }
                _ => {}
            }
        }
    }
}
