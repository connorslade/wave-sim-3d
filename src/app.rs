use compute::{
    bindings::UniformBuffer,
    export::{
        egui::{Context, Key, Slider, Window},
        nalgebra::Vector3,
        wgpu::RenderPass,
    },
    interactive::{GraphicsCtx, Interactive},
    pipeline::render::RenderPipeline,
};
use encase::ShaderType;

use crate::{
    camera::Camera,
    simulation::Simulation,
    ui::{dragger, sci_dragger, vec3_dragger},
};

pub struct App {
    pub render: RenderPipeline,
    pub uniform: UniformBuffer<Uniform>,

    pub simulation: Simulation,
    pub camera: Camera,
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
    size: Vector3<u32>,
    step: u32,
    camera: Camera,
    render: RenderConfig,
}

impl Interactive for App {
    fn init(&mut self, _gcx: GraphicsCtx) {
        self.camera.position = self.simulation.config.size.map(|x| x as f32) / 2.0;
    }

    fn ui(&mut self, _gcx: GraphicsCtx, ctx: &Context) {
        self.camera.update(ctx);

        Window::new("Wave Simulator 3D")
            .default_width(0.0)
            .show(ctx, |ui| {
                ui.heading("Simulation");
                sci_dragger(ui, "dx (m)", &mut self.simulation.config.dx);
                sci_dragger(ui, "dt (s)", &mut self.simulation.config.dt);
                sci_dragger(ui, "Wave Speed (m/s)", &mut self.simulation.config.v);
                // let prev_energy = self.energy;
                // ui.checkbox(&mut self.energy, "Wave Energy");
                // self.scheduled_remesh |= prev_energy != self.energy;

                ui.add_space(8.0);
                ui.horizontal(|ui| {
                    let t_down = ui.input(|input| input.key_down(Key::T));
                    let tick = ui.button("Tick").clicked() || t_down;
                    let reset = ui.button("Reset").clicked();

                    reset.then(|| self.simulation.reset());
                    tick.then(|| self.simulation.tick());
                });

                ui.add_space(8.0);
                ui.heading("Rendering");
                // let (prev_use_iso_level, prev_iso_level) = (self.use_iso_level, self.iso_level);
                // ui.horizontal(|ui| {
                //     ui.checkbox(&mut self.use_iso_level, "");
                //     SciDragValue::new(&mut self.iso_level).show(ui);
                //     ui.label("Iso Level");
                // });

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

                ui.add_space(8.0);
                ui.collapsing("Camera", |ui| {
                    ui.horizontal(|ui| {
                        ui.label("Position");
                        vec3_dragger(ui, &mut self.camera.position, |x| x.speed(0.1));
                    });
                    dragger(ui, "Pitch", &mut self.camera.pitch, |x| x.speed(0.1));
                    dragger(ui, "Yaw", &mut self.camera.yaw, |x| x.speed(0.1));
                    dragger(ui, "Fov", &mut self.camera.fov, |x| x.speed(0.1));
                });
            });
    }

    fn render(&mut self, gcx: GraphicsCtx, render_pass: &mut RenderPass) {
        let window = gcx.window.inner_size().cast::<f32>();
        self.camera.aspect = window.width / window.height;
        self.uniform
            .upload(&Uniform {
                size: self.simulation.config.size,
                step: self.simulation.config.step,
                camera: self.camera,
                render: self.render_config,
            })
            .unwrap();

        self.render.draw_quad(render_pass, 0..1);
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
