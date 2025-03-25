use compute::export::nalgebra::Vector3;
use itertools::Itertools;

use crate::{marching_cubes, vertex::Vertex};

pub struct Simulation {
    pub states: Vec<Vec<f32>>,
    pub step: usize,

    pub config: Config,
}

pub struct Config {
    pub size: Vector3<usize>,
    pub v: f32,
    pub dx: f32,
    pub dt: f32,
}

impl Simulation {
    pub fn reset(&mut self) {
        self.states = vec![vec![0.0; self.config.size.iter().product()]; 3];
        self.step = 0;
    }

    /// ```plain
    /// ∂²u        ∂²u   ∂²u   ∂²u
    /// --- = c² ( --- + --- + --- )
    /// ∂t²        ∂x²   ∂y²   ∂z²
    /// ```
    pub fn tick(&mut self) {
        let size = self.config.size;
        let dx = self.config.dx.powi(3);
        let c = self.config.v;

        let (x, y, z) = (Vector3::x(), Vector3::y(), Vector3::z());

        let index = |pos: Vector3<usize>| {
            (pos.x < size.x && pos.y < size.y && pos.z < size.z)
                .then(|| pos.x * size.y * size.z + pos.y * size.z + pos.z)
        };

        let get = |state: &[f32], pos: Vector3<usize>| index(pos).map(|i| state[i]).unwrap_or(0.0);
        let c = c.powi(2) * (self.config.dt / dx);
        let oscilator = (self.step as f32 / 10.0).cos();
        let (prev, curr, next) = self.get_states();

        for pos in (0..size.x)
            .cartesian_product(0..size.y)
            .cartesian_product(0..size.z)
            .map(|((x, y), z)| Vector3::new(x, y, z))
        {
            let idx = index(pos).unwrap();

            let dx = get(curr, pos + x) + get(curr, pos - x);
            let dy = get(curr, pos + y) + get(curr, pos - y);
            let dz = get(curr, pos + z) + get(curr, pos - z);
            let ds = dx + dy + dz - 6.0 * get(curr, pos);
            let mut u = ds * c - prev[idx] + 2.0 * get(curr, pos);

            {
                let center_dist = (Vector3::new(size.x / 2 + 15, size.y / 2, size.z / 2) - pos)
                    .map(|x| x as f32)
                    .magnitude();
                u += (-center_dist).exp() * oscilator;
            }

            {
                let center_dist = (Vector3::new(size.x / 2 - 15, size.y / 2, size.z / 2) - pos)
                    .map(|x| x as f32)
                    .magnitude();
                u += (-center_dist).exp() * oscilator;
            }

            next[idx] = u;
        }

        self.step += 1;
    }

    pub fn triangluate(&self, iso_level: f32) -> (Vec<Vertex>, Vec<u32>) {
        marching_cubes(&self.states[self.step % 3], self.config.size, iso_level)
    }

    fn get_states(&mut self) -> (&[f32], &[f32], &mut [f32]) {
        unsafe {
            let next = &mut *(&mut self.states[(self.step + 1) % 3][..] as *mut _);
            let prev = &self.states[(self.step + 2) % 3];
            let current = &self.states[self.step % 3];

            (prev, current, next)
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            size: Vector3::repeat(100),
            v: 1.0,
            dx: 0.1,
            dt: 0.00001,
        }
    }
}
