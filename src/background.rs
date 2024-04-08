use bevy::{
    prelude::*,
    render::render_resource::{
        Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
    },
};
use noise::{
    core::worley::ReturnType,
    utils::{NoiseMapBuilder, PlaneMapBuilder},
    Add, Cache, Fbm, MultiFractal, NoiseFn, Perlin, Worley,
};
use rand::Rng;

const NEBULA_FREQUENCY: f64 = 2.0;
const STAR_FREQUENCY: f64 = 1.0;
const NEBULA_LACUNARITY: f64 = 3.0;
const NEBULA_PERSISTENCE: f64 = 0.6;

#[derive(Component)]
pub struct Background;
pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, render_background);
    }
}

fn render_background(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
) {
    let size = Extent3d {
        width: 1000,
        height: 1000,
        ..default()
    };
    let mut image = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Bgra8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        ..default()
    };

    // fill image.data with zeroes
    image.resize(size);

    fn nebula_noise() -> impl NoiseFn<f64, 3> {
        let mut rng = rand::thread_rng();
        let seed = rng.gen_range(0..1000);
        let nebula_base = Fbm::<Perlin>::new(seed)
            .set_frequency(NEBULA_FREQUENCY)
            .set_persistence(NEBULA_PERSISTENCE)
            .set_lacunarity(NEBULA_LACUNARITY)
            .set_octaves(5);

        let stars = Worley::new(seed + 1)
            .set_frequency(STAR_FREQUENCY)
            .set_return_type(ReturnType::Distance);

        let nebula = Add::new(nebula_base, stars);
        Cache::new(nebula)
    }
    let nebula_noise = nebula_noise();
    let noise_map = PlaneMapBuilder::new(&nebula_noise)
        .set_size(size.width as usize, size.height as usize)
        .set_x_bounds(-2.0, 2.0)
        .set_y_bounds(-2.0, 2.0)
        .build();
    for y in 0..size.height {
        for x in 0..size.width {
            let value = noise_map.get_value(x as usize, y as usize);
            let value = (value * 0.5 + 0.5) * 255.0;
            let value = value as u8;
            let index = (y * size.width + x) as usize * 4;
            image.data[index] = value;
            image.data[index + 1] = value;
            image.data[index + 2] = value;
            image.data[index + 3] = 255;
        }
    }

    let image_handle = images.add(image);

    let plane_handle = meshes.add(Plane3d::default().mesh().size(1000., 1000.));
    let plane_material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(image_handle),
        reflectance: 0.0,
        unlit: true,
        ..default()
    });

    commands.spawn((
        PbrBundle {
            mesh: plane_handle,
            material: plane_material_handle,
            transform: Transform::from_translation(Vec3::new(0.0, -250., 0.0)),
            ..default()
        },
        Background,
    ));
}
