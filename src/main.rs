mod components;
mod misc_systems;
mod settings;
mod setup;

pub mod asteroid;
pub mod player_ship;

mod prelude {
    pub use bevy::{
        prelude::*, 
        sprite::MaterialMesh2dBundle,
        core_pipeline::clear_color::ClearColorConfig,
        utils::Instant,
        utils::Duration,
    };
    
    pub use bevy_rapier2d::prelude::*;

    pub use crate::{
        asteroid,
        components::*,
        misc_systems::*,
        settings::*,
        setup::*,
        player_ship::{self, PlayerShipPlugin},
    };
}

use prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Asteroids Clone".into(),
                ..default()
            },
            ..default()
        }))
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(1.0))
        // .add_plugin(RapierDebugRenderPlugin::default())
        .insert_resource(RapierConfiguration {
            gravity: Vec2::ZERO,
            ..default()
        })
        .init_resource::<Settings>()
        .add_startup_system(setup_camera)
        .add_startup_system(setup_player)
        .add_startup_system(setup_asteroids)
        .add_plugin(PlayerShipPlugin)
        .add_system(periodic_bc)
        .add_system(tick_lifetime)
        .add_system_to_stage(CoreStage::PostUpdate, asteroid::asteroid_bullet_collision)
        .run();
}

