use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{platforms, player::spawn};

// tag component for player's feet
#[derive(Component)]
pub struct PlayerFeet;

// detects if player is touching platforms
pub fn player_feet_system(
    mut intersection_events: EventReader<IntersectionEvent>,
    player_feet_query: Query<Entity, With<PlayerFeet>>,
    platform_query: Query<Entity, With<platforms::platform::Platform>>,
    mut player_query: Query<&mut spawn::Player>,
) {
    for event in intersection_events.iter() {
        let collider1_entity = event.collider1.entity();
        let collider2_entity = event.collider2.entity();

        for feet_entity in player_feet_query.iter() {
            for mut player in player_query.iter_mut() {
                for platform_entity in platform_query.iter() {
                    // remove index 0 if there are 3 elements
                    if player.feet_touching_platforms.len() > 2 {
                        player.feet_touching_platforms.remove(0);
                    }

                    if event.intersecting {
                        if collider1_entity == feet_entity
                            && !player.feet_touching_platforms.contains(&collider2_entity)
                            && collider2_entity == platform_entity
                        {
                            player.feet_touching_platforms.push(collider2_entity);
                        } else if collider2_entity == feet_entity
                            && !player.feet_touching_platforms.contains(&collider1_entity)
                            && collider1_entity == platform_entity
                        {
                            player.feet_touching_platforms.push(collider1_entity);
                        }
                    } else if collider1_entity == feet_entity {
                        while player.feet_touching_platforms.contains(&collider2_entity) {
                            let index = player
                                .feet_touching_platforms
                                .iter()
                                .position(|x| *x == collider2_entity)
                                .unwrap();
                            player.feet_touching_platforms.remove(index);
                        }
                    } else if collider2_entity == feet_entity {
                        while player.feet_touching_platforms.contains(&collider1_entity) {
                            let index = player
                                .feet_touching_platforms
                                .iter()
                                .position(|x| *x == collider1_entity)
                                .unwrap();
                            player.feet_touching_platforms.remove(index);
                        }
                    }
                }
            }
        }
    }
}
