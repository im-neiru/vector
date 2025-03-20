[[vk::binding(0, 0)]]
StructuredBuffer<float> inputData;

[[vk::binding(1, 0)]]
RWStructuredBuffer<float> outputData ;

[[vk::binding(2, 0)]]
cbuffer Parameters
{
    float factor;
};

[shader("compute")]
[numthreads(256, 1, 1)]
void main(uint3 dispatchThreadID : SV_DispatchThreadID)
{
    uint idx = dispatchThreadID.x;
    outputData[idx] = inputData[idx] * factor;
}
