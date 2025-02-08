mod camera;
mod colliders;
mod player;

use crate::camera::{camera_fit_inside_current_level, camera_follow_player};
use crate::colliders::WallBundle;
use crate::player::PlayerPlugin;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_modern_pixel_camera::plugin::PixelCameraPlugin;
use bevy_modern_pixel_camera::prelude::{PixelViewport, PixelZoom};
use bevy_rapier2d::prelude::*;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Zelda-ish".into(),
                        ..default()
                    }),
                    ..default()
                }), // prevents blurry sprites
        )
        .add_plugins(LdtkPlugin)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(PixelCameraPlugin)
        .add_systems(Startup, (setup, setup_physics))
        .add_systems(Update, camera_follow_player)
        // .add_systems(Update, camera_fit_inside_current_level)
        .add_plugins(PlayerPlugin)
        .insert_resource(LevelSelection::index(0))
        .register_ldtk_int_cell::<WallBundle>(1)
        .run();
}

fn setup_physics(mut config: Query<&mut RapierConfiguration>) {
    config.single_mut().gravity = Vec2::ZERO;
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Camera2d,
        Msaa::Off,
        PixelZoom::Fixed(4),
        PixelViewport,
    ));

    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("zelda_like.ldtk").into(),
        ..Default::default()
    });
}
