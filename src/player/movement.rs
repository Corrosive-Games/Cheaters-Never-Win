use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
use bevy_parallax::{ParallaxCameraComponent, ParallaxMoveEvent};
use bevy_rapier2d::prelude::*;
use rand::Rng;

use crate::{
    audio::{GameAudioOptions, GameAudioState},
    cheat_codes::{self, CheatCodeKind},
    effects, enemies, game_states,
    game_states::GameStates,
    physics,
    player::spawn,
};

// move player entity
pub fn move_player_system(
    keyboard_input: Res<Input<KeyCode>>,
    rapier_config: Res<RapierConfiguration>,
    mut query: Query<(
        &mut spawn::Player,
        &mut RigidBodyVelocityComponent,
        &RigidBodyMassPropsComponent,
    )>,
    mut animation_query: Query<&mut TextureAtlasSprite, With<spawn::PlayerAnimationTimer>>,
    player_animation_resource: Res<PlayerAnimationResource>,
    cheat_codes: ResMut<cheat_codes::CheatCodeResource>,
    mut game_audio_state: ResMut<GameAudioState>,
    time: Res<Time>,
) {
    for (mut player, mut rb_vel, rb_mprops) in query.iter_mut() {
        // update acceleration value
        if cheat_codes.is_code_activated(&CheatCodeKind::SpeedBoost3) {
            player.acceleration = 0.15;
            player.deceleration = 0.4;
            player.speed = 8.9;
        } else if cheat_codes.is_code_activated(&CheatCodeKind::SpeedBoost2) {
            player.acceleration = 0.14;
            player.deceleration = 0.3;
            player.speed = 8.6;
        } else if cheat_codes.is_code_activated(&CheatCodeKind::SpeedBoost1) {
            player.acceleration = 0.13;
            player.deceleration = 0.2;
            player.speed = 8.3;
        }

        let _up = keyboard_input.pressed(KeyCode::W);
        let _down = keyboard_input.pressed(KeyCode::S);
        let right = keyboard_input.pressed(KeyCode::D);
        let dash = keyboard_input.just_released(KeyCode::D);

        let jump = cheat_codes.is_code_activated(&CheatCodeKind::Jump)
            && keyboard_input.just_pressed(KeyCode::Space)
            && !player.feet_touching_platforms.is_empty()
            || (cheat_codes.is_code_activated(&CheatCodeKind::DoubleJump)
                && keyboard_input.just_pressed(KeyCode::Space));

        let left = cheat_codes.is_code_activated(&CheatCodeKind::MoveLeft)
            && keyboard_input.pressed(KeyCode::A);

        let x_axis = -(left as i8) + right as i8;

        if dash && cheat_codes.is_code_activated(&CheatCodeKind::Dash) {
            if player.dash_input_count == 0 {
                player.dash_input_count = 1;
                player.dash_input_timer.reset();
            } else if player.dash_input_count == 1 && player.dash_cooldown_timer.finished() {
                rb_vel.apply_impulse(rb_mprops, Vec2::new(1000.0, 0.0).into());
                player.is_dashing = true;
                player.dash_cooldown_timer.reset()
            }
        }

        if player.dash_input_count == 1 {
            player.dash_input_timer.tick(time.delta());
            if player.dash_input_timer.just_finished() {
                player.dash_input_count = 0;
            }
        }
        if !player.is_dashing {
            //decrease dash cooldown
            player.dash_cooldown_timer.tick(time.delta());

            if x_axis != 0 {
                rb_vel.linvel.x += player.acceleration * (x_axis as f32) * rapier_config.scale;
                if rb_vel.linvel.x.abs() > player.speed * rapier_config.scale {
                    rb_vel.linvel.x = (rb_vel.linvel.x / rb_vel.linvel.x.abs())
                        * player.speed
                        * rapier_config.scale;
                }
            } else if rb_vel.linvel.x.abs() > 0.4 {
                // decelerate
                rb_vel.linvel.x -= player.deceleration
                    * (rb_vel.linvel.x / rb_vel.linvel.x.abs())
                    * rapier_config.scale;
            } else {
                rb_vel.linvel.x = 0.0;
            }
        } else {
            rb_vel.linvel.y = 0.0;
            rb_vel.linvel.x += player.acceleration * 2.0;
        }

        if jump {
            if !player.feet_touching_platforms.is_empty() {
                // single jump
                physics::jump(1500.0, &mut rb_vel, rb_mprops);
                if cheat_codes.is_code_activated(&CheatCodeKind::DoubleJump) {
                    player.jump_count = 1;
                } else {
                    player.jump_count = 0;
                }
            } else if player.jump_count == 1 {
                // double jump
                rb_vel.linvel.y = 0.0;
                physics::jump(1500.0, &mut rb_vel, rb_mprops);
                for mut sprite in animation_query.iter_mut() {
                    sprite.index = player_animation_resource.jump.offset;
                }
                game_audio_state.queue_sound(
                    "jump-sound".to_owned(),
                    GameAudioOptions {
                        ..Default::default()
                    },
                );
                player.jump_count = 0;
            }
        }
    }
}

// follow player's movement with the camera with parallax effect
pub fn follow_player_camera(
    player: Query<&Transform, With<spawn::Player>>,
    camera: Query<&Transform, (With<ParallaxCameraComponent>, Without<spawn::Player>)>,
    mut move_event_writer: EventWriter<ParallaxMoveEvent>,
) {
    if let Some(player_transform) = player.iter().next() {
        if let Some(camera_transform) = camera.iter().next() {
            // set speed of parallax to difference in player and camera x positions
            move_event_writer.send(ParallaxMoveEvent {
                camera_move_speed: player_transform.translation.x - camera_transform.translation.x,
            });
        }
    }
}

// handle player collisions
pub fn player_collision_system(
    mut commands: Commands,
    mut player_query: Query<(Entity, &mut spawn::Player)>,
    enemy_query: Query<(Entity, &Transform), With<enemies::Enemy>>,
    mut contact_events: EventReader<ContactEvent>,
    mut game_over_event: EventWriter<GameOverEvent>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
    mut game_state: ResMut<State<game_states::GameStates>>,
    mut game_audio_state: ResMut<GameAudioState>,
) {
    for contact_event in contact_events.iter() {
        if let ContactEvent::Started(h1, h2) = contact_event {
            for (player_entity, mut player) in player_query.iter_mut() {
                for (enemy_entity, enemy_transform) in enemy_query.iter() {
                    if h1.entity() == player_entity && h2.entity() == enemy_entity
                        || h2.entity() == player_entity && h1.entity() == enemy_entity
                    {
                        if !player.is_dashing {
                            player.lives -= 1;
                        }
                        commands.entity(enemy_entity).despawn();
                        // spawn explostion
                        effects::spawn_explosion(
                            enemy_transform.translation.xy(),
                            &mut commands,
                            &asset_server,
                            &mut texture_atlases,
                        );
                        game_audio_state.queue_sound(
                            "explosion-sound".to_owned(),
                            GameAudioOptions {
                                ..Default::default()
                            },
                        );
                        if player.lives <= 0 {
                            game_over_event.send(GameOverEvent);
                            game_state.push(game_states::GameStates::GameOver).unwrap();
                        }
                    }
                }
            }
        }
    }
}

// TODO: remove (generalize animation)
pub struct PlayerAnimationResource {
    pub run_right: AnimationData,
    pub run_left: AnimationData,
    pub jump: AnimationData,
    pub idle: AnimationData,
    pub dash_attack: AnimationData,
    pub run_step_counter: u32,
}

// TODO: remove (generalize animation)
pub struct AnimationData {
    pub length: usize,
    pub offset: usize,
}

// TODO:: move/remove (generalize animation)
pub fn animate_sprite(
    time: Res<Time>,
    mut player_animation_resource: ResMut<PlayerAnimationResource>,
    mut player_query: Query<(&mut spawn::Player, &RigidBodyVelocityComponent)>,
    mut query: Query<(&mut spawn::PlayerAnimationTimer, &mut TextureAtlasSprite)>,
    mut game_audio_state: ResMut<GameAudioState>,
    rapier_config: Res<RapierConfiguration>,
) {
    for (mut player, rb_vel) in player_query.iter_mut() {
        for (mut timer, mut sprite) in query.iter_mut() {
            timer.0.tick(time.delta());
            if timer.0.just_finished() {
                if player.is_dashing {
                    if sprite.index
                        == player_animation_resource.dash_attack.offset
                            + player_animation_resource.dash_attack.length
                            - 1
                    {
                        player.is_dashing = false;
                    }
                    if sprite.index < player_animation_resource.dash_attack.offset
                        || sprite.index
                            >= (player_animation_resource.dash_attack.length
                                + player_animation_resource.dash_attack.offset)
                    {
                        sprite.index = player_animation_resource.dash_attack.offset;
                        game_audio_state.queue_sound(
                            "dash-sound".to_owned(),
                            GameAudioOptions {
                                ..Default::default()
                            },
                        );
                    } else if sprite.index
                        < (player_animation_resource.dash_attack.length
                            + player_animation_resource.dash_attack.offset)
                            - 1
                    {
                        sprite.index += 1;
                    }
                } else if player.feet_touching_platforms.is_empty() {
                    // player is jumping
                    if sprite.index < player_animation_resource.jump.offset
                        || sprite.index
                            >= (player_animation_resource.jump.length
                                + player_animation_resource.jump.offset)
                    {
                        sprite.index = player_animation_resource.jump.offset;
                        game_audio_state.queue_sound(
                            "jump-sound".to_owned(),
                            GameAudioOptions {
                                ..Default::default()
                            },
                        );
                    } else if sprite.index
                        < (player_animation_resource.jump.length
                            + player_animation_resource.jump.offset)
                            - 1
                    {
                        sprite.index += 1;
                    }
                } else {
                    if rb_vel.linvel.x > 0.0 {
                        // player is running right
                        if sprite.index >= player_animation_resource.run_right.length {
                            sprite.index = 0;
                            player_animation_resource.run_step_counter = 0;
                        } else {
                            sprite.index =
                                (sprite.index + 1) % player_animation_resource.run_right.length;
                            player_animation_resource.run_step_counter += 1;
                        }
                        if player_animation_resource.run_step_counter % 3 == 0 {
                            game_audio_state.queue_sound(
                                "footsteps-sound".to_owned(),
                                GameAudioOptions {
                                    volume_multiplier: Some(
                                        rb_vel.linvel.x.abs()
                                            / (player.speed * rapier_config.scale),
                                    ),
                                    handle_idx: Some(rand::thread_rng().gen_range(0..10)),
                                },
                            )
                        }
                    } else if rb_vel.linvel.x < 0.0 {
                        //player is running left
                        if sprite.index < player_animation_resource.run_left.offset
                            || sprite.index
                                >= (player_animation_resource.run_left.length
                                    + player_animation_resource.run_left.offset)
                        {
                            sprite.index = player_animation_resource.run_left.offset;
                            player_animation_resource.run_step_counter = 0;
                        } else {
                            sprite.index = ((sprite.index + 1)
                                % player_animation_resource.run_left.length)
                                + player_animation_resource.run_left.offset;
                            player_animation_resource.run_step_counter += 1;
                        }
                        if player_animation_resource.run_step_counter % 3 == 0 {
                            game_audio_state.queue_sound(
                                "footsteps-sound".to_owned(),
                                GameAudioOptions {
                                    volume_multiplier: Some(
                                        rb_vel.linvel.x.abs()
                                            / (player.speed * rapier_config.scale),
                                    ),
                                    handle_idx: Some(rand::thread_rng().gen_range(0..10)),
                                },
                            )
                        }
                    } else {
                        //player is idling
                        if sprite.index < player_animation_resource.idle.offset
                            || sprite.index
                                >= (player_animation_resource.idle.length
                                    + player_animation_resource.idle.offset)
                        {
                            sprite.index = player_animation_resource.idle.offset;
                        } else {
                            sprite.index = ((sprite.index + 1)
                                % player_animation_resource.idle.length)
                                + player_animation_resource.idle.offset;
                        }
                    }
                }
            }
        }
    }
}

pub struct GameOverEvent;

pub fn player_fall_damage(
    mut player_query: Query<(&mut spawn::Player, &Transform)>,
    mut game_over_event: EventWriter<GameOverEvent>,
    mut game_state: ResMut<State<GameStates>>,
) {
    for (mut player, transform) in player_query.iter_mut() {
        if transform.translation.y < -400.0 {
            player.lives = 0;
            game_over_event.send(GameOverEvent);
            info!("Fell down hole");
            game_state.push(GameStates::GameOver).unwrap();
        }
    }
}
