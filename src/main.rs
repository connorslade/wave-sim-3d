use anyhow::Result;
use compute::{
    export::{
        wgpu::{include_wgsl, Limits, ShaderStages},
        winit::window::WindowAttributes,
    },
    gpu::Gpu,
};

use app::{App, Uniform};
use camera::Camera;
use marching_cubes::marching_cubes;
use simulation::{Config, Simulation};
mod app;
mod camera;
mod marching_cubes;
mod misc;
mod sci_dragger;
mod simulation;

fn main() -> Result<()> {
    let gpu = Gpu::builder()
        .with_limits(Limits {
            max_buffer_size: 256 << 21,
            ..Default::default()
        })
        .build()?;

    let config = Config::default();
    let mut simulation = Simulation {
        states: vec![vec![0.0; config.size.iter().product()]; 3],
        step: 0,
        config,
    };

    (0..50).for_each(|_| simulation.tick());
    let (vertices, indices) = simulation.triangluate(0.0);

    let index = gpu.create_index(&indices)?;
    let vertex = gpu.create_vertex(&vertices)?;
    let uniforms = gpu.create_uniform(&Uniform::default())?;
    let render = gpu
        .render_pipeline(include_wgsl!("render.wgsl"))
        .bind(&uniforms, ShaderStages::VERTEX_FRAGMENT)
        .finish();

    gpu.create_window(
        WindowAttributes::default().with_title("Wave Simulator 3D"),
        App {
            render,
            index,
            vertex,
            uniform: uniforms,

            indicies: indices.len() as u32,
            simulation,
            camera: Camera::default(),
            iso_level: 1e-4,
        },
    )
    .run()?;

    Ok(())
}
