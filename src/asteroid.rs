use crate::prelude::*;
use rand::prelude::*;

use bevy::{
    render::mesh::Mesh,
    render::mesh::Indices,
    render::render_resource::PrimitiveTopology,
    render::mesh::VertexAttributeValues::Float32x3,
};

// Define asteroid vertices
pub fn vertices(size: f32) -> Vec<[f32; 3]> {
    let sides = 24;
    let max_delta = 10.;
    let mut rng = rand::thread_rng();

    let mesh: Mesh = shape::RegularPolygon::new(size, sides).into();
    let mut positions = match mesh.attribute(Mesh::ATTRIBUTE_POSITION) {
        Some(Float32x3(pos)) => pos.to_vec(),
        _ => panic!("Could not get vertex positions."),
    };

    // Calculate center of mass
    let mut cm = Vec3::from_array([0., 0., 0.]);
    for pos in positions.iter() {
        cm[0] += pos[0] / sides as f32;
        cm[1] += pos[1] / sides as f32;
        cm[1] += pos[2] / sides as f32;
    }

    for pos in positions.iter_mut() {
        let delta: f32 = rng.gen::<f32>()*max_delta - max_delta/2.;
        let vert = Vec3::from_array(*pos);
        let normal = (vert - cm).normalize_or_zero();
        *pos = (vert - delta*normal).into();
    }

    positions

    // vec![[0., size, 0.], [s, -size/2., 0.], [-s, -size/2., 0.]]
}

pub fn collider(size: f32, verts: &Vec<[f32; 3]>) -> Collider {
    let points: Vec<_> = verts
        .iter()
        .map(|x| Vec2::new(x[0], x[1]))
        .collect();

    Collider::convex_hull(&points).unwrap_or(Collider::ball(size))
}

pub fn mesh(verts: Vec<[f32; 3]>) -> Mesh {
    let sides = verts.len();

    let mut indices = Vec::with_capacity((sides - 2) * 3);
    for i in 1..(sides as u32 - 1) {
        indices.extend_from_slice(&[0, i + 1, i]);
    }

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, verts);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, vec![[0., 1., 0.]; sides]);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, vec![[0., 0.]; sides]);
    mesh.set_indices(Some(Indices::U32(indices)));
    mesh
}

pub fn collider_and_mesh(size: f32) -> (Collider, Mesh) {
    let verts = vertices(size);
    
    (collider(size, &verts), mesh(verts))
}