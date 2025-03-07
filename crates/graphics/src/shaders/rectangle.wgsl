struct TransformUniform {
    scale: vec2<f32>,
    translate: vec2<f32>,
}

struct FsUniform {
    color: vec4<f32>,    // Use this if you want to tint the shape.
    size: vec2<f32>,     // Size of the inner (non–padded) rounded rectangle.
    radius_tl: f32,      // Top–left corner radius.
    radius_tr: f32,      // Top–right corner radius.
    radius_bl: f32,      // Bottom–left corner radius.
    radius_br: f32,      // Bottom–right corner radius.
    padding: vec2<f32>,  // Extra padding.
}

struct VSOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv: vec2<f32>,
}

@group(0) @binding(0)
var<uniform> u_transform: TransformUniform;

@group(0) @binding(1)
var<uniform> fs_uniform: FsUniform;

@vertex
fn vs_main(
    @location(0) position: vec2<f32>, @location(1) uv: vec2<f32>
) -> VSOutput {

    let mapped_pos = (vec2(position.x, -position.y) + u_transform.translate) * u_transform.scale;
    var output: VSOutput;
    output.clip_position = vec4<f32>(mapped_pos, 0.0, 1.0);

    output.uv = uv;
    return output;
}


fn rounded_sdf(p: vec2<f32>, b: vec2<f32>, r: f32) -> f32 {
    let d = abs(p) - b + vec2<f32>(r);
    return min(max(d.x, d.y), 0.0) + length(max(d, vec2<f32>(0.0))) - r;
}

fn pseudo_msdf(uv: vec2<f32>) -> vec3<f32> {
    let halfSize = fs_uniform.size * 0.5;
    let p = uv;

    var r: f32 = 0.0;
    if p.x >= 0.0 && p.y >= 0.0 {
        r = fs_uniform.radius_tr;
    } else if p.x < 0.0 && p.y >= 0.0 {
        r = fs_uniform.radius_tl;
    } else if p.x < 0.0 && p.y < 0.0 {
        r = fs_uniform.radius_bl;
    } else {
        r = fs_uniform.radius_br;
    }

    let sdf = rounded_sdf(p, halfSize, r);

    let bias = 0.25 * fwidth(sdf);
    let sdfR = sdf + bias;
    let sdfG = sdf;
    let sdfB = sdf - bias;
    return vec3<f32>(sdfR, sdfG, sdfB);
}

fn median(r: f32, g: f32, b: f32) -> f32 {
    return max(min(r, g), min(max(r, g), b));
}

@fragment
fn fs_main(@location(0) uv: vec2<f32 >) -> @location(0) vec4<f32> {
    let aa = 0.6; // anti-aliasing factor

    let s = pseudo_msdf(uv);
    let sd = median(s.r, s.g, s.b);

    let afwidth = fwidth(sd) * aa;
    let alpha = 1.0 - smoothstep(-afwidth, afwidth, sd);

    return vec4(fs_uniform.color.rgb, fs_uniform.color.a * alpha);
}
