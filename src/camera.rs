use crate::{third_person_camera::*, virtual_joystick::VirtualJoystickEvent, JoystickControllerID};
use bevy::{core_pipeline::bloom::BloomSettings, prelude::*};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera).add_systems(
            Update,
            (
                toggle_zoom_on_cursor_lock,
                disable_camera_orbit_while_joystick_held,
            ),
        );
    }
}

#[derive(Component)]
pub struct MainCamera;

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Name::new("Camera"),
        Camera3dBundle {
            transform: Transform::from_xyz(16.0, 16.0, 16.0).looking_at(Vec3::ZERO, Vec3::Y),
            camera: Camera {
                hdr: true,
                ..default()
            },
            ..default()
        },
        MainCamera,
        ThirdPersonCamera {
            zoom: Zoom::new(4.0, 16.0),
            cursor_lock_key: KeyCode::Grave,
            ..default()
        },
        BloomSettings {
            intensity: 0.01,
            ..default()
        },
    ));
}

pub fn toggle_zoom_on_cursor_lock(
    input: Res<Input<KeyCode>>,
    mut camera_query: Query<&mut ThirdPersonCamera, With<MainCamera>>,
) {
    let mut camera = camera_query.get_single_mut().unwrap();
    if input.just_pressed(camera.cursor_lock_key) {
        camera.zoom_enabled = !camera.zoom_enabled;
    }
}

pub fn disable_camera_orbit_while_joystick_held(
    mut virtual_joystick: EventReader<VirtualJoystickEvent<JoystickControllerID>>,
    mut camera_query: Query<&mut ThirdPersonCamera, With<MainCamera>>,
) {
    let mut camera = camera_query.get_single_mut().unwrap();
    for joystick in virtual_joystick.iter() {
        match joystick.id() {
            JoystickControllerID::MoveJoystick => {
                camera.orbit_enabled = false;
                return;
            }
        }
    }
    camera.orbit_enabled = true;
}
