struct VertexWithUv
{
    [[vk::location(0)]] float4 position : SV_POSITION;
    [[vk::location(1)]] float2 uv : TEXCOORD0;
};
