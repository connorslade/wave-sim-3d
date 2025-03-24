//! Implementation of <https://paulbourke.net/geometry/polygonise>.

use std::collections::HashMap;

use compute::export::nalgebra::Vector3;
use itertools::Itertools;
use ordered_float::OrderedFloat;
use table::{EDGE_TABLE, TRIANGULATION_TABLE};

use crate::vertex::Vertex;

mod table;

#[rustfmt::skip]
const EDGE_CONNECTIONS: [(usize, usize); 12] = [
    (0, 1), (1, 2), (2, 3), (3, 0),
    (4, 5), (5, 6), (6, 7), (7, 4),
    (0, 4), (1, 5), (2, 6), (3, 7)
];

const GRID_POINTS: [Vector3<usize>; 8] = [
    Vector3::new(0, 0, 0),
    Vector3::new(1, 0, 0),
    Vector3::new(1, 0, 1),
    Vector3::new(0, 0, 1),
    Vector3::new(0, 1, 0),
    Vector3::new(1, 1, 0),
    Vector3::new(1, 1, 1),
    Vector3::new(0, 1, 1),
];

pub fn marching_cubes(
    scalar_field: &[f32],
    size: Vector3<usize>,
    iso_level: f32,
) -> (Vec<Vertex>, Vec<u32>) {
    let mut vertex_lookup = HashMap::<Vector3<OrderedFloat<f32>>, u32>::new();
    let mut vertices = Vec::<Vertex>::new();
    let mut indices = Vec::new();

    for ((x, y), z) in (0..size.x - 1)
        .cartesian_product(0..size.y - 1)
        .cartesian_product(0..size.z - 1)
    {
        let mut grid = [(Vector3::zeros(), 0.0); 8];
        let mut cube_index = 0;

        for (i, offset) in GRID_POINTS.iter().enumerate() {
            let pos = Vector3::new(x, y, z) + offset;

            let index = pos.x * size.y * size.z + pos.y * size.z + pos.z;
            let value = scalar_field[index];

            grid[i] = (pos.map(|x| x as f32), value);
            cube_index |= ((value < iso_level) as usize) << i;
        }

        let edge = EDGE_TABLE[cube_index];
        let mut vertlist = [Vector3::zeros(); 12];
        for (i, &(p1, p2)) in EDGE_CONNECTIONS
            .iter()
            .enumerate()
            .filter(|(i, _)| edge & (1 << i) != 0)
        {
            vertlist[i] = vertex_interp(iso_level, grid[p1], grid[p2]);
        }

        let triangles = TRIANGULATION_TABLE[cube_index];
        for triangle in triangles.chunks(3) {
            let normal = (vertlist[triangle[1] as usize] - vertlist[triangle[0] as usize])
                .cross(&(vertlist[triangle[2] as usize] - vertlist[triangle[0] as usize]))
                .normalize();

            let get_point_idx = |vert: u8| {
                let point = vertlist[vert as usize];
                let orderd = point.map(OrderedFloat);
                if let Some(&idx) = vertex_lookup.get(&orderd) {
                    vertices[idx as usize].normal += normal;
                    return idx;
                }

                let idx = vertices.len() as u32;
                vertices.push(Vertex::new(point.push(1.0), normal));
                vertex_lookup.insert(orderd, idx);
                idx
            };

            let points = triangle.iter().copied().map(get_point_idx);
            indices.extend(points);
        }
    }

    vertices
        .iter_mut()
        .for_each(|v| v.normal = v.normal.normalize());

    (vertices, indices)
}

fn vertex_interp(
    isolevel: f32,
    (point_1, val_1): (Vector3<f32>, f32),
    (point_2, val_2): (Vector3<f32>, f32),
) -> Vector3<f32> {
    let mu = (isolevel - val_1) / (val_2 - val_1);
    Vector3::new(
        point_1.x + mu * (point_2.x - point_1.x),
        point_1.y + mu * (point_2.y - point_1.y),
        point_1.z + mu * (point_2.z - point_1.z),
    )
}
