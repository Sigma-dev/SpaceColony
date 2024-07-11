#import bevy_ui::ui_vertex_output::UiVertexOutput

struct CustomUiMaterial {
    @location(0) progress: f32
}

@group(1) @binding(0) var<uniform> progress: f32;

@fragment
fn fragment(in: UiVertexOutput) -> @location(0) vec4<f32> {
    let uv = in.uv * 2.0 - 1.0;
    let alpha = f32(in.uv.x < progress);

    return vec4<f32>(vec3<f32>(1.0, 1.0, 1.0), alpha);
}