// use std::sync::Arc;

// use bevy::prelude::*;
// use mlua::Lua;
// use rhai::{Engine, EvalAltResult};

// trait GameComponent {}

// #[derive(Component)]
// struct Position {
//     x: f32,
//     y: f32,
// }

// #[derive(Component)]
// struct Script {
//     script: String,
// }

// #[derive(Component)]
// struct Scriptable {
//     script: String
// }

// // impl GameComponent for Scriptable {}

// fn print_position_system(query: Query<&Position>) {
//     for position in &query {
//         println!("position: {} {}", position.x, position.y);
//     }
// }

// fn run_scriptable(mut query: Query<&mut Scriptable>) {
//     query.par_iter_mut().for_each(|scriptable| {
//         let lua = Lua::new();
//         let map_table = lua.create_table()?;
//         map_table.set(1, "one")?;
//         map_table.set("two", 2)?;

//         lua.globals().set("map_table", map_table)?;
//         println!("{:?}", result);
//     })
// }

// fn spawn_things(mut commands: Commands) {
//     commands.spawn((Position { x: 10., y: 10. }));

//     for i in 0..1000 {
//         let mut engine = Engine::new();
//         // let engine = Engine::new_raw();
//         engine.set_max_operations(10000);
//         // commands.spawn(Scriptable {
//         //     script: format!("let a = 1; for i in 0..1000 {{ a = 1+{}; }}; a", i).to_owned(),
//         //     engine,
//         // });
//         commands.spawn(Scriptable {
//             script: r#"
// while true do
//     print("a")
// end
//         "#.to_owned()
//         });
//     }
// }

use rune::alloc::clone::TryClone;
use rune::termcolor::{ColorChoice, StandardStream};
use rune::{Diagnostics, Vm};

use std::sync::Arc;

#[tokio::main]
async fn main() -> rune::support::Result<()> {
    let context = rune_modules::default_context()?;
    let runtime = Arc::new(context.runtime()?);

    let mut sources = rune::sources!(
        entry => {
            pub fn main(number) {
                let i = 0;
                while (i < 10) {
                    println!("{}", i);
                    i += 1;
                }
            }
        },
        entry2 => {
            pub fn doubler(number) {
                number * 2
            }
        }
    );

    let mut diagnostics = Diagnostics::new();

    let result = rune::prepare(&mut sources)
        .with_context(&context)
        .with_diagnostics(&mut diagnostics)
        .build();

    if !diagnostics.is_empty() {
        let mut writer = StandardStream::stderr(ColorChoice::Always);
        diagnostics.emit(&mut writer, &sources)?;
    }

    let unit = result?;
    // let mut handles: Vec<std::thread::JoinHandle<rune::support::Result<()>>> = Vec::new();
    let mut handles = Vec::new();

    let vm = Vm::new(runtime, Arc::new(unit));

    for i in 0..4 {
        let execution = vm.try_clone()?.send_execute(["main"], (5u32,))?;
        handles.push(tokio::task::spawn(async move {
            execution.async_complete().await.unwrap();
            println!("TICK");
        }));
    }

    // for handle in handles {
    //     handle.await;
    // }
    std::thread::sleep_ms(1000);

    Ok(())
}
