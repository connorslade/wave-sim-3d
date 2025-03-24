use compute::export::{
    nalgebra::{Vector3, Vector4},
    wgpu::{VertexAttribute, VertexBufferLayout, VertexFormat, VertexStepMode},
};
use encase::ShaderType;

pub const VERTEX_BUFFER_LAYOUT: VertexBufferLayout = VertexBufferLayout {
    array_stride: 32, // NOTE: WGSL alignment rules factor into this
    step_mode: VertexStepMode::Vertex,
    attributes: &[
        VertexAttribute {
            format: VertexFormat::Float32x4,
            offset: 0,
            shader_location: 0,
        },
        VertexAttribute {
            format: VertexFormat::Float32x3,
            offset: 4 * 4,
            shader_location: 1,
        },
    ],
};

#[derive(ShaderType, Clone, Copy)]
pub struct Vertex {
    pub position: Vector4<f32>,
    pub normal: Vector3<f32>,
}

impl Vertex {
    pub fn new(position: Vector4<f32>, normal: Vector3<f32>) -> Self {
        Self { position, normal }
    }
}
