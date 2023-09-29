use crate::ThirdPersonCamera;
use bevy::{prelude::*, window::PrimaryWindow};
use std::f32::consts::PI;

pub struct TouchPlugin;

impl Plugin for TouchPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (orbit_touch).chain());
    }
}

pub fn orbit_touch(
    window_q: Query<&Window, With<PrimaryWindow>>,
    mut cam_q: Query<(&ThirdPersonCamera, &mut Transform), With<ThirdPersonCamera>>,
    touches: Res<Touches>,
) {
    let mut rotation = Vec2::ZERO;
    if let Some(touch) = touches.iter().next() {
        rotation = touch.position() - touch.start_position();
    }

    let Ok((cam, mut cam_transform)) = cam_q.get_single_mut() else {
        return;
    };

    if !cam.orbit_enabled {
        return;
    }

    rotation *= cam.touch_settings.touch_sensitivity;

    if rotation.length_squared() > 0.0 {
        let window = window_q.get_single().unwrap();
        let delta_x = {
            let delta = rotation.x / window.width() * PI;
            delta
        };

        let delta_y = rotation.y / window.height() * PI;
        let yaw = Quat::from_rotation_y(-delta_x);
        let pitch = Quat::from_rotation_x(-delta_y);
        cam_transform.rotation = yaw * cam_transform.rotation; // rotate around global y axis

        // Calculate the new rotation without applying it to the camera yet
        let new_rotation = cam_transform.rotation * pitch;

        // check if new rotation will cause camera to go beyond the 180 degree vertical bounds
        let up_vector = new_rotation * Vec3::Y;
        if up_vector.y > 0.0 {
            cam_transform.rotation = new_rotation;
        }
    }

    let rot_matrix = Mat3::from_quat(cam_transform.rotation);
    cam_transform.translation =
        cam.focus + rot_matrix.mul_vec3(Vec3::new(0.0, 0.0, cam.zoom.radius));
}
