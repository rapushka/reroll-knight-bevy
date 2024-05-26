use bevy::prelude::*;

pub trait Vec2Ext {
    fn as_flat(&self) -> Vec3;
}

impl Vec2Ext for Vec2 {
    fn as_flat(&self) -> Vec3 { Vec3::new(self.x, 0.0, self.y) }
}
