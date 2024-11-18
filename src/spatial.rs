use bevy::prelude::*;

#[derive(Resource)]
struct Spatial {
    size: IVec3,
    entities: Vec<Entity>,
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
    // We need some universal resource for collision detecting that we can access
    // just a coordinate -> blocked/free map (maybe an entity id for whatever is in the cell)
    query.par_iter_mut().for_each(|(vel, mut pos)| {
        pos.x += vel.x;
        pos.y += vel.y;
        pos.z += vel.z;
    });
}

impl Spatial {
    fn index(&self, mut pos: IVec3) -> Option<usize> {
        pos /= self.size;
        let index = pos.x * (self.size.y * self.size.z) + pos.y * (self.size.z) + pos.z;
        if index > self.size.y * self.size.z * self.size.x {
            None
        } else {
            Some(index as usize)
        }
    }

    fn spatial(&self, index: usize) -> Option<IVec3> {
        let index = index as i32;
        if index > self.size.y * self.size.z * self.size.x {
            None
        } else {
            Some(IVec3::new(
                index / (self.size.y * self.size.z),
                (index % (self.size.y * self.size.z)) / self.size.z,
                index % self.size.z,
            ))
        }
    }

    fn at(&self, pos: IVec3) -> Option<Entity> {
        let index = self.index(pos)?;
        Some(self.entities[index])
    }

    fn update(&self, pos: IVec3, entity: Entity) {
        let index = self.index(pos);
    }
}
