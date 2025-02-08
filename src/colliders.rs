use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;


/// A marker component for walls
#[derive(Component, Default)]
pub struct Wall;

/// Registers walls as collide-able objects
#[derive(Default, Bundle, LdtkIntCell)]
pub struct WallBundle {
    wall: Wall,
    #[bundle()]
    collider: ColliderBundle,
}

/// Defines the physics properties for wall colliders
#[derive(Bundle)]
struct ColliderBundle {
    collider: Collider,
    rigid_body: RigidBody,
}

impl Default for ColliderBundle {
    fn default() -> Self {
        Self {
            collider: Collider::cuboid(8.0, 8.0), // Matches tile size
            rigid_body: RigidBody::Fixed, // Walls don't move
        }
    }
}