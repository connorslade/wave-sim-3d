use anyhow::Result;
use compute::{
    export::{
        wgpu::{include_wgsl, Limits, ShaderStages},
        winit::window::WindowAttributes,
    },
    gpu::Gpu,
};

use app::{App, RenderConfig, Uniform};
use camera::Camera;
use simulation::{Config, Simulation};
mod app;
mod camera;
mod simulation;
mod ui;

fn main() -> Result<()> {
    let gpu = Gpu::builder()
        .with_limits(Limits {
            max_buffer_size: 2147483647,
            max_compute_invocations_per_workgroup: 512,
            ..Default::default()
        })
        .build()?;

    let config = Config::default();

    let mut simulation = Simulation::new(&gpu, config)?;
    (0..100).for_each(|_| simulation.tick());

    let uniforms = gpu.create_uniform(&Uniform::default())?;
    let render = gpu
        .render_pipeline(include_wgsl!("shaders/render.wgsl"))
        .bind(&uniforms, ShaderStages::VERTEX_FRAGMENT)
        .bind(&simulation.states, ShaderStages::FRAGMENT)
        .finish();

    gpu.create_window(
        WindowAttributes::default().with_title("Wave Simulator 3D"),
        App {
            render,
            uniform: uniforms,

            simulation,
            camera: Camera::default(),
            render_config: RenderConfig::default(),
        },
    )
    .run()?;

    Ok(())
}
