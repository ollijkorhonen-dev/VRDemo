use bevy::prelude::*;
use bevy_mod_openxr::{add_xr_plugins, resources::OxrSessionConfig};
use bevy_mod_openxr::types::EnvironmentBlendMode;
use bevy_rapier3d::prelude::*;
use bevy_xr_utils;
use bevy_mod_xr::session::XrTrackingRoot;
//use bevy_inspector_egui::bevy_egui::EguiPlugin;
//use bevy_inspector_egui::quick::WorldInspectorPlugin;

// todo Publish to bevy_xr_utils context.
mod xr_camera_mirroring_plugin;

pub struct VrDemoPlugin;

#[derive(Component)]
pub struct SteerablePlatform;

#[derive(Component)]
pub struct PlatformController {
    pub speed: f32,
    pub rotation_speed: f32,
}

impl Plugin for VrDemoPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugins(add_xr_plugins(
            DefaultPlugins.build(),
        ))
        .insert_resource(OxrSessionConfig {
            blend_mode_preference: vec![
                EnvironmentBlendMode::ALPHA_BLEND,
                EnvironmentBlendMode::ADDITIVE,
                EnvironmentBlendMode::OPAQUE,
            ],
            ..default()
        })
        .add_plugins(bevy_mod_xr::hand_debug_gizmos::HandGizmosPlugin)
        .add_plugins(bevy_xr_utils::tracking_utils::TrackingUtilitiesPlugin)
        .add_plugins(xr_camera_mirroring_plugin::XrCameraMirroringPlugin)
            //.add_plugins(EguiPlugin::default())
            //.add_plugins(WorldInspectorPlugin::new())
        .add_systems(Startup, setup)
        .add_systems(Update, (platform_controls, follow_xr_rig))
        .insert_resource(ClearColor(Color::NONE));
    }
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // circular base
    commands.spawn((
        Mesh3d(meshes.add(Circle::new(4.0))),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
    ));

    // cube
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
        Transform::from_xyz(0.0, 0.5, 0.0),
    ));

    // light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));

    // XrCamera follows this moving platform
    commands.spawn((
        SteerablePlatform,
        PlatformController {
            speed: 0.2,
            rotation_speed: 0.5,
        },
        Mesh3d(meshes.add(Cuboid::new(2.0, 0.2, 2.0))),
        MeshMaterial3d(materials.add(Color::srgb_u8(100, 200, 100))),
        Transform::from_xyz(-2.0, 0.10, 0.0), // Position in front and below VR camera
        RigidBody::KinematicPositionBased,
        Collider::cuboid(1.0, 0.1, 1.0),
    ));
}

fn platform_controls(
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&PlatformController, &mut Transform), (With<SteerablePlatform>, Without<XrTrackingRoot>)>,
) {
    for (controller, mut transform) in query.iter_mut() {
        let mut movement = Vec3::ZERO;
        let mut rotation = 0.0;

        // Keyboard controls for testing (WASD + QE for rotation)
        if keys.pressed(KeyCode::KeyW) {
            movement.z -= 1.0;
        }
        if keys.pressed(KeyCode::KeyS) {
            movement.z += 1.0;
        }
        if keys.pressed(KeyCode::KeyA) {
            movement.x -= 1.0;
        }
        if keys.pressed(KeyCode::KeyD) {
            movement.x += 1.0;
        }
        if keys.pressed(KeyCode::KeyQ) {
            rotation += 1.0;
        }
        if keys.pressed(KeyCode::KeyE) {
            rotation -= 1.0;
        }

        // Normalize movement vector and apply speed
        if movement != Vec3::ZERO {
            movement = movement.normalize() * controller.speed;
            // Transform movement to local space
            let forward = transform.forward();
            let right = transform.right();
            let local_movement = forward * movement.z + right * movement.x;
            // Apply movement directly to transform (kinematic body will follow)
            transform.translation += local_movement;
        }

        // Apply rotation
        if rotation != 0.0 {
            // Apply rotation directly to transform
            let rotation_amount = rotation * controller.rotation_speed * 0.1; // Scale down for smoother rotation
            let rotation_quat = Quat::from_rotation_y(rotation_amount);
            transform.rotation = rotation_quat * transform.rotation;
        }
    }
}

fn follow_xr_rig(
    steerable_platform: Query<&Transform, With<SteerablePlatform>>,
    mut xr_tracking_root: Query<&mut Transform, (With<XrTrackingRoot>, Without<SteerablePlatform>)>,
) {
    if let Some(steerable_platform_transform) = steerable_platform.iter().next() {
        // there should be only one xr_tracking_root
        for mut xr_tracking_root_transform in xr_tracking_root.iter_mut() {
            *xr_tracking_root_transform = *steerable_platform_transform;
            // assert if counts many
        }
    }
}