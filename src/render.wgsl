struct VertexInput {
    @location(0) pos: vec4f,
    @location(1) normal: vec3f
}

struct VertexOutput {
    @builtin(position) pos: vec4f,
    @location(0) normal: vec3f,
    @location(1) world_position: vec3f
};

struct Uniform {
    view_projection: mat4x4f,
    camera_dir: vec3f,

    ambiant: f32,
    intensity: f32,
    edge_falloff: f32
}

@group(0) @binding(0) var<uniform> ctx: Uniform;

@vertex
fn vert(in: VertexInput) -> VertexOutput {
    return VertexOutput(
        ctx.view_projection * in.pos,
        in.normal,
        in.pos.xyz
    );
}

@fragment
fn frag(in: VertexOutput) -> @location(0) vec4f {
    let opacity = abs(dot(in.normal, ctx.camera_dir));
    return vec4(ctx.ambiant + ctx.intensity * (1.0 - pow(opacity, ctx.edge_falloff)));
}
