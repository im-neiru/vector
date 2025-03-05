
struct TranformUniform {
    scale: vec2<f32>,
    translate: vec2<f32>,
};

struct FsUniform {
    color: vec4<f32>,
    center_tl: vec2<f32>,
    center_tr: vec2<f32>,
    center_bl: vec2<f32>,
    center_br: vec2<f32>,
    radius_tl: f32,
    radius_tr: f32,
    radius_bl: f32,
    radius_br: f32,
};

struct VSOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv: vec2<f32>,
};


@group(0) @binding(0)
var<uniform> u_transform: TranformUniform;


@group(0) @binding(1)
var<uniform> fs_uniform: FsUniform;

@vertex
fn vs_main(@location(0) position: vec2<f32>,
    @location(1) uv: vec2<f32>) -> VSOutput {
    let mapped_pos = (vec2(position.x, -position.y) + u_transform.translate) * u_transform.scale;
    var output: VSOutput;
    output.clip_position = vec4<f32>(mapped_pos, 0.0, 1.0);

    output.uv = uv;
    return output;
}

@fragment
fn fs_main(@location(0) uv: vec2<f32>) -> @location(0) vec4<f32> {
    let aa: f32 = 3.0;
    var alpha: f32 = 1.0;
    let baseColor = fs_uniform.color;

    if uv.x < fs_uniform.center_tl.x && uv.y < fs_uniform.center_tl.y {
        let d = distance(uv, fs_uniform.center_tl);
        alpha = 1.0 - smoothstep(fs_uniform.radius_tl, fs_uniform.radius_tl + aa, d);
    } else if uv.x > fs_uniform.center_tr.x && uv.y < fs_uniform.center_tr.y {
        let d = distance(uv, fs_uniform.center_tr);
        alpha = 1.0 - smoothstep(fs_uniform.radius_tr, fs_uniform.radius_tr + aa, d);
    } else if uv.x < fs_uniform.center_bl.x && uv.y > fs_uniform.center_bl.y {
        let d = distance(uv, fs_uniform.center_bl);
        alpha = 1.0 - smoothstep(fs_uniform.radius_bl, fs_uniform.radius_bl + aa, d);
    } else if uv.x > fs_uniform.center_br.x && uv.y > fs_uniform.center_br.y {
        let d = distance(uv, fs_uniform.center_br);
        alpha = 1.0 - smoothstep(fs_uniform.radius_br, fs_uniform.radius_br + aa, d);
    }

    return vec4<f32>(baseColor.rgb, baseColor.a * alpha);
}
