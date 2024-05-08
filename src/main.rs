#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    winit::WinitWindows,
};
use bevy_rapier3d::prelude::*;
use jumpy::{
    asset_loader::AssetLoaderPlugin, asteroids::AsteroidPlugin, background::BackgroundPlugin,
    behaviors::BehaviorsPlugin, camera::CameraPlugin,
    collision_detection::CollisionDetectionPlugin, debug::DebugPlugin, despawn::DespawnPlugin,
    enemy::EnemyPlugin, menus::MenusPlugin, planet::PlanetPlugin,
    post_processing::PostProcessPlugin, schedule::SchedulePlugin, spaceship::SpaceshipPlugin,
    splash::SplashPlugin, stars::StarsPlugin, state::StatePlugin,
};
use winit::window::Icon;

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
        .add_systems(Startup, set_window_icon)
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
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .insert_resource(RapierConfiguration {
            gravity: Vec3::ZERO,
            ..default()
        })
        // User configured plugins
        .add_plugins(AssetLoaderPlugin)
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
        .add_plugins(BehaviorsPlugin)
        .add_plugins(EnemyPlugin)
        .add_plugins(BackgroundPlugin)
        .add_plugins(PostProcessPlugin)
        .run();
}

fn set_window_icon(
    // we have to use `NonSend` here
    windows: NonSend<WinitWindows>,
) {
    // here we use the `image` crate to load our icon data from a png file
    // this is not a very bevy-native solution, but it will do
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open("assets/icons/icon.png")
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };
    let icon = Icon::from_rgba(icon_rgba, icon_width, icon_height).unwrap();

    // do it for all windows
    for window in windows.windows.values() {
        window.set_window_icon(Some(icon.clone()));
    }
}
