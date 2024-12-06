#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput

@group(0) @binding(0) var screen_texture: texture_2d<f32>;
@group(0) @binding(1) var texture_sampler: sampler;
struct PostProcessSettings {
    white_color: vec3<f32>,
    black_color: vec3<f32>
#ifdef SIXTEEN_BYTE_ALIGNMENT
    // WebGL2 structs must be 16 byte aligned.
    _webgl2_padding: vec3<f32>
#endif
}
@group(0) @binding(2) var<uniform> settings: PostProcessSettings;

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    let sampled = textureSample(screen_texture, texture_sampler, in.uv);
    let white = settings.white_color;
    let black = settings.black_color;
    let whiteness = (sampled.r + sampled.g + sampled.b) / 3.;
    /* 
    if (sampled.r == 1.0 && sampled.g == 1.0 && sampled.b == 1.0) {
        return vec4<f32>(white.r, white.g, white.b, sampled.a);
    }
    else {
        return vec4<f32>(black.r, black.g, black.b, sampled.a);
    }
    if (sampled.r == 0.0 && sampled.g == 0.0 && sampled.b == 0.0) {
        return vec4<f32>(black.r, black.g, black.b, sampled.a);
    }
    else {
        return vec4<f32>(white.r, white.g, white.b, sampled.a);
    }
    */
    //return vec4<f32>(white.r, white.g, white.b, sampled.a);
    let v3 = mix(black, white, whiteness);
    return vec4<f32>(v3.r, v3.g, v3.b, 1.0);
}