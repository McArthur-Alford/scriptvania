mod scheduler;
mod scripting;
mod spatial;

use crate::scheduler::TickratePlugin;
use crate::scripting::ScriptingPlugin;
use bevy::ecs::component::Tick;
use bevy::math::{uvec2, vec2};
use bevy::prelude::*;
use bevy::render::render_resource::{
    Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
};
use bevy_fast_tilemap::bundle::MapBundleManaged;
use bevy_fast_tilemap::map::Map;
use bevy_fast_tilemap::plugin::FastTileMapPlugin;
use spatial::apply_velocity;

#[derive(Component)]
struct Window {
    width: i32,
    height: i32,
    depth: i32,
    orientation: IVec3,
}

fn startup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<Map>>,
) {
    commands.spawn(Camera2dBundle::default());

    let tiles_texture = asset_server.load("test_tiles.png");

    let map = Map::builder(uvec2(1024, 1024), tiles_texture, vec2(16., 16.)).build_and_set(|_| 2);

    commands.spawn(MapBundleManaged::new(map, materials.as_mut()));

    // let size = Extent3d {
    //     width: 512,
    //     height: 512,
    //     ..Default::default()
    // }

    // let mut image = Image {
    //     texture_descriptor: TextureDescriptor {
    //         label: None,
    //         size: size,
    //         dimension: TextureDimension::D2,
    //         format: TextureFormat::Bgra8UnormSrgb,
    //         mip_level_count: 1,
    //         sample_count: 1,
    //         usage: TextureUsages::TEXTURE_BINDING
    //             | TextureUsages::COPY_DST
    //             | TextureUsages::RENDER_ATTACHMENT,
    //         view_formats: &[]
    //     },
    //     ..Default::default()
    // };

    // image.resize(size);
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(bevy::log::LogPlugin {
                // level: bevy::log::Level::TRACE,
                filter: "bevy_ecs=info,scheduler=warn".to_string(),
                ..Default::default()
            }),
            FastTileMapPlugin::default(),
            ScriptingPlugin,
            TickratePlugin,
        ))
        .add_systems(Startup, startup)
        .add_systems(Tick, apply_velocity)
        .run();
}
