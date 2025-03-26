use anyhow::{Ok, Result};
use compute::{
    bindings::{StorageBuffer, UniformBuffer},
    export::{nalgebra::Vector3, wgpu::include_wgsl},
    gpu::Gpu,
    misc::mutability::Mutable,
    pipeline::compute::ComputePipeline,
};
use encase::ShaderType;

const WORKGROUP_SIZE: Vector3<u32> = Vector3::new(8, 8, 8);

pub struct Simulation {
    pub pipeline: ComputePipeline,

    pub states: StorageBuffer<Vec<f32>, Mutable>,
    pub energy: StorageBuffer<Vec<f32>, Mutable>,
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
        let cells = config.cells();
        let states = gpu.create_storage(&vec![0.0; cells * 3])?;
        let energy = gpu.create_storage(&vec![0.0; cells])?;
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

        self.states
            .upload(&vec![0.0; self.config.cells() * 3])
            .unwrap();
        self.energy.upload(&vec![0.0; self.config.cells()]).unwrap();
    }

    pub fn tick(&mut self) {
        self.uniform.upload(&self.config).unwrap();
        self.config.step += 1;

        self.pipeline
            .dispatch(self.config.size.component_div(&WORKGROUP_SIZE));
    }
}

impl Config {
    fn cells(&self) -> usize {
        self.size.iter().product::<u32>() as usize
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
