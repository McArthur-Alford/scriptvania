use bevy::prelude::*;

use crate::scheduler::Tick;

#[derive(Resource)]
struct Spatial {
    size: IVec3,
    entities: Vec<Option<Entity>>,
}

#[derive(Event)]
struct SpatialUpdateEvent {
    position: IVec3,
}

pub struct SpatialPlugin;

impl Plugin for SpatialPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Spatial::new(IVec3::new(100, 100, 100)))
            .add_event::<SpatialUpdateEvent>()
            .add_systems(Tick, apply_velocity);
    }
}

#[derive(Resource)]
struct SpatialChanges(Vec<usize>);

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

pub fn apply_velocity(mut query: Query<(&Velocity, &mut Position)>) {
    query.par_iter_mut().for_each(|(vel, mut pos)| {
        pos.x += vel.x;
        pos.y += vel.y;
        pos.z += vel.z;
    });
}

impl Spatial {
    fn new(size: IVec3) -> Self {
        Self {
            size,
            entities: vec![None; (size.x * size.y * size.z) as usize],
        }
    }

    fn index(&self, mut pos: IVec3) -> Option<usize> {
        pos /= self.size;
        if !((-self.size.x / 2..self.size.x / 2).contains(&pos.x)
            && (-self.size.y / 2..self.size.y / 2).contains(&pos.y)
            && (-self.size.z / 2..self.size.z / 2).contains(&pos.z))
        {
            return None;
        }

        let index = pos.x * (self.size.y * self.size.z) + pos.y * (self.size.z) + pos.z;
        Some(index as usize)
    }

    fn spatial(&self, index: usize) -> Option<IVec3> {
        let index = index as i32;
        if index > self.size.y * self.size.z * self.size.x {
            None
        } else {
            let pos = IVec3::new(
                index / (self.size.y * self.size.z),
                (index % (self.size.y * self.size.z)) / self.size.z,
                index % self.size.z,
            ) - (self.size / 2);
            Some(pos)
        }
    }

    fn get(&self, pos: IVec3) -> Option<Entity> {
        let index = self.index(pos)?;
        self.entities[index]
    }

    fn update(&mut self, pos: IVec3, entity: Entity, mut ev: EventWriter<SpatialUpdateEvent>) {
        let Some(index) = self.index(pos) else {
            return;
        };
        ev.send(SpatialUpdateEvent { position: pos });
        self.entities[index] = Some(entity);
    }
}
