use crate::player::feet;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use std::collections::HashMap;

// core component for player entity
#[derive(Debug, Component)]
pub struct Player {
    pub speed: f32, // maximum speed
    pub acceleration: f32,
    pub deceleration: f32,
    pub lives: i32,                           // number of hits left until game over
    pub feet_touching_platforms: Vec<Entity>, // playforms entities that player's feet are touching
    pub jump_count: u8,                       // number of jumps currently performed by player
    pub dash_input_timer: Timer,              // timer used to detect double tap of dash input
    pub dash_cooldown_timer: Timer,           // cooldown timer of dash ability
    pub dash_input_count: u8,                 // number of dash inputs performed
    pub is_dashing: bool,                     // whether the player is currently dashing
    pub inventory: Inventory,
}

#[derive(Debug, Default)]
pub struct Inventory {
    pub keycaps: HashMap<char, u32>,
    pub words: HashMap<String, u32>,
}

// TODO: remove
#[derive(Component)]
pub struct PlayerAnimationTimer(pub Timer);

/// spawns player entity
pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    rapier_config: Res<RapierConfiguration>,
) {
    // TODO: remove
    let mut test_words = HashMap::new();
    test_words.insert("till".to_string(), 1);
    test_words.insert("rich".to_string(), 1);
    test_words.insert("weak".to_string(), 1);
    test_words.insert("mode".to_string(), 1);
    test_words.insert("upon".to_string(), 1);
    test_words.insert("core".to_string(), 1);
    test_words.insert("dawn".to_string(), 1);
    test_words.insert("tiny".to_string(), 1);
    test_words.insert("zero".to_string(), 1);
    test_words.insert("kick".to_string(), 1);
    test_words.insert("back".to_string(), 1);
    test_words.insert("show".to_string(), 1);

    let mut test_keycaps = HashMap::new();
    test_keycaps.insert('b', 3);
    test_keycaps.insert('a', 3);
    test_keycaps.insert('c', 3);
    test_keycaps.insert('k', 3);
    test_keycaps.insert('s', 3);
    test_keycaps.insert('h', 3);
    test_keycaps.insert('o', 3);
    test_keycaps.insert('w', 3);

    let texture_handle = asset_server.load("player.png");
    // TODO: store texture atlas params in data file
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(71.0, 67.0), 8, 5);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    // TODO: store player stats in data file
    let player = Player {
        speed: 8.0,
        lives: 6,
        acceleration: 0.12,
        deceleration: 0.1,
        feet_touching_platforms: vec![],
        jump_count: 0,
        dash_input_timer: Timer::from_seconds(0.25, false),
        dash_cooldown_timer: Timer::from_seconds(1.5, false),
        dash_input_count: 1,
        is_dashing: false,
        //inventory: Inventory::default(),
        inventory: Inventory {
            keycaps: test_keycaps,
            words: test_words,
        },
    };

    let collider_size_hx = 30.0 / rapier_config.scale / 2.0;
    let collider_size_hy = 70.0 / rapier_config.scale / 2.0;

    commands
        .spawn_bundle(RigidBodyBundle {
            body_type: RigidBodyType::Dynamic.into(),
            mass_properties: RigidBodyMassPropsFlags::ROTATION_LOCKED.into(),
            position: Vec2::new(0.0, 300.0 / rapier_config.scale).into(),
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(collider_size_hx, collider_size_hy).into(),
            flags: ColliderFlags {
                active_events: ActiveEvents::CONTACT_EVENTS,
                ..Default::default()
            }
            .into(),
            material: ColliderMaterial {
                friction: 0.5,
                restitution: 0.1,
                ..Default::default()
            }
            .into(),
            ..Default::default()
        })
        .insert(ColliderPositionSync::Discrete)
        .insert(Name::new("Player"))
        .insert(player)
        .with_children(|parent| {
            parent
                .spawn_bundle(SpriteSheetBundle {
                    texture_atlas: texture_atlas_handle,
                    transform: Transform {
                        scale: Vec3::new(1.5, 1.5, 1.0),
                        translation: Vec3::new(0.0, 12.0, 100.0),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(PlayerAnimationTimer(Timer::from_seconds(0.1, true)));

            // spawn player feet as child of player
            parent
                .spawn_bundle(ColliderBundle {
                    shape: ColliderShape::cuboid(
                        collider_size_hx,
                        35.0 / rapier_config.scale / 2.0,
                    )
                    .into(),
                    position: [0.0, -35.0 / rapier_config.scale / 2.0].into(),
                    collider_type: ColliderType::Sensor.into(),
                    flags: ColliderFlags {
                        active_events: ActiveEvents::INTERSECTION_EVENTS,
                        ..Default::default()
                    }
                    .into(),
                    ..Default::default()
                })
                .insert(feet::PlayerFeet);
        });
}

// depawn the player entity
pub fn despawn_player(mut commands: Commands, query: Query<Entity, With<Player>>) {
    for entity in query.iter() {
        warn!("Despawning player");
        commands.entity(entity).despawn_recursive();
    }
}
