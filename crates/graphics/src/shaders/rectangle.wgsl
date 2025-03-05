
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
    let aa: f32 = 1.9;
    var alpha: f32 = 1.0;
    let baseColor = fs_uniform.color;


    let c_left = fs_uniform.center_tl.y > fs_uniform.center_bl.y;
    let c_right = fs_uniform.center_tr.y > fs_uniform.center_br.y;
    let c_top = fs_uniform.center_tl.x > fs_uniform.center_tr.x;
    let c_bottom = fs_uniform.center_bl.x > fs_uniform.center_br.x;


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

    if c_left && c_right && c_top && c_bottom {
        let center = (fs_uniform.center_tl + fs_uniform.center_tr + fs_uniform.center_bl + fs_uniform.center_br) * 0.25;
        let avgRadius = (fs_uniform.radius_tl + fs_uniform.radius_tr + fs_uniform.radius_bl + fs_uniform.radius_br) * 0.25;
        let d = distance(uv, center);
        alpha = 1.0 - smoothstep(avgRadius, avgRadius + aa, d);
    } else {
        if c_left && uv.x < ((fs_uniform.center_tl.x + fs_uniform.center_bl.x) * 0.5) {
            let leftCenter = (fs_uniform.center_tl + fs_uniform.center_bl) * 0.5;
            let leftRadius = (fs_uniform.radius_tl + fs_uniform.radius_bl) * 0.5;
            let d = distance(uv, leftCenter);
            alpha = min(alpha, 1.0 - smoothstep(leftRadius, leftRadius + aa, d));
        }

        if c_right && uv.x > ((fs_uniform.center_tr.x + fs_uniform.center_br.x) * 0.5) {
            let rightCenter = (fs_uniform.center_tr + fs_uniform.center_br) * 0.5;
            let rightRadius = (fs_uniform.radius_tr + fs_uniform.radius_br) * 0.5;
            let d = distance(uv, rightCenter);
            alpha = min(alpha, 1.0 - smoothstep(rightRadius, rightRadius + aa, d));
        }

        if c_top && uv.y < ((fs_uniform.center_tl.y + fs_uniform.center_tr.y) * 0.5) {
            let topCenter = (fs_uniform.center_tl + fs_uniform.center_tr) * 0.5;
            let topRadius = (fs_uniform.radius_tl + fs_uniform.radius_tr) * 0.5;
            let d = distance(uv, topCenter);
            alpha = min(alpha, 1.0 - smoothstep(topRadius, topRadius + aa, d));
        }

        if c_bottom && uv.y > ((fs_uniform.center_bl.y + fs_uniform.center_br.y) * 0.5) {
            let bottomCenter = (fs_uniform.center_bl + fs_uniform.center_br) * 0.5;
            let bottomRadius = (fs_uniform.radius_bl + fs_uniform.radius_br) * 0.5;
            let d = distance(uv, bottomCenter);
            alpha = min(alpha, 1.0 - smoothstep(bottomRadius, bottomRadius + aa, d));
        }
    }

    return vec4<f32>(baseColor.rgb, baseColor.a * alpha);
}
