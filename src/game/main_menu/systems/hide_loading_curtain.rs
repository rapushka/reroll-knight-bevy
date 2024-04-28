use bevy::prelude::*;
use crate::ui::components::*;

pub fn hide_loading_curtain(
    mut commands: Commands,
    main_menus: Query<Entity, With<LoadingCurtain>>,
) {
    for curtain in main_menus.iter() {
        commands.entity(curtain).despawn_recursive();
    }
}

pub fn hide<T>(
    mut commands: Commands,
    mut entities: Query<&mut Visibility, With<T>>,
) where
    T: Component
{
    for mut entity in entities.iter_mut() {
        *entity = Visibility::Hidden;
        // commands.entity(entity).despawn_recursive();
    }
}
