use compute::export::nalgebra::Vector3;

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
    /// ∂²u        ∂²x   ∂²y   ∂²z
    /// --- = c² ( --- + --- + --- )
    /// ∂t²        ∂t²   ∂t²   ∂t²
    /// ```
    fn tick(&mut self) {}
}

impl Default for Config {
    fn default() -> Self {
        Config {
            size: Vector3::repeat(100),
            c: 1.0,
            ds: 0.01 * 0.01 * 0.01,
            dt: 0.01,
        }
    }
}
