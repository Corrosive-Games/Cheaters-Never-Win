use bevy::prelude::*;
use bevy_parallax::ParallaxCameraComponent;

// UI camera tag component
#[derive(Component)]
pub struct UICameraComponent;

// Initialize UI camera and 2D camera with parallax
pub fn add_camera(mut commands: Commands) {
    commands
        .spawn_bundle(UiCameraBundle::default())
        .insert(UICameraComponent);
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(ParallaxCameraComponent);
}
