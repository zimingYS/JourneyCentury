use bevy::prelude::*;

pub fn setup_lighting(mut commands: Commands) {
    // 主方向光（模拟太阳）
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            color: Color::rgb(0.98, 0.95, 0.82), // 暖色调
            illuminance: 50_000.0,              // 亮度（lux）
            shadows_enabled: true,              // 启用阴影
            shadow_depth_bias: 0.3,             // 减少阴影伪影
            shadow_normal_bias: 1.2,            // 修复阴影偏移
            ..default()
        },
        transform: Transform::from_rotation(Quat::from_euler(
            EulerRot::XYZ,
            -45.0f32.to_radians(), // 俯角
            45.0f32.to_radians(),  // 水平角度
            0.0,
        )),
        ..default()
    }).insert(Name::new("Sunlight"));

    // 环境光（整体基础照明）
    commands.insert_resource(AmbientLight {
        color: Color::rgb(0.2, 0.2, 0.2),       // 冷色调补光
        brightness: 0.8,                        // 增强暗部细节
    });
}

pub fn adjust_lighting(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<DirectionalLight>>,
) {
    // 模拟太阳移动
    for mut light_transform in &mut query {
        light_transform.rotate_y(time.delta_secs() * 0.05);
    }
}