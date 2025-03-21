#include "common/vertex_with_uv.slang"

struct RoundedRectangleColorFill
{
    float4 color;
    float2 size;
    float radius_tl;
    float radius_tr;
    float radius_bl;
    float radius_br;
    float2 padding;
};

[vk::binding(0, 1)]
ParameterBlock<RoundedRectangleColorFill> inputs;

float rounded_sdf(float2 p, float2 b, float r)
{
    float2 d = abs(p) - b + float2(r);
    return min(max(d.x, d.y), 0.0) + length(max(d, float2(0.0))) - r;
}

float3 pseudo_msdf(float2 uv)
{
    float2 half_size = inputs.size * 0.5;
    float r = 0.0;
    if (uv.x >= 0.0 && uv.y >= 0.0)
    {
        r = inputs.radius_tr;
    }
    else if (uv.x < 0.0 && uv.y >= 0.0)
    {
        r = inputs.radius_tl;
    }
    else if (uv.x < 0.0 && uv.y < 0.0)
    {
        r = inputs.radius_bl;
    }
    else
    {
        r = inputs.radius_br;
    }

    float sdf = rounded_sdf(uv, half_size, r);
    float bias = fwidth(sdf);
    float sdf_r = sdf + bias;
    float sdf_g = sdf;
    float sdf_b = sdf - bias;
    return float3(sdf_r, sdf_g, sdf_b);
}

float median(float3 s)
{
    return max(min(s.r, s.g), min(max(s.r, s.g), s.b));
}

[shader("fragment")]
float4 rounded_rectangle_color_fill(VertexWithUv input) : SV_Target0
{
    float anti_alias_factor = 0.6;
    float3 msdf_values = pseudo_msdf(input.uv);
    float signed_distance = median(msdf_values);
    float smoothing_threshold = fwidth(signed_distance) * anti_alias_factor;
    float alpha = 1.0 - smoothstep(-smoothing_threshold, smoothing_threshold, signed_distance);

    return float4(inputs.color.rgb, inputs.color.a * alpha);
}
