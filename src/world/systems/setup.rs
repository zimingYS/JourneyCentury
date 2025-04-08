use std::collections::VecDeque;
use bevy::prelude::*;
use bevy::utils::HashMap;
use noise::{NoiseFn, Perlin};
use rand::Rng;
use crate::world::terrain;

pub fn setup_world(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let material = materials.add(StandardMaterial {
        base_color: Color::rgb(0.8, 0.7, 0.6),
        perceptual_roughness: 0.9,
        ..default()
    });
    let seed = rand::thread_rng().gen_range(1..u32::MAX);
    let perlin = Perlin::new(seed);

    commands.insert_resource(terrain::World {
        chunks: HashMap::new(),
        material,
        noise: perlin,
        generation_queue: VecDeque::new(),
    });


    // 添加太阳光源
    commands.spawn((
        DirectionalLightBundle {
            directional_light: DirectionalLight {
                color: Color::rgb(0.98, 0.95, 0.82),
                illuminance: 100_000.0,
                shadows_enabled: false,
                shadow_depth_bias: 0.2,
                shadow_normal_bias: 0.8,
                ..default()
            },
            transform: Transform::from_rotation(
                Quat::from_euler(EulerRot::XYZ, -45.0f32.to_radians(), 45.0f32.to_radians(), 0.0)
            ),
            ..default()
        },
        Name::new("Sunlight"),
    ));

    // 环境光配置
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 100.0,
    });
}