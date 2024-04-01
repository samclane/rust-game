mod asset_loader;
mod asteroids;
mod camera;
mod collision_detection;
mod debug;
mod despawn;
mod health;
mod menus;
mod movement;
mod planet;
mod schedule;
mod spaceship;
mod splash;
mod stars;
mod state;

use asset_loader::AssetLoaderPlugin;
use asteroids::AsteroidPlugin;
use bevy::prelude::*;
use camera::CameraPlugin;
use collision_detection::CollisionDetectionPlugin;
use debug::DebugPlugin;
use despawn::DespawnPlugin;
use menus::MenusPlugin;
use movement::MovementPlugin;
use planet::PlanetPlugin;
use schedule::SchedulePlugin;
use spaceship::SpaceshipPlugin;
use splash::SplashPlugin;
use stars::StarsPlugin;
use state::StatePlugin;

const WW: u32 = 1000;
const WH: u32 = 800;

fn main() {
    App::new()
        //Bevy built-ins
        .insert_resource(ClearColor(Color::rgb(0.1, 0.0, 0.15)))
        .insert_resource(AmbientLight {
            color: Color::default(),
            brightness: 1000., // Hack to make the light brighter
        })
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "jumpy".to_string(),
                resolution: (WW as f32, WH as f32).into(),
                mode: bevy::window::WindowMode::Windowed,
                ..default()
            }),
            ..default()
        }))
        // User configured plugins
        .add_plugins(AssetLoaderPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(SpaceshipPlugin)
        .add_plugins(AsteroidPlugin)
        .add_plugins(PlanetPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(CollisionDetectionPlugin)
        .add_plugins(DespawnPlugin)
        .add_plugins(SchedulePlugin)
        .add_plugins(StatePlugin)
        .add_plugins(MenusPlugin)
        .add_plugins(SplashPlugin)
        .add_plugins(DebugPlugin)
        .add_plugins(StarsPlugin)
        .run();
}
