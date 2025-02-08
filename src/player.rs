use bevy::app::{App, Plugin};
use bevy::input::ButtonInput;
use bevy::prelude::*;
use bevy_ecs_ldtk::app::LdtkEntityAppExt;
use bevy_ecs_ldtk::LdtkEntity;
use bevy_rapier2d::prelude::*;

#[derive(Component, Default)]
pub struct Player;

#[derive(Default, Component)]
pub struct Health {
    current: i32,
    max: i32,
}

#[derive(Default, Component)]
pub struct PlayerStats {
    strength: i32,
    dexterity: i32,
    intelligence: i32,
    luck: i32,
}

#[derive(Bundle)]
struct PhysicsBundle {
    rigid_body: RigidBody,
    collider: Collider,
    velocity: Velocity,
    locked_axes: LockedAxes
}

#[derive(Default, Bundle, LdtkEntity)]
pub struct PlayerBundle {
    health: Health,
    stats: PlayerStats,
    player: Player,
    #[sprite_sheet]
    sprite_sheet: Sprite,
    #[bundle()]
    transform: Transform,
    global_transform: GlobalTransform,
    #[bundle()]
    physics: PhysicsBundle,
}

impl Default for PhysicsBundle {
    fn default() -> Self {
        Self {
            rigid_body: RigidBody::Dynamic,
            collider: Collider::cuboid(8.0, 8.0), // Smaller than tile size to fit
            velocity: Velocity::zero(),
            locked_axes: LockedAxes::ROTATION_LOCKED
        }
    }
}

fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Velocity, With<Player>>,
) {
    let speed = 200.0;

    for mut velocity in query.iter_mut() {
        let mut direction = Vec2::ZERO;

        if keyboard_input.pressed(KeyCode::KeyW) {
            direction.y += 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            direction.y -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            direction.x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            direction.x += 1.0;
        }

        velocity.linvel = direction.normalize_or_zero() * speed;
    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, player_movement)
            .register_ldtk_entity::<PlayerBundle>("Player");
    }
}