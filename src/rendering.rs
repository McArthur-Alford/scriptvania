use bevy::prelude::*;

#[derive(Component)]
struct Window {
    width: i32,
    height: i32,
    depth: i32,
    orientation: IVec3,
}

pub struct RenderingPlugin;

impl Plugin for RenderingPlugin {
    fn build(&self, app: &mut App) {}
}
