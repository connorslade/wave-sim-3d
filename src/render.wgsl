struct VertexOutput {
    @builtin(position) pos: vec4f,
    @location(0) world_position: vec3f
};

struct Uniform {
    view_projection: mat4x4f,
}

@group(0) @binding(0) var<uniform> ctx: Uniform;

@vertex
fn vert(
    @location(0) pos: vec4f,
    @location(1) uv: vec2f,
) -> VertexOutput {
    return VertexOutput(ctx.view_projection * pos, pos.xyz);
}

@fragment
fn frag(in: VertexOutput) -> @location(0) vec4f {
    let dy = dpdy(in.world_position);
    let dx = dpdx(in.world_position);
    let normal = normalize(abs(cross(dy, dx)));

    return vec4(normal, 1.0);
}
