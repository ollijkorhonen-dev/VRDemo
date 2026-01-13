use std::cmp::PartialEq;
use bevy::prelude::*;
use bevy_mod_xr::camera::XrCamera;
use bevy_mod_xr::session::XrTrackingRoot;

#[derive(Component)]
pub struct XrMirroringCamera3d {
    eye: Eye,
}

// Creates normal Camera3d for a desktop window, which reflect XrCamera orientation.
pub struct XrCameraMirroringPlugin;

#[derive(Clone, Copy)]
pub enum Eye {
    Left = 0,   // todo value order assumed
    Right = 1,
}

impl Plugin for XrCameraMirroringPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup_camera)
            .add_systems(Update, follow_xr_camera);
    }
}

fn setup_camera(mut commands: Commands) {
    // camera
    commands.spawn((
        Camera3d::default(),
        //Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
        XrMirroringCamera3d{
//            eye: Eye::Left,
            eye: Eye::Right,
        },
    ));
}

impl PartialEq<Eye> for u32 {
    fn eq(&self, other: &Eye) -> bool {
        self == other
    }
}

fn follow_xr_camera(
    mut param_set: ParamSet<(
        Query<(&XrCamera, &Transform), With<XrCamera>>,
        Query<&Transform, With<XrTrackingRoot>>,
        Query<(&mut Transform, &XrMirroringCamera3d), (With<XrMirroringCamera3d>, Without<XrCamera>)>,
    )>,
) {
    // only one XrTrackingRoot should exist
    let xr_tracking_root_transform = param_set.p1().iter().next().cloned();

    if let Some(xr_tracking_root_transform) = xr_tracking_root_transform {
        // Collect XR camera data as copies
        let xr_camera_data: Vec<_> = param_set.p0().iter().map(|(cam, trans)| (cam.0, *trans)).collect();

        // Now iterate over mirroring cameras and update them
        for (mut mirroring_transform, mirroring_camera) in param_set.p2().iter_mut() {
            // Find matching XR camera for this eye
            if let Some((_, xr_camera_transform)) = xr_camera_data.iter().find(|(eye, _)| *eye == mirroring_camera.eye as u32) {
                *mirroring_transform = xr_tracking_root_transform * *xr_camera_transform;
            }
        }
    }
}
