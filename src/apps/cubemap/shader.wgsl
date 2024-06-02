struct VertexInput {
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
    modelViewProjection: mat4x4f,
}

@group(0) @binding(0) var<uniform> uniforms: Uniforms;
@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var output: VertexOutput;
    output.position = uniforms.modelViewProjection * model.position;
    output.uv = model.uv;
    output.frag_position = 0.5 * (model.position + vec4(1.0, 1.0, 1.0, 1.0));
    return output;
}

@group(1) @binding(0) var cubemap_texture: texture_cube<f32>;
@group(1) @binding(1) var cubemap_sampler: sampler;
@fragment
fn fs_main(
    in: VertexOutput
) -> @location(0) vec4f {
  // Our camera and the skybox cube are both centered at (0, 0, 0)
  // so we can use the cube geomtry position to get viewing vector to sample the cube texture.
  // The magnitude of the vector doesn't matter.
  var cubemap_vec = in.frag_position.xyz - vec3(0.5);
  return textureSample(cubemap_texture, cubemap_sampler, cubemap_vec);
}
