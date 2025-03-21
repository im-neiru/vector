[[ForceInline]]
float median(float3 s)
{
    return max(min(s.r, s.g), min(max(s.r, s.g), s.b));
}

float msdf_alpha(float3 msdf, float anti_alias_factor)
{
    let signed_distance = median(msdf);
    float smoothing_threshold = fwidth(signed_distance) * anti_alias_factor;

    return 1.0 - smoothstep(-smoothing_threshold, smoothing_threshold, signed_distance);
}
