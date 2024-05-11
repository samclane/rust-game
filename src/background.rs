use bevy::{
    prelude::*,
    render::render_resource::{
        Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
        TextureViewDescriptor, TextureViewDimension,
    },
};
use noise::{
    core::worley::ReturnType,
    utils::{ColorGradient, NoiseMapBuilder, SphereMapBuilder},
    Add, Cache, Fbm, MultiFractal, NoiseFn, Perlin, Worley,
};
use rand::Rng;

const WIDTH: u32 = 1000;
const HEIGHT: u32 = 6000;
const NEBULA_FREQUENCY: f64 = 2.0;
const NEBULA_OCATAVES: usize = 5;
const STAR_FREQUENCY: f64 = 1.0;
const NEBULA_LACUNARITY: f64 = 3.0;
const NEBULA_PERSISTENCE: f64 = 0.6;

#[derive(Component)]
pub struct Background;

pub fn build_image(mut images: ResMut<Assets<Image>>) -> Handle<Image> {
    let size = Extent3d {
        width: WIDTH,
        height: HEIGHT,
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
        let seed = rng.gen_range(0..u32::MAX);
        let nebula_base = Fbm::<Perlin>::new(seed)
            .set_frequency(NEBULA_FREQUENCY)
            .set_persistence(NEBULA_PERSISTENCE)
            .set_lacunarity(NEBULA_LACUNARITY)
            .set_octaves(NEBULA_OCATAVES);

        let stars = Worley::new(seed + 1)
            .set_frequency(STAR_FREQUENCY)
            .set_return_type(ReturnType::Distance);

        let nebula = Add::new(nebula_base, stars);
        Cache::new(nebula)
    }
    let nebula_noise = nebula_noise();

    let noise_map = SphereMapBuilder::new(&nebula_noise)
        .set_latitude_bounds(-360.0, 360.0)
        .set_longitude_bounds(-180.0, 180.0)
        .set_size(size.width as usize, size.height as usize)
        .build();
    let nebula_gradient = ColorGradient::new()
        .clear_gradient()
        .add_gradient_point(-1.0000, [5, 0, 15, 255]) // Almost black, deep space
        .add_gradient_point(-0.9375, [18, 0, 30, 255]) // Very dark purple, the void of space
        .add_gradient_point(-0.7500, [28, 0, 60, 255]) // Dark purple, the depth of a nebula
        .add_gradient_point(-0.5000, [40, 0, 72, 255]) // Dark indigo, suggesting a dense nebula region
        .add_gradient_point(-0.2500, [48, 20, 80, 255]) // Dark purple with a hint of color, adding depth
        .add_gradient_point(0.0000, [60, 20, 92, 255]) // Slightly lighter purple, for internal nebula lighting
        .add_gradient_point(0.2500, [75, 0, 130, 255]) // Indigo, brighter regions of the nebula
        .add_gradient_point(0.5000, [0, 0, 0, 255]) // Black, to reintroduce the concept of vast, empty space
        .add_gradient_point(0.7500, [143, 0, 255, 255]) // Electric purple, rarefied areas of gas illumination
        .add_gradient_point(1.0000, [255, 255, 255, 255]); // Bright white, representing the brightest stars
    for y in 0..size.height {
        for x in 0..size.width {
            let value = noise_map.get_value(x as usize, y as usize);
            let index = (y * size.width + x) as usize * 4;
            let color = nebula_gradient.get_color(value as f64);
            image.data[index..index + 4].copy_from_slice(&color);
        }
    }
    image.reinterpret_stacked_2d_as_array(
        image.texture_descriptor.size.height / image.texture_descriptor.size.width,
    );
    image.texture_view_descriptor = Some(TextureViewDescriptor {
        dimension: Some(TextureViewDimension::Cube),
        ..default()
    });
    images.add(image)
}
