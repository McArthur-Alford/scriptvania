mod rendering;
mod scheduler;
mod scripting;
mod spatial;

use crate::scheduler::TickratePlugin;
use crate::scripting::ScriptingPlugin;
use bevy::math::{uvec2, vec2};
use bevy::prelude::*;
use bevy_fast_tilemap::bundle::MapBundleManaged;
use bevy_fast_tilemap::map::Map;
use bevy_fast_tilemap::plugin::FastTileMapPlugin;
use rendering::RenderingPlugin;
use spatial::SpatialPlugin;

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
                filter: "bevy_ecs=info,scheduler=warn,wgpu_hal=error,naga=error".to_string(),
                ..Default::default()
            }),
            FastTileMapPlugin::default(),
            ScriptingPlugin,
            TickratePlugin,
            SpatialPlugin,
            RenderingPlugin,
        ))
        .add_systems(Startup, startup)
        .run();
}
