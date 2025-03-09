
struct RoundedRectangleColorFill {
    color: vec4<f32>,    // Use this if you want to tint the shape.
    size: vec2<f32>,     // Size of the inner (non–padded) rounded rectangle.
    radius_tl: f32,      // Top–left corner radius.
    radius_tr: f32,      // Top–right corner radius.
    radius_bl: f32,      // Bottom–left corner radius.
    radius_br: f32,      // Bottom–right corner radius.
    padding: vec2<f32>,  // Extra padding.
}


@group(1) @binding(0)
var<uniform> inputs: RoundedRectangleColorFill;

fn rounded_sdf(p: vec2<f32>, b: vec2<f32>, r: f32) -> f32 {
    let d = abs(p) - b + vec2<f32>(r);
    return min(max(d.x, d.y), 0.0) + length(max(d, vec2<f32>(0.0))) - r;
}

fn pseudo_msdf(uv: vec2<f32>) -> vec3<f32> {
    let half_size = inputs.size * 0.5;

    var r: f32 = 0.0;
    if uv.x >= 0.0 && uv.y >= 0.0 {
        r = inputs.radius_tr;
    } else if uv.x < 0.0 && uv.y >= 0.0 {
        r = inputs.radius_tl;
    } else if uv.x < 0.0 && uv.y < 0.0 {
        r = inputs.radius_bl;
    } else {
        r = inputs.radius_br;
    }

    let sdf = rounded_sdf(uv, half_size, r);
    let bias = fwidth(sdf);
    let sdf_r = sdf + bias;
    let sdf_g = sdf;
    let sdf_b = sdf - bias;
    return vec3<f32>(sdf_r, sdf_g, sdf_b);
}

fn median(s: vec3<f32>) -> f32 {
    return max(min(s.r, s.g), min(max(s.r, s.g), s.b));
}

@fragment
fn fs_main(@location(0) uv: vec2<f32>) -> @location(0) vec4<f32> {
    let anti_alias_factor = 0.6;

    let msdf_values = pseudo_msdf(uv);


    let signed_distance = median(msdf_values);

    let smoothing_threshold = fwidth(signed_distance) * anti_alias_factor;
    let alpha = 1.0 - smoothstep(-smoothing_threshold, smoothing_threshold, signed_distance);
    return vec4(inputs.color.rgb, inputs.color.a * alpha);
}
