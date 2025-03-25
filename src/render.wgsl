struct VertexInput {
    @location(0) pos: vec4f,
    @location(1) uv: vec2f
}

struct VertexOutput {
    @builtin(position) pos: vec4f,
    @location(0) uv: vec2f
};

struct Uniform {
    size: vec3<u32>,
    camera: Camera,

    ambiant: f32,
    intensity: f32,
    edge_falloff: f32
}


struct Camera {
    pos: vec3f,
    pitch: f32,
    yaw: f32,

    fov: f32,
    aspect: f32,
}

@group(0) @binding(0) var<uniform> ctx: Uniform;
@group(0) @binding(1) var<storage, read_write> state: array<f32>;

@vertex
fn vert(in: VertexInput) -> VertexOutput {
    return VertexOutput(in.pos, in.uv);
}

@fragment
fn frag(in: VertexOutput) -> @location(0) vec4f {
    let dir = ray_direction(in.uv);
    var pos = ctx.camera.pos;

    var accumulate = 0.0;
    var last = 0.0;
    for (var i = 0u; i < 1000; i++) {
        pos += dir * 0.1;

        let val = get_voxel_interp(pos);
        if abs(val) > 0.1 { accumulate += 0.01; }
    }

    return vec4(vec3(saturate(accumulate)), 1.0);
}

fn get_voxel_interp(pos: vec3f) -> f32 {
    if pos.x < 0.0 || pos.y < 0.0 || pos.z < 0.0 { return 0.0; }
    return get_voxel(vec3u(pos));
}

fn get_voxel(pos: vec3u) -> f32 {
    if pos.x >= ctx.size.x || pos.y >= ctx.size.y || pos.z >= ctx.size.z { return 0.0; }
    return state[pos.x * ctx.size.y * ctx.size.z + pos.y * ctx.size.z + pos.z];
}

fn ray_direction(pos: vec2f) -> vec3f {
    let forward = camera_direction();
    let right = -normalize(cross(vec3f(0, 1, 0), forward));
    let up = normalize(cross(forward, -right));

    let fov_scale = tan(ctx.camera.fov * 0.5);
    let uv = pos * vec2(ctx.camera.aspect, 1.0) * fov_scale;

    return normalize(forward + right * uv.x + up * uv.y);
}

fn camera_direction() -> vec3f {
    var pitch = ctx.camera.pitch;
    var yaw = ctx.camera.yaw;

    return normalize(vec3(
        cos(yaw) * cos(pitch),
        sin(pitch),
        sin(yaw) * cos(pitch)
    ));
}
