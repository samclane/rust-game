use bevy::prelude::*;

#[derive(Resource, Debug, Default)]
pub struct SceneAssets {
    pub asteroids: Handle<Scene>,
    pub spaceship: Handle<Scene>,
    pub missiles: Handle<Scene>,
}

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SceneAssets>()
            .add_systems(Startup, load_assets);
    }
}

fn load_assets(mut scene_assets: ResMut<SceneAssets>, asset_server: Res<AssetServer>) {
    scene_assets.asteroids = asset_server.load("Rock.glb#Scene0");
    scene_assets.spaceship = asset_server.load("Spaceship.glb#Scene0");
    scene_assets.missiles = asset_server.load("Bush.glb#Scene0");
}
