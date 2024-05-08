#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput

@group(0) @binding(0) var screen_texture: texture_2d<f32>;
@group(0) @binding(1) var texture_sampler: sampler;
struct PostProcessSettings {
    intensity: f32,
#ifdef SIXTEEN_BYTE_ALIGNMENT
    // WebGL2 structs must be 16 byte aligned.
    _webgl2_padding: vec3<f32>
#endif
}
@group(0) @binding(2) var<uniform> settings: PostProcessSettings;

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    // Chromatic aberration strength and scanline effect
    let offset_strength = settings.intensity;
    let scanline_effect = sin(in.position.y * 3.0) * 1.0; // Intensity of scanlines

    // Revised barrel distortion parameters
    let barrel_distortion = settings.intensity; // Scaled by intensity for easier control
    let center = vec2<f32>(0.5, 0.5);
    let dist = length((in.uv - center) * vec2<f32>(1.6, 1.0)); // Aspect ratio correction
    let distorted_uv = center + (in.uv - center) * (1.0 + barrel_distortion * dist * dist); // Adjust distortion formula

    // Sample each color channel with an arbitrary shift
    let red_channel = textureSample(screen_texture, texture_sampler, distorted_uv + vec2<f32>(offset_strength, -offset_strength)).r;
    let green_channel = textureSample(screen_texture, texture_sampler, distorted_uv + vec2<f32>(-offset_strength, 0.0)).g;
    let blue_channel = textureSample(screen_texture, texture_sampler, distorted_uv + vec2<f32>(0.0, offset_strength)).b;

    // Apply scanline effect
    let color = vec4<f32>(red_channel, green_channel, blue_channel, 1.0);
    return color * (1.0 - scanline_effect);
}