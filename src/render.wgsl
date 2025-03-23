struct VertexOutput {
    @builtin(position) pos: vec4f,
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
    return VertexOutput(ctx.view_projection * pos);
}

@fragment
fn frag(in: VertexOutput) -> @location(0) vec4f {
    return vec4(1.0, 0.0, 0.0, 1.0);
}
