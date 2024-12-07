#import bevy_sprite::{mesh2d_vertex_output::VertexOutput, mesh2d_view_bindings::globals}
const pi = radians(180.0);
const e = 2.71828;
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

fn handle_water(height: f32, angle: f32, time: f32) -> vec4<f32> {
    let w1 = generate_wave(angle, 50.0, 0.01, time, 0.5);
    let w2 = generate_wave(angle, 40.0, 0.005, time, -0.5);
    let w3 = generate_wave(angle, 20.0, 0.001, time, -0.5);
    let sum = w1 + w2 + w3;
    if height < 1. - sum {
        if height > 0.99 - sum {
            return vec4<f32>(1., 1.0, 1.0, 1.0);
        } else {
            return vec4<f32>(0., 0., 0., 1.);
        }
    }
    return vec4<f32>(0., 0.0, 0.0, 0.0);
}

fn normalized_sigmoid(x: f32) -> f32 {
    let expo = exp((-x + 0.5) * 10.);
    return 1. / (1 + expo);
}

fn sig(x: f32, alpha: f32) -> f32 {
    if (x > 1.) {
        return 1.;
    }

    let x_sqr = pow(x, alpha);
    return x_sqr / (x_sqr + pow(1 - x, alpha));
}

fn noise(x: f32, amplitude: f32) -> f32 {
    let s1 = sin(x);
    let s2 = sin(x * 0.6) * 1.2;
    let s3 = sin(x * 1.2) * 0.8;
    let s4 = sin(x * 3.) * 0.2;
    let s5 = sin(x * 0.2) * 5.;
    return (s1 + s2 + s3 + s4 + s5) * amplitude; 
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let centered_uv = in.uv * 2. - vec2<f32>(1., 1.);
    let angle = atan2(centered_uv.y, centered_uv.x);
    let time = globals.time / 3.; 
    let height = (length(centered_uv));
    var pos_deg = ((angle * 180) / pi) + 90;
    if pos_deg < 0 {
        pos_deg = pos_deg + 360.;
    }
    if (height >= 1.0) {
        return vec4<f32>(0.0);
    }
    
    for (var i: u32 = 0u; i < array_size; i = i + 1u) {
        let hole: vec2<f32> = get_hole(i);
        if (pos_deg > hole.x && pos_deg < hole.y) {
            let hole_size = distance(hole.x, hole.y);
            let depth_mult = hole_size;
            let dist_x = distance(pos_deg, hole.x);
            let dist_y = distance(pos_deg, hole.y);
            let shore_dist = min(dist_x, dist_y);
            let r = (shore_dist / (hole_size / 2));
            //let log = (log(r) + 1.) / 10.;
            let log = 0.;
            let random_variation = sig(shore_dist * 0.6, 2.) * noise(pos_deg, 0.02);
            let absolute_depth = sig(shore_dist / 5., 2.) + random_variation;
            let depth = absolute_depth * 0.1;
            if (height > 1. - depth) {
                return handle_water(height, angle, time);
            }
        }
    }
    return vec4<f32>(1.);
}