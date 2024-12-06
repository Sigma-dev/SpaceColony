#import bevy_sprite::{mesh2d_vertex_output::VertexOutput, mesh2d_view_bindings::globals}
#import noisy_bevy::simplex_noise_2d

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let p = in.uv; //+ vec2(globals.time, globals.time);
    let n = simplex_noise_2d(p * 1000);
    let alpha = f32(n > 0.95 + (sin(globals.time * 1) / 10000));
    if (alpha < 0.5) {
        return vec4<f32>(0.0, 0.0, 0.0, 1.0);
    }
    else {
        return vec4<f32>(1.0);
    }
}