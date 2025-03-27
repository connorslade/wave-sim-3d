@group(0) @binding(0) var<uniform> ctx: Context;
@group(0) @binding(1) var<storage, read_write> map: array<f32>;

struct Context {
    size: vec3u,
    v: f32,
    dx: f32,
    dt: f32,

    step: u32,
}

fn index(pos: vec3u, n: u32) -> u32 {
    return n * ctx.size.x * ctx.size.y * ctx.size.z + pos.x * ctx.size.y * ctx.size.z + pos.y * ctx.size.z + pos.z;
}

@compute
@workgroup_size(8, 8, 8)
fn main(@builtin(global_invocation_id) pos: vec3u) {
    let prev = (ctx.step + 2) % 3;
    let curr = ctx.step % 3;
    let next = (ctx.step + 1) % 3;

    // todo make consts
    let x = vec3u(1, 0, 0);
    let y = vec3u(0, 1, 0);
    let z = vec3u(0, 0, 1);

    // move out of shader
    let c = pow(ctx.v, 2.0) * (ctx.dt / pow(ctx.dx, 3.0));

    let dx = map[index(pos + x, curr)] + map[index(pos - x, curr)];
    let dy = map[index(pos + y, curr)] + map[index(pos - y, curr)];
    let dz = map[index(pos + z, curr)] + map[index(pos - z, curr)];
    let ds = dx + dy + dz - 6.0 * map[index(pos, curr)];
    var u = ds * c - map[index(pos, prev)] + 2.0 * map[index(pos, curr)];

    {
        let center_dist = length(vec3f(f32(ctx.size.x) / 2.0 + 15.0, f32(ctx.size.y) / 2.0, f32(ctx.size.z) / 2.0) - vec3f(pos));
        u += exp(-center_dist) * cos(f32(ctx.step) * 0.1);
    }

    {
        let center_dist = length(vec3f(f32(ctx.size.x) / 2.0 - 15.0, f32(ctx.size.y) / 2.0, f32(ctx.size.z) / 2.0) - vec3f(pos));
        u += exp(-center_dist) * cos(f32(ctx.step) * 0.1);
    }

    map[index(pos, next)] = u;
}
