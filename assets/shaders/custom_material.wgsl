#import bevy_sprite::mesh2d_vertex_output::VertexOutput

// Custom materials in Bevy 2D typically bind to group 2.
@group(2) @binding(0)
var<uniform> color: vec4<f32>;

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    // This creates a simple gradient effect based on the mesh's UV coordinates,
    // multiplied by the color we pass in from Rust.
    return color * vec4<f32>(mesh.uv.x, mesh.uv.y, 1.0, 1.0);
}
