struct VertexOutput {
    @builtin(position) pos: vec4f,
    @location(0) world_position: vec3f
};

struct Uniform {
    view_projection: mat4x4f,
    camera_dir: vec3f
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

    // let diffuse = max(dot(normal, ctx.camera_dir), 0.0);
    // let reflect_dir = reflect(-ctx.camera_dir, normal);
    // let specular = pow(max(dot(ctx.camera_dir, reflect_dir), 0.0), 32.0);

    // let intensity = (diffuse + specular + 0.1);
    // return vec4(vec3(intensity), 1.0);
}
