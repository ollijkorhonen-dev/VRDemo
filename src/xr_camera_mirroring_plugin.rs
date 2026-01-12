use bevy::prelude::*;
use bevy_mod_xr::camera::XrCamera;
use bevy_mod_xr::session::XrTrackingRoot;

#[derive(Component)]
pub struct XrMirroringCamera3d;

// Creates normal Camera3d for a desktop window, which reflect XrCamera orientation.
pub struct XrCameraMirroringPlugin;

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
        XrMirroringCamera3d,
    ));
}

fn follow_xr_camera(
    mut param_set: ParamSet<(
        Query<&Transform, With<XrCamera>>,
        Query<&Transform, With<XrTrackingRoot>>,
        Query<&mut Transform, (With<XrMirroringCamera3d>, Without<XrCamera>)>,
    )>,
) {
    // todo configurable for left/right/both enum
    // Get the first XR camera's transform
    let xr_camera_transform = param_set.p0().iter().next().cloned();
    // only one XrTrackingRoot should exist
    let xr_tracking_root_transform = param_set.p1().iter().next().cloned();

    if let (Some(xr_camera_transform), Some(xr_tracking_root_transform)) = (xr_camera_transform, xr_tracking_root_transform) {
        // Update all regular cameras to follow the XR camera
        for mut regular_camera_transform in param_set.p2().iter_mut() {
            *regular_camera_transform = xr_tracking_root_transform * xr_camera_transform;
        }
    }
}