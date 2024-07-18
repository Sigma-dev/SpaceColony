#import bevy_sprite::{mesh2d_vertex_output::VertexOutput, mesh2d_view_bindings::globals}
const pi = radians(180.0);
const array_size: u32 = 3;

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let centered_uv = in.uv * 2. - vec2<f32>(1., 1.);
    let angle = atan2(centered_uv.y, centered_uv.x);
    let time = globals.time / 3.; 
    let len = (length(centered_uv));
    var hole_array: array<vec2<f32>, array_size> = array<vec2<f32>, array_size>(vec2<f32>(45., 90.), vec2<f32>(170., 190.), vec2<f32>(132., 160.));
    let angle_deg = (angle * 180/pi) + 90;
    for (var i: u32 = 0u; i < array_size; i = i + 1u) {
        let hole: vec2<f32> = hole_array[i];
        if (angle_deg > hole.x && angle_deg < hole.y) {
            if (len > 0.9) {
                return vec4<f32>(0.);
            }
        }
        else if (len >= 1.0) {
            return vec4<f32>(0.0);
        }
    }
    return vec4<f32>(1.);
}