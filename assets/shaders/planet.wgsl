#import bevy_sprite::{mesh2d_vertex_output::VertexOutput, mesh2d_view_bindings::globals}
const pi = radians(180.0);
const array_size: u32 = 8;

struct PlanetSettings {
    hole_array: array<vec4<f32>, array_size>
}

@group(2) @binding(0) var<uniform> properties: PlanetSettings;

fn pos_sin(x: f32) -> f32 {
    return (sin(x) + 1.) / 2.0;
}

fn generate_elevation(angle: f32, frequency: f32, amplitude: f32) -> f32 {
    return pos_sin(angle * frequency) * amplitude;
}

fn get_hole(index: u32) -> vec2<f32> {
    return properties.hole_array[index].xy;
}

fn generate_wave(angle: f32, frequency: f32, amplitude: f32, time: f32, time_mult: f32) -> f32 {
    let x = angle + (time * time_mult);
    return pos_sin(x * frequency) * amplitude;
}

fn handle_water(len: f32, angle: f32, time: f32) -> vec4<f32> {
    let w1 = generate_wave(angle, 50.0, 0.01, time, 0.5);
    let w2 = generate_wave(angle, 40.0, 0.005, time, -0.5);
    let w3 = generate_wave(angle, 20.0, 0.001, time, -0.5);
    let sum = w1 + w2 + w3;
    if len < 1. - sum {
        if len > 0.99 - sum {
            return vec4<f32>(1., 1.0, 1.0, 1.0);
        } else {
            return vec4<f32>(0.024, 0.025, 0.028, 1.);
        }
    }
    return vec4<f32>(0., 1.0, 1.0, 0.0);
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let centered_uv = in.uv * 2. - vec2<f32>(1., 1.);
    let angle = atan2(centered_uv.y, centered_uv.x);
    let time = globals.time / 3.; 
    let len = (length(centered_uv));
    //var hole_array: array<vec2<f32>, array_size> = array<vec2<f32>, array_size>(vec2<f32>(45., 90.), vec2<f32>(170., 190.), vec2<f32>(132., 160.));
    let angle_deg = (angle * 180/pi) + 90;

    if (len >= 1.0) {
        return vec4<f32>(0.0);
    }

    for (var i: u32 = 0u; i < array_size; i = i + 1u) {
        //let hole: vec2<f32> = properties.hole_array[i];
        let hole: vec2<f32> = get_hole(i);
        if (angle_deg > hole.x && angle_deg < hole.y) {
            let hole_size = distance(hole.x, hole.y);
            let depth_mult = hole_size;
            let dist_x = distance(angle_deg, hole.x);
            let dist_y = distance(angle_deg, hole.y);
            let shore_dist = min(dist_x, dist_y);
            let r = shore_dist / (hole_size / 2);
            let log = (log(r) + 1) / 10.;
            let elevation = generate_elevation(angle_deg, 1., 0.01);
            let depth = (log + elevation) / 2.;
            if (len > 0.97 - depth - (depth_mult / 250.)) {
                //return vec4<f32>(0.);
                return handle_water(len, angle, time);
            }
        }
    }
    return vec4<f32>(1.);
}