use bevy::prelude::{Commands, default, Transform};
use bevy::pbr::{PointLight, PointLightBundle};

pub fn spawn_light(
    mut commands: Commands,
) {
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
}
