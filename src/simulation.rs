use compute::export::nalgebra::Vector3;
use itertools::Itertools;

pub struct Simulation {
    pub states: Vec<Vec<f32>>,
    pub step: usize,
}

pub struct Config {
    pub size: Vector3<usize>,
    pub c: f32,
    pub ds: f32,
    pub dt: f32,
}

impl Simulation {
    /// ```plain
    /// ∂²u        ∂²u   ∂²u   ∂²u
    /// --- = c² ( --- + --- + --- )
    /// ∂t²        ∂x²   ∂y²   ∂z²
    /// ```
    pub fn tick(&mut self, config: &Config) {
        let size = config.size;

        let index = |x: usize, y: usize, z: usize| {
            (x < size.x && y < size.y && z < size.z).then(|| x * size.y * size.z + y * size.z + z)
        };

        for ((x, y), z) in (0..size.x)
            .cartesian_product(0..size.y)
            .cartesian_product(0..size.z)
        {
            let get_last = |x: usize, y: usize, z: usize| {
                index(x, y, z)
                    .map(|i| self.states[(self.step + 2) % 3][i])
                    .unwrap_or(0.0)
            };
            let get = |x: usize, y: usize, z: usize| {
                index(x, y, z)
                    .map(|i| self.states[self.step % 3][i])
                    .unwrap_or(0.0)
            };

            let dx = get(x + 1, y, z) + get(x - 1, y, z) - 2.0 * get(x, y, z);
            let dy = get(x, y + 1, z) + get(x, y - 1, z) - 2.0 * get(x, y, z);
            let dz = get(x, y, z + 1) + get(x, y, z - 1) - 2.0 * get(x, y, z);
            let ds = dx + dy + dz;

            let mut next = config.c.powi(2) * ds * (config.dt / config.ds) - get_last(x, y, z)
                + 2.0 * get(x, y, z);

            let center_dist = (size.map(|x| x as f32 / 2.0)
                - Vector3::new(x as f32, y as f32, z as f32))
            .magnitude();
            next += (-center_dist).exp() * (self.step as f32 / 10.0).cos();

            self.step += 1;
            self.states[self.step % 3][index(x, y, z).unwrap()] = next;
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            size: Vector3::repeat(100),
            c: 0.5,
            ds: 0.001,
            dt: 0.0001,
        }
    }
}
