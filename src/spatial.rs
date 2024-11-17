use bevy::prelude::*;

#[derive(Component)]
struct Position {
    x: i64,
    y: i64,
    z: i64,
}

#[derive(Component)]
struct Velocity {
    x: i64,
    y: i64,
    z: i64,
}

fn apply_velocity(mut query: Query<(&Velocity, &mut Position)>) {
    // We need some universal resource for collision detecting that we can access
    // just a coordinate -> blocked/free map (maybe an entity id for whatever is in the cell)
    query.par_iter_mut().for_each(|(vel, mut pos)| {
        pos.x += vel.x;
        pos.y += vel.y;
        pos.z += vel.z;
    });
}
