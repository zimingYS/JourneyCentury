use std::f32::consts::PI;
use bevy::prelude::*;
use bevy::prelude::light_consts::lux;

pub fn setup_lighting(mut commands: Commands) {
    // 主方向光（模拟太阳）
    commands.spawn(
        (DirectionalLight {
            color: Color::srgb(0.98, 0.95, 0.82), // 暖色调
            illuminance: lux::RAW_SUNLIGHT,              // 亮度（lux）
            shadows_enabled: true,              // 启用阴影
            shadow_depth_bias: 0.3,             // 减少阴影伪影
            shadow_normal_bias: 1.2,            // 修复阴影偏移
            ..default()
        },
         Transform::from_rotation(Quat::from_euler(
             EulerRot::XYZ,
             -45.0f32.to_radians(), // 俯角
             45.0f32.to_radians(),  // 水平角度
             0.0,)
         ).looking_at(Vec3::ZERO, Vec3::Y),
        ));

    // // 环境光（整体基础照明）
    // commands.insert_resource(AmbientLight {
    //     color: Color::srgb(0.2, 0.2, 0.2),       // 冷色调补光
    //     brightness: 0.8,                        // 增强暗部细节
    //     affects_lightmapped_meshes: false,
    // });
}

pub fn adjust_lighting(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<DirectionalLight>>,
) {
    query.iter_mut().for_each(|mut tf| tf.rotate_x(-time.delta_secs() * PI / 10.0));
}