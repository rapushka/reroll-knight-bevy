use bevy::prelude::*;
use crate::common::components::OnTableCoordinates;
use crate::constants;
use crate::prelude::*;
use crate::prelude::world::field::components::SnapToGrid;

pub fn snap_to_grid(
    mut entities: Query<(&mut Transform, &OnTableCoordinates), With<SnapToGrid>>,
) {
    for (mut transform, coordinates) in entities.iter_mut() {
        let scale = 0.17; // TODO: to config

        let tmp: Vec2 = coordinates.coordinates.into();
        let position = tmp.as_flat() * scale;
        transform.translation = position + CELLS_ORIGIN;
    }
}