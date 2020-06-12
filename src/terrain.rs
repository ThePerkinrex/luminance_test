use cgmath::{InnerSpace, Point3, Vector3};

use wavefront_obj::mtl;

use crate::engine::noise::PerlinNoise;
use crate::engine::spatial::{obj, Vertex, VertexIndex, VertexNormal, VertexPosition};

const MESH_SIZE: f32 = 1.;
const NOISE_SCALE: f64 = 0.02;

pub fn generate(size_x: u32, size_y: u32) -> obj::Obj {
	let mut mesh: Vec<Vec<f64>> = Vec::new();
	let mut row_template = Vec::new();
	row_template.resize(size_x as usize, 0.0);
	mesh.resize(size_y as usize, row_template);

	let n = PerlinNoise::new();

	for y in 0..size_y as usize {
		for x in 0..size_x as usize {
			mesh[y][x] = n.get2d([x as f64 * NOISE_SCALE, y as f64 * NOISE_SCALE]) * 50. - 25.;
			// println!("{}", mesh[y][x])
		}
	}

	// Load mesh into the vertices
	let mut points: Vec<Point3<f32>> = Vec::new();
	for y in 0..size_y as usize {
		for x in 0..size_x as usize {
			points.push(Point3::new(
				x as f32 * MESH_SIZE,
				mesh[y][x] as f32,
				y as f32 * MESH_SIZE,
			));
		}
	}

	// Add indices
	// Calculate normals
	let mut indices: Vec<VertexIndex> = Vec::new();
	let mut normals: Vec<Vector3<f32>> = Vec::new();
	normals.resize(
		(size_x * size_y) as usize,
		Vector3 {
			x: 0.,
			y: 0.,
			z: 0.,
		},
	);
	for y in 0..size_y as usize - 1 {
		for x in 0..size_x as usize - 1 {
			//let i = index(x, y, size_x as usize);
			// Add indices
			indices.append(&mut vec![
				// First triangle
				index(x, y, size_x as usize) as VertexIndex,
				index(x + 1, y, size_x as usize) as VertexIndex,
				index(x, y + 1, size_x as usize) as VertexIndex,
				// Second triangle
				index(x + 1, y + 1, size_x as usize) as VertexIndex,
				index(x + 1, y, size_x as usize) as VertexIndex,
				index(x, y + 1, size_x as usize) as VertexIndex,
			]);

			// Calculate normals
			// 0 1
			// 2 3
			let point0 = points[index(x, y, size_x as usize)];
			let point1 = points[index(x + 1, y, size_x as usize)];
			let point2 = points[index(x, y + 1, size_x as usize)];
			let point3 = points[index(x + 1, y + 1, size_x as usize)];

			// 0
			normals[index(x, y, size_x as usize)] = (normals[index(x, y, size_x as usize)]
				+ point_up((point2 - point1).cross(point1 - point0)))
			.normalize();
			// 1
			normals[index(x + 1, y, size_x as usize)] = (normals[index(x, y, size_x as usize)]
				+ point_up((point0 - point1).cross(point2 - point1)))
			.normalize();
			normals[index(x + 1, y, size_x as usize)] = (normals[index(x, y, size_x as usize)]
				+ point_up((point2 - point1).cross(point3 - point1)))
			.normalize();
			// 2
			normals[index(x, y + 1, size_x as usize)] = (normals[index(x, y, size_x as usize)]
				+ point_up((point1 - point2).cross(point0 - point2)))
			.normalize();
			normals[index(x, y + 1, size_x as usize)] = (normals[index(x, y, size_x as usize)]
				+ point_up((point3 - point2).cross(point1 - point2)))
			.normalize();
			// 3
			normals[index(x + 1, y + 1, size_x as usize)] = (normals[index(x, y, size_x as usize)]
				+ point_up((point1 - point3).cross(point2 - point1)))
			.normalize();
			//vertices.push(Vertex::new(VertexPosition::new([x as f32, y as f32, mesh[y][x] as f32]), VertexNormal::new([0.0, 1.0, 0.0])))
		}
	}

	let mut vertices = Vec::new();
	for y in 0..size_y as usize {
		for x in 0..size_x as usize {
			let i = index(x, y, size_x as usize);
			vertices.push(Vertex::new(
				VertexPosition::new(points[i].into()),
				VertexNormal::new(normals[i].into()),
			))
		}
	}
	//println!("{:?} {:?}", indices, points);
	let material = obj::color_material(
		"terrain",
		mtl::Color {
			r: 1.0,
			g: 0.5,
			b: 0.0,
		},
	);
	//material.color_ambient = mtl::Color {r: 1.0, g: 1.0, b: 1.0};
	let g = obj::Geometry {
		indices,
		vertices,
		material,
	};
	return obj::Obj {
		geometries: vec![g],
	};
}

fn point_up(v: Vector3<f32>) -> Vector3<f32> {
	// if v.z <= 0. {
	// 	-v
	// } else {
	// 	v
	// }
	v
}

fn index(x: usize, y: usize, width: usize) -> usize {
	y * width + x
}
