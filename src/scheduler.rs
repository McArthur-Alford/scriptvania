use std::time::Duration;

use bevy::{
    app::FixedMain,
    ecs::schedule::{self, ScheduleLabel},
    prelude::*,
};

#[derive(Resource, Default)]
struct Ticker {
    tick: usize,
    timer: Timer,
}

#[derive(ScheduleLabel, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Tick;

pub struct TickratePlugin;

impl Plugin for TickratePlugin {
    fn build(&self, app: &mut App) {
        let mut schedule = Schedule::new(Tick);
        schedule.set_executor_kind(schedule::ExecutorKind::MultiThreaded);

        app.add_schedule(schedule)
            .add_systems(FixedMain, run_ticker)
            .insert_resource(Ticker {
                tick: 0,
                timer: Timer::new(Duration::from_millis(100), TimerMode::Repeating),
            });
    }
}

fn run_ticker(world: &mut World) {
    world.resource_scope(|world, mut ticker: Mut<Ticker>| {
        let Some(world_time) = world.get_resource::<Time>() else {
            return;
        };

        ticker.timer.tick(world_time.delta());

        if ticker.timer.finished() {
            let _ = world.try_run_schedule(Tick);
        }

        info!(target: "scheduler", "Tick {}", ticker.tick);
        ticker.tick += 1;
    });
}
