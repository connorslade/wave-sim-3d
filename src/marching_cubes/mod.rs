/// Implementation of <https://paulbourke.net/geometry/polygonise>.
use compute::{
    export::nalgebra::{Vector2, Vector3},
    pipeline::render::Vertex,
};
use table::{EDGE_TABLE, TRIANGULATION_TABLE};

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
    let (mut vertices, mut indices) = (Vec::new(), Vec::new());

    for x in 0..size.x - 1 {
        for y in 0..size.y - 1 {
            for z in 0..size.z - 1 {
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
                    for &vert in triangle {
                        vertices.push(Vertex::new(
                            vertlist[vert as usize].push(1.0),
                            Vector2::zeros(),
                        ));
                    }

                    indices.extend_from_slice(&[
                        (vertices.len() - 3) as u32,
                        (vertices.len() - 2) as u32,
                        (vertices.len() - 1) as u32,
                    ]);
                }
            }
        }
    }

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
