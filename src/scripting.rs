use bevy::app::Plugin;
use bevy::prelude::*;
use bevy::tasks::{AsyncComputeTaskPool, Task};
use rune::alloc::clone::TryClone;
use rune::runtime::{RuntimeContext, VmResult};
use rune::termcolor::{ColorChoice, StandardStream};
use rune::{Context, Diagnostics, Source, Sources, Unit, Vm};
use std::borrow::BorrowMut;
use std::sync::Arc;
use tokio::runtime::Runtime;

pub struct ScriptingPlugin;

impl Plugin for ScriptingPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, spawn_things)
            .add_systems(Update, start_scripts_system)
            .add_systems(Update, hot_reload_system)
            .add_systems(PostUpdate, execute_scripts_system);
    }
}

#[derive(Component)]
struct VirtualMachine {
    context: Arc<RuntimeContext>,
    unit: Arc<Unit>,
}

#[derive(Component)]
struct Script(String, bool);

#[derive(Component)]
struct ExecutingScript(Task<rune::support::Result<()>>);

fn spawn_things(mut commands: Commands) {
    commands.spawn(Script(
        r#"
            use time::Duration;
            pub async fn main() {
                let i = 0; 
                while i < 10 { 
                    println!("{}", i);
                    time::sleep(Duration::from_secs(1)).await;
                    i += 1;
                }
            }"#
        .to_owned(),
        true,
    ));
}

fn init_vm(script: &Script) -> rune::support::Result<VirtualMachine> {
    let context = rune_modules::default_context()?;
    let runtime = Arc::new(context.runtime()?);

    let mut sources = Sources::new();
    sources.insert(Source::new("entry".to_owned(), script.0.to_owned())?);

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

    Ok(VirtualMachine {
        context: runtime,
        unit: Arc::new(unit),
    })
}

fn hot_reload_system(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Script), With<ExecutingScript>>,
) {
    for (entity, mut script) in &mut query {
        if script.1 {
            script.1 = false;
            commands.entity(entity).remove::<ExecutingScript>();
        }
    }
}

fn start_scripts_system(
    mut commands: Commands,
    query: Query<(Entity, &Script), (Without<VirtualMachine>, Without<ExecutingScript>)>,
) {
    query.iter().for_each(|(entity, script)| {
        let vm = init_vm(script).expect("TODO Handle this properly later pls");
        commands.entity(entity).insert(vm);
    });
}

fn execute_scripts_system(
    mut commands: Commands,
    mut query: Query<(Entity, &VirtualMachine, Option<&mut ExecutingScript>)>,
) {
    for (entity, VirtualMachine { context, unit }, ex) in &mut query {
        match ex {
            Some(mut ex) if ex.0.is_finished() => {
                let vm = Vm::new(context.clone(), unit.clone());
                let execution = vm
                    .send_execute(["main"], ())
                    .expect("TODO pls handle this properly later");
                let task_pool = AsyncComputeTaskPool::get();
                let newtask = task_pool.spawn(async move {
                    execution.async_complete().await.into_result()?;
                    Ok(())
                });

                ex.0 = newtask;
            }
            None => {
                let vm = Vm::new(context.clone(), unit.clone());
                let execution = vm
                    .send_execute(["main"], ())
                    .expect("TODO pls handle this properly later");
                let task_pool = AsyncComputeTaskPool::get();
                let task = task_pool.spawn(async move {
                    execution.async_complete().await.into_result()?;
                    Ok(())
                });

                commands.entity(entity).insert(ExecutingScript(task));
            }
            _ => {}
        }
    }
}
