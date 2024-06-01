struct VertexInput {
    @builtin(instance_index) instance_index: u32,
    @location(0) position: vec4f,
    @location(1) color: vec4f,
    @location(2) uv: vec2f,
}

struct VertexOutput {
    @builtin(position) position: vec4f,
    @location(0) uv: vec2f,
    @location(1) frag_position: vec4f,
}

struct Uniforms {
    mvpMatrixArray: array<mat4x4f, 16>,
}

@binding(0) @group(0) var<uniform> uniforms: Uniforms;
@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var output: VertexOutput;
    output.position = uniforms.mvpMatrixArray[model.instance_index] * model.position;
    output.uv = model.uv;
    output.frag_position = 0.5 * (model.position + vec4(1.0, 1.0, 1.0, 1.0));
    return output;
}

@fragment
fn fs_main(
    in: VertexOutput
) -> @location(0) vec4f {
    return in.frag_position;
}
