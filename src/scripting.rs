use bevy::app::Plugin;
use bevy::prelude::*;
use bevy::tasks::futures_lite::FutureExt;
use bevy::tasks::{AsyncComputeTaskPool, Task};
use bevy_tokio_tasks::TokioTasksRuntime;
use rune::alloc::clone::TryClone;
use rune::runtime::{RuntimeContext, VmResult};
use rune::termcolor::{ColorChoice, StandardStream};
use rune::{Context, Diagnostics, Source, Sources, Unit, Vm};
use std::borrow::{Borrow, BorrowMut};
use std::sync::Arc;
use tokio::runtime::Runtime;
use tokio::task::JoinHandle;

pub struct ScriptingPlugin;

impl Plugin for ScriptingPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, spawn_things)
            .add_systems(Update, start_scripts_system)
            .add_systems(Update, hot_reload_system)
            .add_systems(PostUpdate, execute_scripts_system)
            .add_plugins(bevy_tokio_tasks::TokioTasksPlugin::default());
    }
}

#[derive(Component)]
struct VirtualMachine {
    context: Arc<RuntimeContext>,
    unit: Arc<Unit>,
}

#[derive(Component)]
struct Script {
    text: String,
    changed: bool,
    result: rune::support::Result<()>,
}

#[derive(Component)]
struct ExecutingScript(Option<JoinHandle<rune::support::Result<()>>>);

fn spawn_things(mut commands: Commands) {
    commands.spawn(Script {
        text: r#"
            pub async fn main() {
                let i = 0; 
                while i < 10 { 
                    println!("Script Counter: {}", i);
                    time::sleep(time::Duration::from_secs(1)).await;
                    i += 1;
                }
            }"#
        .to_owned(),
        changed: true,
        result: Ok(()),
    });
}

fn init_vm(script: &Script) -> rune::support::Result<VirtualMachine> {
    let context = rune_modules::default_context()?;
    let runtime = Arc::new(context.runtime()?);

    let mut sources = Sources::new();
    sources.insert(Source::new("entry".to_owned(), script.text.to_owned())?);

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
        if script.changed {
            script.changed = false;
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
    task_pool: ResMut<TokioTasksRuntime>,
    mut query: Query<(
        Entity,
        &VirtualMachine,
        &mut Script,
        Option<&mut ExecutingScript>,
    )>,
) {
    for (entity, VirtualMachine { context, unit }, mut script, mut ex) in &mut query {
        script.changed = false;
        match ex {
            Some(mut ex) if ex.0.as_ref().is_some_and(|inner| inner.is_finished()) => {
                let jh = std::mem::take(ex.0.borrow_mut()).unwrap();
                let result = task_pool.runtime().block_on(jh).unwrap();
                if result.is_err() {
                    println!("{:?}", result);
                    script.result = result;
                    break;
                    // exit early, leaving the join handle as "None" until something in the future updates the script (such as an update!!!)
                    // also stores the error so we can show the user later
                }

                let vm = Vm::new(context.clone(), unit.clone());
                let execution = vm
                    .send_execute(["main"], ())
                    .expect("TODO pls handle this properly later");
                let newtask = task_pool.spawn_background_task(|mut ctx| async move {
                    execution.async_complete().await.into_result()?;
                    Ok(())
                });

                ex.0 = Some(newtask);
            }
            None => {
                let vm = Vm::new(context.clone(), unit.clone());
                let execution = vm
                    .send_execute(["main"], ())
                    .expect("TODO pls handle this properly later");
                let newtask = task_pool.spawn_background_task(|mut ctx| async move {
                    execution.async_complete().await.into_result()?;
                    Ok(())
                });

                commands
                    .entity(entity)
                    .insert(ExecutingScript(Some(newtask)));
            }
            _ => {}
        }
    }
}
