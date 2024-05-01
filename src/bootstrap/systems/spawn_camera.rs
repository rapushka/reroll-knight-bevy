use bevy::prelude::*;

pub fn spawn_camera(
    mut commands: Commands,
) {
    let look_at = Vec3 { x: 0.0, y: 0.0, z: 1.5 };
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 1.5, -3.5).looking_at(look_at, Vec3::Y),
        ..default()
    });
}
