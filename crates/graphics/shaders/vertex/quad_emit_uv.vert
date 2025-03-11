#version 450

layout(set = 0, binding = 0) uniform ProjectionBlock {
    vec2 scale;
    vec2 translate;
} projection;

layout(set = 0, binding = 1) uniform EmitQuadUvBlock {
    mat3 transform;
    vec2 position;
    float z;
} inputs;

layout(location = 0) in vec2 in_position;

layout(location = 0) out vec2 uv;

void main() {
    vec3 local_position = vec3(in_position, inputs.z) * inputs.transform;

    local_position.z = local_position.z / 1024.0 + 0.5;

    vec3 mapped = vec3((local_position.xy + inputs.position + projection.translate) * projection.scale, local_position.z);

    gl_Position = vec4(mapped, 1.0);

    uv = vec2(in_position.x, -in_position.y);
}
