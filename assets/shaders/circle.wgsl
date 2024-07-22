#import bevy_sprite::{mesh2d_vertex_output::VertexOutput, mesh2d_view_bindings::globals}
const pi = radians(180.0);
const array_size: u32 = 8;

struct CircleSettings {
    size: f32
}

@group(2) @binding(0) var<uniform> properties: CircleSettings;

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let centered_uv = in.uv * 2. - vec2<f32>(1., 1.);
    let angle = atan2(centered_uv.y, centered_uv.x);
    let time = globals.time / 3.; 
    let len = (length(centered_uv));
    var angle_deg = ((angle * 180) / pi) + 90;
    if angle_deg < 0 {
        angle_deg = angle_deg + 360.;
    }
    let target_len = (properties.size / 100.);
    if (len >= target_len || len <= target_len - 0.01) {
        return vec4<f32>(0.0);
    }
    return vec4<f32>(1.);
}