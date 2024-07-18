#import bevy_sprite::{mesh2d_vertex_output::VertexOutput, mesh2d_view_bindings::globals}
const pi = radians(180.0);

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let centered_uv = in.uv * 2. - vec2<f32>(1., 1.);
    let angle = atan2(centered_uv.y, centered_uv.x);
    let time = globals.time / 3.; 
    let len = (length(centered_uv));
    let hole = vec2<f32>(45., 90.);
    let angle_deg = (angle * 180/pi) + 90; //- (pi / 2.);
    if (angle_deg > hole.x && angle_deg < hole.y) {
        if (len < 0.9) {
            return vec4<f32>(1.0);
        }
    }
    else if (len <= 1.0) {
        return vec4<f32>(1.0);
    }
    return vec4<f32>(0.);
}