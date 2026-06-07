// Import Bevy's built-in vertex output, which handles the screen UV coordinates
#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput

// Bevy automatically binds the existing camera render output here
@group(0) @binding(0) var screen_texture: texture_2d<f32>;
@group(0) @binding(1) var texture_sampler: sampler;

// This matches our `FullscreenEffect` struct in Rust
struct FullScreenEffect {
    intensity: f32,
#ifdef SIXTEEN_BYTE_ALIGNMENT
    // Required padding if compiling for WebGL2 environments
    _webgl2_padding: vec3<f32>
#endif
}

@group(0) @binding(2) var<uniform> settings: FullScreenEffect;

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    // Sample the original pixels from the game world (if you rendered any sprites)
    let base_color = textureSample(screen_texture, texture_sampler, in.uv);
    
    // Generate a simple gradient based on the screen UVs
    let r = in.uv.x;
    let g = in.uv.y;
    let b = settings.intensity;
    let my_custom_color = vec4<f32>(r, g, b, 1.0);
    
    // Blend our custom gradient over the original screen texture 
    // based on the intensity value we passed from Rust
    return mix(base_color, my_custom_color, settings.intensity);
}
