use bevy::prelude::{Component, Query, Visibility, With};

pub fn show<T>(
    mut entities: Query<&mut Visibility, With<T>>,
) where
    T: Component
{
    for mut entity in entities.iter_mut() {
        *entity = Visibility::Visible;
    }
}

pub fn hide<T>(
    mut entities: Query<&mut Visibility, With<T>>,
) where
    T: Component
{
    for mut entity in entities.iter_mut() {
        *entity = Visibility::Hidden;
    }
}
