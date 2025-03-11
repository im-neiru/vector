#version 450

layout(set = 1, binding = 0) uniform RoundedRectangleColorFillBlock {
    vec4 color;
    vec2 size;
    float radius_tl;
    float radius_tr;
    float radius_bl;
    float radius_br;
    vec2 padding;
} inputs;

layout(location = 0) in vec2 uv;

layout(location = 0) out vec4 outColor;

float rounded_sdf(vec2 p, vec2 b, float r) {
    vec2 d = abs(p) - b + vec2(r);
    return min(max(d.x, d.y), 0.0) + length(max(d, vec2(0.0))) - r;
}

vec3 pseudo_msdf(vec2 uv) {
    vec2 half_size = inputs.size * 0.5;
    float r = 0.0;
    if (uv.x >= 0.0 && uv.y >= 0.0) {
        r = inputs.radius_tr;
    } else if (uv.x < 0.0 && uv.y >= 0.0) {
        r = inputs.radius_tl;
    } else if (uv.x < 0.0 && uv.y < 0.0) {
        r = inputs.radius_bl;
    } else {
        r = inputs.radius_br;
    }

    float sdf = rounded_sdf(uv, half_size, r);
    float bias = fwidth(sdf);
    float sdf_r = sdf + bias;
    float sdf_g = sdf;
    float sdf_b = sdf - bias;
    return vec3(sdf_r, sdf_g, sdf_b);
}

float median(vec3 s) {
    return max(min(s.r, s.g), min(max(s.r, s.g), s.b));
}

void main() {
    float anti_alias_factor = 0.6;
    vec3 msdf_values = pseudo_msdf(uv);
    float signed_distance = median(msdf_values);
    float smoothing_threshold = fwidth(signed_distance) * anti_alias_factor;
    float alpha = 1.0 - smoothstep(-smoothing_threshold, smoothing_threshold, signed_distance);
    outColor = vec4(inputs.color.rgb, inputs.color.a * alpha);
}
