use std::f32::consts::FRAC_PI_2;

use anyhow::Result;
use compute::{
    bindings::{IndexBuffer, UniformBuffer, VertexBuffer},
    export::{
        egui::{Context, Key, PointerButton, Window},
        nalgebra::{Matrix4, Point3, Vector3},
        wgpu::{include_wgsl, Limits, RenderPass, ShaderStages},
        winit::window::WindowAttributes,
    },
    gpu::Gpu,
    interactive::{GraphicsCtx, Interactive},
    pipeline::render::{RenderPipeline, Vertex},
};
use encase::ShaderType;
use marching_cubes::marching_cubes;
use misc::{dragger, vec3_dragger};
use sci_dragger::SciDragValue;
use simulation::{Config, Simulation};

mod marching_cubes;
mod misc;
mod sci_dragger;
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

    iso_level: f32,
}

#[derive(ShaderType, Default)]
struct Uniform {
    view_projection: Matrix4<f32>,
    camera_dir: Vector3<f32>,
}

struct Camera {
    position: Vector3<f32>,
    pitch: f32,
    yaw: f32,

    fov: f32,
    near: f32,
    far: f32,
}

impl Default for Camera {
    fn default() -> Self {
        Camera {
            position: Vector3::zeros(),
            pitch: 0.0,
            yaw: 0.0,

            fov: FRAC_PI_2,
            near: 0.1,
            far: 10_000.0,
        }
    }
}

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
    };

    for _ in 0..100 {
        // todo: move config into simulation
        simulation.tick(&config);
    }

    let (vertices, indices) =
        marching_cubes(&simulation.states[simulation.step % 3], config.size, 0.0);

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

            iso_level: 1e-4,
        },
    )
    .run()?;

    Ok(())
}

impl Interactive for App {
    fn ui(&mut self, _gcx: GraphicsCtx, ctx: &Context) {
        ctx.input(|input| {
            let facing = self.camera.facing();
            let forward = Vector3::new(facing.x, 0.0, facing.z).normalize();
            let right = facing.cross(&Vector3::new(0.0, 1.0, 0.0));
            let directions = [
                (Key::W, forward),
                (Key::S, -forward),
                (Key::A, -right),
                (Key::D, right),
                (Key::Space, Vector3::new(0.0, 1.0, 0.0)),
            ];

            let mut delta = Vector3::zeros();
            delta -= Vector3::new(0.0, 1.0, 0.0) * input.modifiers.shift as u8 as f32;
            for (key, direction) in directions.iter() {
                delta += direction * input.key_down(*key) as u8 as f32;
            }

            self.camera.position +=
                delta.try_normalize(0.0).unwrap_or_default() * 10.0 * input.stable_dt;

            if input.pointer.button_down(PointerButton::Primary) {
                let mouse = -input.pointer.delta() * 0.01;
                self.camera.pitch += mouse.y;
                self.camera.yaw += mouse.x;
            }
        });

        Window::new("Wave Simulator 3D").show(ctx, |ui| {
            ui.horizontal(|ui| {
                SciDragValue::new(&mut self.iso_level).show(ui);
                ui.label("Iso Level");
            });

            if ui.button("Remesh").clicked() {
                let (vertices, indices) = marching_cubes(
                    &self.simulation.states[self.simulation.step % 3],
                    self.config.size,
                    self.iso_level,
                );
                self.vertex.upload(&vertices).unwrap();
                self.index.upload(&indices).unwrap();
                self.indicies = indices.len() as u32;
            }

            ui.separator();

            ui.collapsing("Camera", |ui| {
                ui.horizontal(|ui| {
                    ui.label("Position");
                    vec3_dragger(ui, &mut self.camera.position, |x| x.speed(0.1));
                });
                dragger(ui, "Pitch", &mut self.camera.pitch, |x| x.speed(0.1));
                dragger(ui, "Yaw", &mut self.camera.yaw, |x| x.speed(0.1));
                ui.separator();
                dragger(ui, "Fov", &mut self.camera.fov, |x| x.speed(0.1));
                dragger(ui, "Near", &mut self.camera.near, |x| x.speed(0.1));
                dragger(ui, "Far", &mut self.camera.far, |x| x.speed(0.1));
            });
        });
    }

    fn render(&mut self, gcx: GraphicsCtx, render_pass: &mut RenderPass) {
        let window = gcx.window.inner_size().cast::<f32>();
        let aspect = window.width / window.height;

        let facing = self.camera.facing();
        self.uniform
            .upload(&Uniform {
                view_projection: Matrix4::new_perspective(
                    aspect,
                    self.camera.fov,
                    self.camera.near,
                    self.camera.far,
                ) * Matrix4::look_at_rh(
                    &Point3::from(self.camera.position),
                    &Point3::from(self.camera.position + facing),
                    &Vector3::new(0.0, 1.0, 0.0),
                ),
                camera_dir: facing,
            })
            .unwrap();

        self.render
            .draw(render_pass, &self.index, &self.vertex, 0..self.indicies);
    }
}

impl Camera {
    fn facing(&self) -> Vector3<f32> {
        Vector3::new(
            self.pitch.cos() * self.yaw.sin(),
            self.pitch.sin(),
            self.pitch.cos() * self.yaw.cos(),
        )
    }
}
