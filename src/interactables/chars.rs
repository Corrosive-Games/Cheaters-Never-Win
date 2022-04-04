use bevy::prelude::*;

use super::{InteractableComponent, InteractableType};

#[derive(Component)]
pub struct KeycapComponent {
    pub value: char,
}

#[derive(Component)]
pub struct WordComponent {
    pub value: String,
}

pub fn spawn_keycap_pickup(
    commands: &mut Commands,
    asset_server: &AssetServer,
    texture_atlases: &mut Assets<TextureAtlas>,
    value: String,
    position: &Vec2,
) {
    let interactable_type = InteractableType::CharText;

    let entity_commands = commands
        .spawn()
        .insert(InteractableComponent {
            interactable_type,
            range: 25.0,
        })
        .insert(Transform {
            translation: Vec3::new(position.x, position.y, 99.0),
            ..Default::default()
        })
        .insert(Name::new("Keycap Pickup"));

    if value.len() == 1 {
        entity_commands.insert(KeycapComponent {
            value: value.chars().next().unwrap(),
        });
    } else {
        entity_commands.insert(WordComponent { value });
    }

    // TODO: implement for odd numbered character words and single character words
    entity_commands.with_children(|parent| {
        for (i, c) in value.chars().enumerate() {
            // get the texture animation for the keycap
            let path = format!("chars/{}_key.png", c);

            let texture_handle = asset_server.load(&path);
            let texture_atlas =
                TextureAtlas::from_grid(texture_handle, Vec2::new(16.0, 16.0), 2, 2);
            let texture_atlas_handle = texture_atlases.add(texture_atlas);

            parent.spawn_bundle(SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                transform: Transform {
                    scale: Vec3::new(2.0, 2.0, 1.0),
                    translation: Vec3::new(
                        (((value.len() as f32 / 2.0) - i as f32) * -32.0) + 16.0,
                        0.0,
                        0.0,
                    ),
                    ..Default::default()
                },
                ..Default::default()
            });
        }
    });
}
