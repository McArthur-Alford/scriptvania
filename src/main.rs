mod scripting;

use crate::scripting::ScriptingPlugin;
use bevy::prelude::*;

struct Position {
    x: i64,
    y: i64,
    z: i64,
}

struct Symbol {
    char: char,
    color: Color,
}

fn main() {
    App::new()
        .add_plugins((MinimalPlugins, ScriptingPlugin))
        .run();
}
