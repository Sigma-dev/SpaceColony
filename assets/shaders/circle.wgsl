#import bevy_sprite::{mesh2d_vertex_output::VertexOutput, mesh2d_view_bindings::globals}
const pi = radians(180.0);

struct CircleSettings {
    radius: f32,
    width: f32,
}

@group(2) @binding(0) var<uniform> properties: CircleSettings;

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let centered_uv = in.uv * 2. - vec2<f32>(1., 1.);
    let len = (length(centered_uv));
    let target_len = (properties.radius / 100.);
    let width = properties.width / 100.; // Divide to make it more manageable

    if distance(len, target_len) < width {
        return vec4<f32>(1.);
    }
    return vec4<f32>(0.0);
}