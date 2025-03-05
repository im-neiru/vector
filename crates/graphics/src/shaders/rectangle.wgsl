struct VSOutput {
    @builtin(position) clipPosition: vec4<f32>,
    @location(0) fragCoord: vec2<f32>,
}

@vertex
fn vs_main(@location(0) position: vec2<f32>) -> VSOutput {
    var output: VSOutput;
    output.clipPosition = vec4<f32>(position, 0.0, 1.0);
    output.fragCoord = position;
    return output;
}
struct UFragment {
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

@group(0) @binding(0)
var<uniform> u_frag: UFragment;

@fragment
fn fs_main(@location(0) fragCoord: vec2<f32>) -> @location(0) vec4<f32> {

    let uv = fragCoord;
    let aa: f32 = 0.0025;
    var alpha: f32 = 1.0;

    if uv.x < u_frag.center_tl.x && uv.y > u_frag.center_tl.y {
        let d = distance(uv, u_frag.center_tl);
        alpha = 1.0 - smoothstep(u_frag.radius_tl, u_frag.radius_tl + aa, d);
        return vec4<f32>(u_frag.color.rgb, u_frag.color.a * alpha);
    } else if uv.x > u_frag.center_tr.x && uv.y > u_frag.center_tr.y {
        let d = distance(uv, u_frag.center_tr);
        alpha = 1.0 - smoothstep(u_frag.radius_tr, u_frag.radius_tr + aa, d);
        return vec4<f32>(u_frag.color.rgb, u_frag.color.a * alpha);
    } else if uv.x < u_frag.center_bl.x && uv.y < u_frag.center_bl.y {
        let d = distance(uv, u_frag.center_bl);
        alpha = 1.0 - smoothstep(u_frag.radius_bl, u_frag.radius_bl + aa, d);
        return vec4<f32>(u_frag.color.rgb, u_frag.color.a * alpha);
    } else if uv.x > u_frag.center_br.x && uv.y < u_frag.center_br.y {
        let d = distance(uv, u_frag.center_br);
        alpha = 1.0 - smoothstep(u_frag.radius_br, u_frag.radius_br + aa, d);
        return vec4<f32>(u_frag.color.rgb, u_frag.color.a * alpha);
    }

    return u_frag.color;
}
