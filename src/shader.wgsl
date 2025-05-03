struct Uniforms {
    model: mat4x4<f32>,
    view: mat4x4<f32>,
    projection: mat4x4<f32>,
}

@group(0) @binding(0) var<uniform> uniforms: Uniforms;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
}

struct FragmentInput {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec4<f32>,
}

@vertex
fn vertex(in: VertexInput) -> FragmentInput {
    var out: FragmentInput;
    out.position = uniforms.projection * uniforms.view * uniforms.model * vec4<f32>(in.position, 1.0);
    out.color = vec4<f32>(in.color, 1.0);
    return out;
}

@fragment
fn fragment(in: FragmentInput) -> @location(0) vec4<f32> {
    return in.color;
}

/// Generates vertices from the vertex index.
/// - [-1, -1,  0,  1]
/// - [ 0,  1,  0,  1]
/// - [ 1, -1,  0,  1]
// @vertex
// fn vertex_from_index(@builtin(vertex_index) in_vertex_index: u32) -> @builtin(position) vec4<f32> {
//     let x = f32(i32(in_vertex_index) - 1);
//     let y = f32(i32(in_vertex_index & 1u) * 2 - 1);
//     return vec4<f32>(x, y, 0.0, 1.0);
// }
