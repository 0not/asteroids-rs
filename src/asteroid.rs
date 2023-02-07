use crate::prelude::*;
use rand::prelude::*;

use bevy::{
    render::mesh::Mesh,
    render::mesh::Indices,
    render::render_resource::PrimitiveTopology,
    render::mesh::VertexAttributeValues::Float32x3,
};

pub struct AsteroidPlugin;

impl Plugin for AsteroidPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<SpawnAsteroidEvent>()
            .add_event::<DespawnAsteroidEvent>()
            .add_system(despawn_asteroid);
    }
}

#[derive(Component)]
pub struct Asteroid(pub AsteroidSize);

#[derive(Debug, Copy, Clone)]
pub enum AsteroidSize {
    LARGE,
    MEDIUM,
    SMALL,
}

#[derive(Debug, Copy, Clone)]
pub struct SpawnAsteroidEvent {
    pub position:  Vec2,
    pub velocity:  Vec2,
    pub a_size:    AsteroidSize,
}

#[derive(Debug, Copy, Clone)]
pub struct DespawnAsteroidEvent {
    pub entity: Entity,
}

// Define asteroid vertices
pub fn vertices(size: f32) -> Vec<[f32; 3]> {
    let sides = 24;
    let max_delta = 0.2 * size;
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

pub fn spawn_asteroid(
    commands:  &mut Commands,
    meshes:    &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    settings:  &Res<Settings>,
    position:  Vec2,
    velocity:  Vec2,
    a_size:    AsteroidSize,
) {
    let (mesh_size, health) = match a_size {
        AsteroidSize::LARGE  => (settings.asteroid.size_large,  3),
        AsteroidSize::MEDIUM => (settings.asteroid.size_medium, 2),
        AsteroidSize::SMALL  => (settings.asteroid.size_small,  1),
    };

    let (collider, mesh) = collider_and_mesh(mesh_size);

    commands.spawn((
        Name::new("Asteroid"),
        MaterialMesh2dBundle {
            mesh: meshes.add(mesh).into(),
            material: materials.add(ColorMaterial::from(settings.asteroid.color)),
            transform: Transform::from_translation(position.extend(0.0)),
            ..default()
        },
        Asteroid(a_size),
        Health {value: health},
        RigidBody::Dynamic,
        Velocity { linvel: velocity, angvel: 0.0},
        Damping { linear_damping: 0.0, angular_damping: 0. },
        ExternalForce {
            force: Vec2::ZERO,
            torque: 0.0,
        },
        collider,
        // Asteroids are group 3, but can only interact with GROUP_1 and GROUP_2 (ship and bullet)
        CollisionGroups::new(Group::GROUP_3, Group::GROUP_1 | Group::GROUP_2),
        ActiveEvents::COLLISION_EVENTS,
        Restitution::coefficient(1.),
    ));
}

pub fn despawn_asteroid(
    mut commands:  Commands,
    mut meshes:    ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    settings:      Res<Settings>,
    query: Query<(Entity, &Health, &Transform, &Velocity, &Asteroid)>,
) {
    for (entity, health, transform, velocity, asteroid) in query.iter() {
        if health.value <= 0 {
            commands.entity(entity).despawn();
            // TODO: Add to score.

            let size = match asteroid.0 {
                AsteroidSize::LARGE  => AsteroidSize::MEDIUM,
                AsteroidSize::MEDIUM => AsteroidSize::SMALL,
                AsteroidSize::SMALL  => break,
            };

            
            let num_ast  = match size {
                AsteroidSize::LARGE  => panic!("Cannot spawn large asteroid in `despawn_asteroid`."),
                AsteroidSize::MEDIUM => 2,
                AsteroidSize::SMALL  => 3,
            };

            let mut angles: Vec<f32> = Vec::new(); //(0..num_ast).map(|n| n).collect();

            if num_ast == 2 {
                angles.push(std::f32::consts::FRAC_PI_6);
                angles.push(-std::f32::consts::FRAC_PI_6);
            } else {
                angles.push(std::f32::consts::FRAC_PI_4);
                angles.push(0.);
                angles.push(-std::f32::consts::FRAC_PI_4);
            }

            for n in 0..num_ast {

                let position = transform.translation.truncate();
                let velocity = Vec2::from_angle(angles[n]).rotate(velocity.linvel);

                // Spawn new asteroids
                spawn_asteroid(
                    &mut commands,
                    &mut meshes,
                    &mut materials,
                    &settings,
                    position,
                    velocity,
                    size,
                )
            }
        }
    }
}