//! Pos functions as the source of truth for element translational placement.
//! It should be updated ONLY during `CollisionsSet`, which is a subset of `PhysicsSet`.
//! IPos is updated also in `CollisionsSet`, but is simply the rounded version of Pos.
//! Transforms are updated by looking at the IPos diffs, and adding.
//! This way we avoid global transform shenanigans.

use bevy::prelude::*;

use crate::{
    glue::{fvec::FVec2, Fx},
    physics::PhysicsSet,
};

#[derive(Copy, Clone, Debug, Default, Component)]
#[component(on_add = on_add_pos)]
#[require(Transform, Visibility)]
pub struct Pos {
    pub x: Fx,
    pub y: Fx,
    pub z: Fx,
}
fn on_add_pos(
    mut world: bevy::ecs::world::DeferredWorld,
    eid: Entity,
    _: bevy::ecs::component::ComponentId,
) {
    let me = *world.get::<Pos>(eid).expect("Couldn't get Pos after add");
    match world.get_mut::<Transform>(eid) {
        Some(mut tran) => {
            tran.translation.x = me.x.round().to_num();
            tran.translation.y = me.y.round().to_num();
            tran.translation.z = me.z.to_num();
        }
        None => {
            world
                .commands()
                .entity(eid)
                .insert(Transform::from_translation(me.as_vec2().extend(0.0)));
        }
    }
}
impl Pos {
    pub fn new(x: Fx, y: Fx) -> Self {
        Self {
            x,
            y,
            z: Fx::default(),
        }
    }
    pub fn with_z(mut self, z: Fx) -> Self {
        self.z = z;
        self
    }
    pub fn as_fvec2(&self) -> FVec2 {
        FVec2::new(self.x, self.y)
    }
    pub fn as_ivec2(&self) -> IVec2 {
        self.as_fvec2().round()
    }
    pub fn as_vec2(&self) -> Vec2 {
        self.as_ivec2().as_vec2()
    }
    pub fn translate(&mut self, offset: FVec2) {
        self.x += offset.x;
        self.y += offset.y;
    }
    pub fn translated(&self, offset: FVec2) -> Self {
        Self::new(self.x + offset.x, self.y + offset.y)
    }
}
impl std::ops::Add<FVec2> for Pos {
    type Output = Self;

    fn add(mut self, rhs: FVec2) -> Self::Output {
        self.x += rhs.x;
        self.y += rhs.y;
        self
    }
}
impl std::ops::AddAssign<FVec2> for Pos {
    fn add_assign(&mut self, rhs: FVec2) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}
impl std::ops::Sub<FVec2> for Pos {
    type Output = Self;

    fn sub(mut self, rhs: FVec2) -> Self::Output {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self
    }
}
impl std::ops::SubAssign<FVec2> for Pos {
    fn sub_assign(&mut self, rhs: FVec2) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}
impl std::ops::Neg for Pos {
    type Output = Self;

    fn neg(mut self) -> Self::Output {
        self.x = -self.x;
        self.y = -self.y;
        self
    }
}

fn update_transforms(mut ents: Query<(&Pos, &mut Transform)>) {
    for (pos, mut tran) in &mut ents {
        tran.translation.x = pos.x.round().to_num();
        tran.translation.y = pos.y.round().to_num();
        tran.translation.z = pos.z.to_num();
    }
}

pub(super) fn register_pos(app: &mut App) {
    app.add_systems(
        Update,
        update_transforms
            .in_set(PhysicsSet)
            .in_set(super::CollSet)
            .in_set(super::PosSet),
    );
}
