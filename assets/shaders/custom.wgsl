[[block]]
struct InstanceData {
    position: vec3<f32>;
    block_type: u32;
};

[[group(1), binding(0)]]
var<storage> instances: [[stride(16)]] InstanceData;

[[stage(vertex)]]
fn vertex(
    [[location(0)]] position: vec3<f32>,
    [[location(1)]] normal: vec3<f32>,
    [[location(2)]] uv: vec2<f32>,
    [[instance_index]] instance_index: u32
) -> VertexOutput {
    let instance = instances[instance_index];
    var output: VertexOutput;
    output.position = vec4<f32>(position + instance.position, 1.0);
    output.normal = normal;
    output.uv = uv;
    return output;
}