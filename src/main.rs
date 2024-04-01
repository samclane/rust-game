use bevy::{diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin}, prelude::*};
use jumpy::{
    asset_loader::AssetLoaderPlugin, asteroids::AsteroidPlugin, camera::CameraPlugin,
    collision_detection::CollisionDetectionPlugin, debug::DebugPlugin, despawn::DespawnPlugin,
    menus::MenusPlugin, movement::MovementPlugin, planet::PlanetPlugin, schedule::SchedulePlugin,
    spaceship::SpaceshipPlugin, splash::SplashPlugin, stars::StarsPlugin, state::StatePlugin,
};

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
        .add_plugins(LogDiagnosticsPlugin::default())
        .add_plugins(FrameTimeDiagnosticsPlugin)
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
