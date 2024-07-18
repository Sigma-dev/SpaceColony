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
                return vec4<f32>(0.);
            }
        }
    }
    return vec4<f32>(1.);
}