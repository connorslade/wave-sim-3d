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
    let camera_dir = camera_direction();
    let dir = ray_direction(camera_dir, in.uv) * 0.5;
    var pos = ctx.camera.pos;

    var accumulate = 0.0;
    var last = 0.0;
    let seen = false;
    for (var i = 0u; i < 100; i++) {
        pos += dir;

        let val = get_voxel_smooth(pos);
        if val.normal.x == 0.0 {
            if seen { break; }
            else { continue; }
        }

        if (last < 0.1) != (val.value < 0.1) {
            let opacity = abs(dot(val.normal, camera_dir));
            accumulate += ctx.ambiant + ctx.intensity * (1.0 - pow(opacity, ctx.edge_falloff));
        }
        last = val.value;
    }

    return vec4(vec3(saturate(accumulate)), 1.0);
}

struct VoxelLookupResult {
    value: f32,
    normal: vec3f
}

fn get_voxel_smooth(pos: vec3f) -> VoxelLookupResult {
    if pos.x < 0.0 || pos.y < 0.0 || pos.z < 0.0 {
        return VoxelLookupResult(0.0, vec3f(0, 0, 0));
    }

    let p0 = vec3u(floor(pos));
    let p1 = p0 + vec3u(1, 1, 1);

    let d000 = get_voxel(p0);
    let d100 = get_voxel(vec3u(p1.x, p0.y, p0.z));
    let d010 = get_voxel(vec3u(p0.x, p1.y, p0.z));
    let d110 = get_voxel(vec3u(p1.x, p1.y, p0.z));
    let d001 = get_voxel(vec3u(p0.x, p0.y, p1.z));
    let d101 = get_voxel(vec3u(p1.x, p0.y, p1.z));
    let d011 = get_voxel(vec3u(p0.x, p1.y, p1.z));
    let d111 = get_voxel(p1);

    let frac = pos - vec3f(p0);

    let d00 = mix(d000, d100, frac.x);
    let d10 = mix(d010, d110, frac.x);
    let d01 = mix(d001, d101, frac.x);
    let d11 = mix(d011, d111, frac.x);

    let d0 = mix(d00, d10, frac.y);
    let d1 = mix(d01, d11, frac.y);

    let value = mix(d0, d1, frac.z);

    let dx = d100 - d000;
    let dy = d010 - d000;
    let dz = d001 - d000;
    let normal = normalize(vec3f(dx, dy, dz));

    return VoxelLookupResult(value, normal);
}

fn get_voxel(pos: vec3u) -> f32 {
    if pos.x >= ctx.size.x || pos.y >= ctx.size.y || pos.z >= ctx.size.z { return 0.0; }
    return state[pos.x * ctx.size.y * ctx.size.z + pos.y * ctx.size.z + pos.z];
}

fn ray_direction(forward: vec3f, pos: vec2f) -> vec3f {
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
