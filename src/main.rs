use anyhow::Result;
use compute::{
    export::{
        wgpu::{include_wgsl, CompareFunction, Limits, ShaderStages},
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
            ..Default::default()
        })
        .build()?;

    let config = Config::default();
    let cells = config.size.iter().product();
    let mut simulation = Simulation {
        states: vec![vec![0.0; cells]; 3],
        energy: vec![0.0; cells],
        step: 0,
        config,
    };

    (0..100).for_each(|_| simulation.tick());
    let state = gpu.create_storage(&simulation.states[simulation.step % 3])?;

    let uniforms = gpu.create_uniform(&Uniform::default())?;
    let render = gpu
        .render_pipeline(include_wgsl!("render.wgsl"))
        .depth_compare(CompareFunction::Always)
        .bind(&uniforms, ShaderStages::VERTEX_FRAGMENT)
        .bind(&state, ShaderStages::FRAGMENT)
        .finish();

    gpu.create_window(
        WindowAttributes::default().with_title("Wave Simulator 3D"),
        App {
            render,
            state,
            uniform: uniforms,

            simulation,
            camera: Camera::default(),
            render_config: RenderConfig::default(),
        },
    )
    .run()?;

    Ok(())
}
