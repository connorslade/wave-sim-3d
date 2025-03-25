struct VertexInput {
    @location(0) pos: vec4f,
    @location(1) uv: vec2f
}

struct VertexOutput {
    @builtin(position) pos: vec4f,
    @location(0) uv: vec2f
};

struct Uniform {
    size: vec3u,
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
    for (var i = 0u; i < 100; i++) {
        pos += dir * 0.1;
        // if (pos.x < 0.0 || pos.x >= f32(ctx.size.x) || pos.y < 0.0 || pos.y >= f32(ctx.size.y) || pos.z < 0.0 || pos.z >= f32(ctx.size.z)) {
        //     break;
        // }

        let val = state[u32(pos.x * f32(ctx.size.y * ctx.size.z) + pos.y * f32(ctx.size.z) + pos.z)];
        if val > 0.0 {
            return vec4(1.0);
        }

        // if sign(val) != sign(last) {
        //     accumulate += (1.0 / 100.0);
        // }

        // last = val;
    }

    return vec4(vec3(0.0), 1.0);
}

fn ray_direction(pos: vec2f) -> vec3f {
    let forward = camera_direction();
    let right = normalize(cross(vec3f(0, 1, 0), forward));
    let up = normalize(cross(forward, right));

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
