use bevy::prelude::ZIndex;

pub const LOADING_CURTAIN: ZIndex = ZIndex::Global(1_000);
pub const MAIN_MENU: ZIndex = ZIndex::Global(100);
pub const GAMEPLAY_HUD: ZIndex = ZIndex::Global(10);