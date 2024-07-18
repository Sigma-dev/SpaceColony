#import bevy_sprite::{mesh2d_vertex_output::VertexOutput, mesh2d_view_bindings::globals}

fn pos_sin(x: f32) -> f32 {
    return (sin(x) + 1.) / 2.0;
}

fn generate_wave(angle: f32, frequency: f32, amplitude: f32, time: f32, time_mult: f32) -> f32 {
    let x = angle + (time * time_mult);
    return pos_sin(x * frequency) * amplitude;
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let centered_uv = in.uv * 2. - vec2<f32>(1., 1.);
    //let sinus = sin((in.uv.x + in.uv.y) * 100.) / 100.;
    let angle = atan2(centered_uv.y, centered_uv.x);
    let time = globals.time / 3.; 
    //let sin1 = pos_sin((angle + time) * 50. ) / 50.;
    //let sin2 = pos_sin((angle + (time / 2.)) * 25.) / 40.;
    //let sin3 = pos_sin((angle + (time / 10.)) * 10.) / 20.;
    //let sin4 = pos_sin((angle + (time * 10)) * 215) / 100.;
    //let sum = sin1 + sin2 + sin3;
    let w1 = generate_wave(angle, 50.0, 0.01, time, 0.5);
    let w2 = generate_wave(angle, 40.0, 0.005, time, -0.5);
    let w3 = generate_wave(angle, 20.0, 0.001, time, -0.5);
    let sum = w1 + w2 + w3;
    let len = (length(centered_uv) );
    if (len < 1. - sum) && (len > 0.99 - sum) {
        return vec4<f32>(1., 1.0, 1.0, 1.0);
    }
    return vec4<f32>(0., 1.0, 1.0, 0.0);
}