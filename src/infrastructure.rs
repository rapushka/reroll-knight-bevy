use bevy::prelude::*;
use bevy_inspector_egui::__macro_exports::bevy_reflect;

#[derive(Resource, Debug)]
pub struct PreparingState<S: States>(pub Option<S>);

impl<S: States> Default for PreparingState<S> {
    fn default() -> Self {
        Self(None)
    }
}

impl<S: States> PreparingState<S> {
    pub fn set(&mut self, state: S) {
        self.0 = Some(state);
    }
}

#[derive(Component)]
pub struct Preparing {}