use compute::{
    bindings::{IndexBuffer, UniformBuffer, VertexBuffer},
    export::{
        egui::{Context, Slider, Window},
        nalgebra::{Matrix4, Point3, Vector3},
        wgpu::RenderPass,
    },
    interactive::{GraphicsCtx, Interactive},
    pipeline::render::RenderPipeline,
};
use encase::ShaderType;

use crate::{
    camera::Camera,
    simulation::Simulation,
    ui::{dragger, sci_dragger::SciDragValue, vec3_dragger},
    vertex::Vertex,
};

pub struct App {
    pub render: RenderPipeline,
    pub index: IndexBuffer,
    pub vertex: VertexBuffer<Vertex>,
    pub uniform: UniformBuffer<Uniform>,
    pub indicies: u32,

    pub simulation: Simulation,
    pub camera: Camera,
    pub iso_level: f32,
    pub render_config: RenderConfig,
}

#[derive(ShaderType, Clone, Copy)]
pub struct RenderConfig {
    pub ambiant: f32,
    pub intensity: f32,
    pub edge_falloff: f32,
}

#[derive(ShaderType, Default)]
pub struct Uniform {
    view_projection: Matrix4<f32>,
    camera_dir: Vector3<f32>,
    render: RenderConfig,
}

impl Interactive for App {
    fn ui(&mut self, _gcx: GraphicsCtx, ctx: &Context) {
        self.camera.update(ctx);

        Window::new("Wave Simulator 3D")
            .default_width(0.0)
            .show(ctx, |ui| {
                ui.heading("Triangulation");

                ui.horizontal(|ui| {
                    SciDragValue::new(&mut self.iso_level).show(ui);
                    ui.label("Iso Level");
                });

                if ui.button("Remesh").clicked() {
                    let (vertices, indices) = self.simulation.triangluate(self.iso_level);
                    self.vertex.upload(&vertices).unwrap();
                    self.index.upload(&indices).unwrap();
                    self.indicies = indices.len() as u32;
                }

                ui.heading("Rendering");
                ui.horizontal(|ui| {
                    ui.add(Slider::new(&mut self.render_config.ambiant, 0.0..=1.0));
                    ui.label("Ambiant");
                });
                ui.horizontal(|ui| {
                    ui.add(Slider::new(&mut self.render_config.intensity, 0.0..=1.0));
                    ui.label("intensity");
                });
                ui.horizontal(|ui| {
                    ui.add(Slider::new(&mut self.render_config.edge_falloff, 0.0..=1.0));
                    ui.label("Edge Falloff");
                });

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
                render: self.render_config,
            })
            .unwrap();

        self.render
            .draw(render_pass, &self.index, &self.vertex, 0..self.indicies);
    }
}

impl Default for RenderConfig {
    fn default() -> Self {
        Self {
            ambiant: 0.1,
            intensity: 0.9,
            edge_falloff: 0.1,
        }
    }
}
