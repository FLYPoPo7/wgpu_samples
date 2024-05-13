var<private> v_positions: array<vec2<f32>, 3> = array<vec2<f32>, 3>(
    vec2<f32>(0.0, 0.5),
    vec2<f32>(-0.5, -0.5),
    vec2<f32>(0.5, -0.5)
);

@vertex
fn vs_main(
    @builtin(vertex_index) VertexIndex: u32
) -> @builtin(position) vec4f {
    let pos = array<vec2f, 3>(
        vec2f(0.0, 0.5),
        vec2f(-0.5, -0.5),
        vec2f(0.5, -0.5)
    );

    return vec4f(v_positions[VertexIndex], 0.0, 1.0);
}

@fragment
fn fs_main() -> @location(0) vec4f {
    return vec4(1.0, 0.0, 0.0, 1.0);
}
