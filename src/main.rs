use std::f32::consts::FRAC_PI_2;

use anyhow::Result;
use compute::{
    bindings::{IndexBuffer, UniformBuffer, VertexBuffer},
    export::{
        egui::{Context, Window},
        nalgebra::{Matrix4, Point3, Vector3},
        wgpu::{include_wgsl, RenderPass, ShaderStages},
        winit::window::WindowAttributes,
    },
    gpu::Gpu,
    interactive::{GraphicsCtx, Interactive},
    pipeline::render::{RenderPipeline, Vertex},
};
use encase::ShaderType;
use marching_cubes::marching_cubes;
use misc::{dragger, vec3_dragger};
use simulation::{Config, Simulation};

mod marching_cubes;
mod misc;
mod simulation;

struct App {
    render: RenderPipeline,
    index: IndexBuffer,
    vertex: VertexBuffer<Vertex>,
    uniform: UniformBuffer<Uniform>,

    simulation: Simulation,
    config: Config,
    camera: Camera,
    indicies: u32,
}

#[derive(ShaderType, Default)]
struct Uniform {
    view_projection: Matrix4<f32>,
}

struct Camera {
    position: Vector3<f32>,
    target: Vector3<f32>,
    fov: f32,
    near: f32,
    far: f32,
}

impl Default for Camera {
    fn default() -> Self {
        Camera {
            position: Vector3::new(0.0, 0.0, 1.0),
            target: Vector3::new(0.0, 0.0, 0.0),
            fov: FRAC_PI_2,
            near: 0.1,
            far: 10_000.0,
        }
    }
}

fn main() -> Result<()> {
    let gpu = Gpu::new()?;

    let config = Config::default();
    let mut simulation = Simulation {
        states: vec![vec![0.0; config.size.iter().product()]; 3],
        step: 0,
    };

    let size = config.size;
    let inner_size = 20;
    let offset = (size[0] - inner_size) / 2;

    for x in offset..offset + inner_size {
        for y in offset..offset + inner_size {
            for z in offset..offset + inner_size {
                simulation.states[0][x * size[1] * size[2] + y * size[2] + z] = 1.0;
            }
        }
    }

    let (vertices, indices) = marching_cubes(&simulation.states[simulation.step], config.size, 0.5);

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

            simulation,
            config,
            camera: Camera::default(),
            indicies: indices.len() as u32,
        },
    )
    .run()?;

    Ok(())
}

impl Interactive for App {
    fn ui(&mut self, _gcx: GraphicsCtx, ctx: &Context) {
        Window::new("Wave Simulator 3D").show(ctx, |ui| {
            ui.collapsing("Camera", |ui| {
                ui.horizontal(|ui| {
                    ui.label("Position");
                    vec3_dragger(ui, &mut self.camera.position, |x| x.speed(0.1));
                });
                ui.horizontal(|ui| {
                    ui.label("Target");
                    vec3_dragger(ui, &mut self.camera.target, |x| x.speed(0.1));
                });
                dragger(ui, "Fov", &mut self.camera.fov, |x| x.speed(0.1));
                dragger(ui, "Near", &mut self.camera.near, |x| x.speed(0.1));
                dragger(ui, "Far", &mut self.camera.far, |x| x.speed(0.1));
            });
        });
    }

    fn render(&mut self, gcx: GraphicsCtx, render_pass: &mut RenderPass) {
        let window = gcx.window.inner_size().cast::<f32>();
        let aspect = window.width / window.height;

        self.uniform
            .upload(&Uniform {
                view_projection: Matrix4::new_perspective(
                    aspect,
                    self.camera.fov,
                    self.camera.near,
                    self.camera.far,
                ) * Matrix4::look_at_rh(
                    &Point3::from(self.camera.position),
                    &Point3::from(self.camera.target),
                    &Vector3::new(0.0, 1.0, 0.0),
                ),
            })
            .unwrap();

        self.render
            .draw(render_pass, &self.index, &self.vertex, 0..self.indicies);
    }
}
