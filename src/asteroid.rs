use crate::prelude::*;
use rand::prelude::*;

use bevy::{
    render::mesh::Mesh,
    render::mesh::Indices,
    render::render_resource::PrimitiveTopology,
    render::mesh::VertexAttributeValues::Float32x3,
};

pub enum AsteroidSize {
    LARGE,
    MEDIUM,
    SMALL,
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
    position: Vec2,
    velocity: Vec2,
    size: AsteroidSize,
) {
    let (size, health) = match size {
        AsteroidSize::LARGE  => (settings.asteroid.size_large,  3),
        AsteroidSize::MEDIUM => (settings.asteroid.size_medium, 2),
        AsteroidSize::SMALL  => (settings.asteroid.size_small,  1),
    };

    let (collider, mesh) = asteroid::collider_and_mesh(size);

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(mesh).into(),
            material: materials.add(ColorMaterial::from(settings.asteroid.color)),
            transform: Transform::from_translation(position.extend(0.0)),
            ..default()
        },
        Asteroid,
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

pub fn asteroid_bullet_collision(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Health), With<Asteroid>>,
    mut collision_events: EventReader<CollisionEvent>,
) {
    for collision_event in collision_events.iter() {
        println!("Received collision event: {:?}", collision_event);

        for (asteroid, mut health) in query.iter_mut() {
            if let CollisionEvent::Started(e1, e2, _) = collision_event {
                if e1 != &asteroid && e2 != &asteroid {
                    continue
                }

                let other = if e1 == &asteroid { // e1 is the Asteroid
                    e2
                } else {// e2 is the Asteroid
                    e1
                };

                health.value -= 1;

                if health.value < 1 {
                    commands.entity(asteroid).despawn();
                    // TODO: Spawn two or three new asteroids with same momentum as previous
                }

                // TODO: if `other` is PlayerShip, then cause damage to ship

                // TODO: if `other` is Bullet, shorten LifeTime
            }
        }

        // let (entity_1, entity_2) = match collision_event {
        //     Started(e1, e2, CollisionEventFlags) => (e1, e2),
        //     _ => 
        // }
    }
}

// pub fn asteroid_bullet_collision(
//     mut commands: Commands,
//     mut query: Query<(Entity, &mut Health), With<Asteroid>>,
// ) {
//     for (entity, mut health) in query.iter_mut() {
//         // Despawn
//         commands.entity(entity).despawn();
//     }
// }