
struct Projection {
    scale: vec2<f32>,
    translate: vec2<f32>,
}

struct EmitQuadUv {
    transform: mat3x3<f32>,
    position: vec2<f32>,
    z: f32,
}

@group(0) @binding(0)
var<uniform> projection: Projection;
@group(0) @binding(1)
var<uniform> inputs: EmitQuadUv;

struct OutputVertex {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv: vec2<f32>,
}

@vertex
fn vs_main(
    @location(0) position: vec2<f32>
) -> OutputVertex {
    var local_position = vec3(position, inputs.z) * inputs.transform;
    local_position.z = local_position.z / 1024. + 0.5;

    let mapped = vec3((local_position.xy + inputs.position + projection.translate) * projection.scale, local_position.z);
    var output: OutputVertex;

    output.clip_position = vec4<f32>(mapped, 1.0);
    output.uv = vec2(position.x, -position.y);
    return output;
}
