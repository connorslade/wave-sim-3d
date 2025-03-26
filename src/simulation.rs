use anyhow::{Ok, Result};
use compute::{
    bindings::{MappedStorageBuffer, UniformBuffer},
    export::{nalgebra::Vector3, wgpu::include_wgsl},
    gpu::Gpu,
    misc::mutability::Mutable,
    pipeline::compute::ComputePipeline,
};
use encase::ShaderType;

use crate::{marching_cubes, vertex::Vertex};

const WORKGROUP_SIZE: Vector3<u32> = Vector3::new(8, 8, 8);

pub struct Simulation {
    pub pipeline: ComputePipeline,

    pub states: MappedStorageBuffer<Mutable>,
    pub energy: MappedStorageBuffer<Mutable>,
    pub uniform: UniformBuffer<Config>,

    pub config: Config,
}

#[derive(ShaderType)]
pub struct Config {
    pub size: Vector3<u32>,
    pub v: f32,
    pub dx: f32,
    pub dt: f32,

    pub step: u32,
}

impl Simulation {
    pub fn new(gpu: &Gpu, config: Config) -> Result<Self> {
        let cells = config.size.iter().product::<u32>() as u64;
        let states = gpu.create_mapped_storage(cells * 4 * 3)?;
        let energy = gpu.create_mapped_storage(cells * 4)?;
        let uniform = gpu.create_uniform(&config)?;

        let pipeline = gpu
            .compute_pipeline(include_wgsl!("shaders/compute.wgsl"))
            .bind(&uniform)
            .bind(&states)
            .finish();

        Ok(Self {
            pipeline,
            states,
            energy,
            uniform,
            config,
        })
    }

    pub fn reset(&mut self) {
        self.config.step = 0;
        self.uniform.upload(&self.config).unwrap();

        self.states.as_mut_slice(..).fill(0);
        self.energy.as_mut_slice(..).fill(0);
    }

    pub fn tick(&mut self) {
        self.config.step += 1;

        self.uniform.upload(&self.config).unwrap();
        self.pipeline
            .dispatch(self.config.size.component_div(&WORKGROUP_SIZE));
    }

    pub fn triangluate(&self, iso_level: f32) -> (Vec<Vertex>, Vec<u32>) {
        let cells = self.config.size.iter().product::<u32>() as u64;
        let step = (self.config.step % 3) as u64;
        let range = (step * cells)..(((step + 1) % 3) * cells);

        let state = self.states.as_mut_slice(range);
        let state = bytemuck::cast_slice(&state);
        marching_cubes(&state, self.config.size.map(|x| x as usize), iso_level)
    }

    pub fn triangluate_energy(&self, iso_level: f32) -> (Vec<Vertex>, Vec<u32>) {
        let energy = self.energy.as_mut_slice(..);
        let energy = bytemuck::cast_slice(&energy);
        marching_cubes(&energy, self.config.size.map(|x| x as usize), iso_level)
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            size: Vector3::repeat(100),
            v: 1.0,
            dx: 0.1,
            dt: 0.00001,
            step: 0,
        }
    }
}
