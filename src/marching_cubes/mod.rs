use compute::{
    export::nalgebra::{Vector2, Vector3},
    pipeline::render::Vertex,
};
use table::{EDGE_TABLE, TRIANGULATION_TABLE};

mod table;

pub fn marching_cubes(
    scalar_field: &[f32],
    size: Vector3<usize>,
    iso_level: f32,
) -> (Vec<Vertex>, Vec<u32>) {
    let (mut vertices, mut indices) = (Vec::new(), Vec::new());

    for x in 0..size.x - 1 {
        for y in 0..size.y - 1 {
            for z in 0..size.z - 1 {
                let mut cube_index = 0;

                let mut grid = [(Vector3::zeros(), 0.0); 8];
                let positions = [
                    (x as f32, y as f32, z as f32),
                    ((x + 1) as f32, y as f32, z as f32),
                    ((x + 1) as f32, y as f32, (z + 1) as f32),
                    (x as f32, y as f32, (z + 1) as f32),
                    (x as f32, (y + 1) as f32, z as f32),
                    ((x + 1) as f32, (y + 1) as f32, z as f32),
                    ((x + 1) as f32, (y + 1) as f32, (z + 1) as f32),
                    (x as f32, (y + 1) as f32, (z + 1) as f32),
                ];

                for (i, &(px, py, pz)) in positions.iter().enumerate() {
                    let index =
                        (px as usize) * size.y * size.z + (py as usize) * size.z + (pz as usize);
                    let value = scalar_field[index];

                    grid[i] = (Vector3::new(px, py, pz), value);
                    if value < iso_level {
                        cube_index |= 1 << i;
                    }
                }

                let edge = EDGE_TABLE[cube_index];

                let mut vertlist = [Vector3::zeros(); 12];
                for (i, &(p1, p2)) in [
                    (0, 1),
                    (1, 2),
                    (2, 3),
                    (3, 0),
                    (4, 5),
                    (5, 6),
                    (6, 7),
                    (7, 4),
                    (0, 4),
                    (1, 5),
                    (2, 6),
                    (3, 7),
                ]
                .iter()
                .enumerate()
                {
                    if edge & (1 << i) != 0 {
                        vertlist[i] = vertex_interp(iso_level, grid[p1], grid[p2]);
                    }
                }

                let triangles = TRIANGULATION_TABLE[cube_index];
                for triangle in triangles.chunks(3) {
                    vertices.push(Vertex::new(
                        vertlist[triangle[0] as usize].push(1.0),
                        Vector2::zeros(),
                    ));
                    vertices.push(Vertex::new(
                        vertlist[triangle[1] as usize].push(1.0),
                        Vector2::zeros(),
                    ));
                    vertices.push(Vertex::new(
                        vertlist[triangle[2] as usize].push(1.0),
                        Vector2::zeros(),
                    ));
                    indices.extend_from_slice(&[
                        (vertices.len() - 1) as u32,
                        (vertices.len() - 2) as u32,
                        (vertices.len() - 3) as u32,
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
