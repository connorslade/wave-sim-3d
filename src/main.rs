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
use marching_cubes::marching_cubes;
use simulation::{Config, Simulation};
use vertex::VERTEX_BUFFER_LAYOUT;
mod app;
mod camera;
mod marching_cubes;
mod simulation;
mod ui;
mod vertex;

fn main() -> Result<()> {
    let gpu = Gpu::builder()
        .with_limits(Limits {
            max_buffer_size: 2147483647,
            ..Default::default()
        })
        .build()?;

    let config = Config::default();
    let simulation = Simulation {
        states: vec![vec![0.0; config.size.iter().product()]; 3],
        step: 0,
        config,
    };

    let index = gpu.create_index_empty(1_000_000);
    let vertex = gpu.create_vertex_empty(1_000_000)?;
    let uniforms = gpu.create_uniform(&Uniform::default())?;
    let render = gpu
        .render_pipeline(include_wgsl!("render.wgsl"))
        .vertex_layout(VERTEX_BUFFER_LAYOUT)
        .depth_compare(CompareFunction::Always)
        .bind(&uniforms, ShaderStages::VERTEX_FRAGMENT)
        .finish();

    gpu.create_window(
        WindowAttributes::default().with_title("Wave Simulator 3D"),
        App {
            render,
            index,
            vertex,
            uniform: uniforms,

            indicies: 0,
            simulation,
            camera: Camera::default(),
            iso_level: 0.4,
            render_config: RenderConfig::default(),

            scheduled_remesh: false,
            use_iso_level: true,
        },
    )
    .run()?;

    Ok(())
}
