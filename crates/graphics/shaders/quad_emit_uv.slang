
[[vk::binding(0, 0)]]
cbuffer ProjectionBlock
{
    float2 scale;
    float2 translate;
};

[[vk::binding(1, 0)]]
cbuffer EmitQuadUvBlock
{
    float3x3 transform;
    float2 position;
    float z;
};

struct VsInput
{
    [[vk::location(0)]] float2 in_position : POSITION;
};

struct VsOutput
{
    [[vk::location(0)]] float4 position : SV_POSITION;
    [[vk::location(1)]] float2 uv : TEXCOORD0;
};

[shader("vertex")]
VsOutput vs_main(VsInput input)
{
    VsOutput output;

    float3 local_position = mul(float3(input.in_position, z), transform);


    local_position.z = local_position.z / 1024.0f + 0.5f;

    float3 mapped = float3((local_position.xy + position + translate) * scale, local_position.z);

    output.position = float4(mapped, 1.0f);

    output.uv = float2(input.in_position.x, -input.in_position.y);

    return output;
}
