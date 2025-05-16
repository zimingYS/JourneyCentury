use std::f32::consts::PI;
use bevy::prelude::*;
use bevy::prelude::light_consts::lux;

pub fn setup_lighting(mut commands: Commands) {
    // 主方向光（模拟太阳）
    commands.spawn((
        DirectionalLight {
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
         Name::new("Sun"),
    ));

    // 月亮
    commands.spawn((
        DirectionalLight {
            color: Color::srgb(0.85, 0.9, 0.98),  // 冷色调
            illuminance: lux::FULL_MOON_NIGHT * 50.0,      // 月光亮度
            shadows_enabled: false,                 // 启用阴影
            shadow_depth_bias: 0.3,                // 减少阴影伪影
            shadow_normal_bias: 1.2,               // 修复阴影偏移
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(
            EulerRot::XYZ,
            45.0f32.to_radians(),  // 仰角（与太阳相反）
            225.0f32.to_radians(), // 水平角度相差180度
            0.0,
        )).looking_at(Vec3::ZERO, Vec3::Y),
        Name::new("Moon"),
    ));

    // // 环境光（整体基础照明）
    // commands.insert_resource(AmbientLight {
    //     color: Color::srgb(0.2, 0.2, 0.2),       // 冷色调补光
    //     brightness: 800.0,                        // 增强暗部细节
    //     affects_lightmapped_meshes: false,
    // });
}

pub fn adjust_lighting(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Name), With<DirectionalLight>>,
) {
    let day_length_s = 10.0;
    let t = (time.elapsed_secs() - 1.0).max(0.0) + day_length_s * 0.3;
    let earth_tilt_rad = PI / 3.0;
    let day_fract = ((t % day_length_s) / day_length_s).clamp(0.0, 1.0);

    query.iter_mut().for_each(|(mut tf, name)| {
        let moon_offset = if name.as_str() == "Sun" { 0.0 } else { 1.1 };
        tf.rotation = Quat::from_euler(
            EulerRot::ZYX,
            earth_tilt_rad,
            0.0,
            -day_fract * PI * 2.0 + moon_offset * PI,
        );
    });
}