mod scheduler;
mod scripting;
mod spatial;

use crate::scheduler::TickratePlugin;
use crate::scripting::ScriptingPlugin;
use bevy::prelude::*;

#[derive(Component)]
struct Symbol {
    char: char,
    color: Color,
}

fn main() {
    App::new()
        .add_plugins((
            MinimalPlugins,
            bevy::log::LogPlugin {
                // level: bevy::log::Level::TRACE,
                filter: "bevy_ecs=info,scheduler=warn".to_string(),
                ..Default::default()
            },
            ScriptingPlugin,
            TickratePlugin,
        ))
        .run();
}
