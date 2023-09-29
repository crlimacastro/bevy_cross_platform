pub mod camera;
pub mod lifetime;
pub mod player;
pub mod third_person_camera;
pub mod ui;
pub mod virtual_joystick;
pub mod world;

use bevy::{
    asset::ChangeWatcher,
    input::common_conditions::input_toggle_active,
    prelude::*,
    window::{close_on_esc, CursorGrabMode, PresentMode, PrimaryWindow, WindowFocused, WindowMode},
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier3d::prelude::*;
use camera::*;
use lifetime::*;
use player::*;
use std::time::Duration;
use third_person_camera::*;
use ui::*;
use virtual_joystick::*;
use wasm_bindgen::prelude::*;
use world::*;

#[derive(Default, Reflect, Hash, Clone, PartialEq, Eq)]
pub enum JoystickControllerID {
    #[default]
    MoveJoystick,
}

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    App::new()
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "3D Example".into(),
                        canvas: Some("#bevy-app".into()),
                        resizable: false,
                        mode: WindowMode::BorderlessFullscreen,
                        present_mode: PresentMode::AutoNoVsync,
                        fit_canvas_to_parent: true,
                        ..default()
                    }),
                    ..default()
                })
                .set(AssetPlugin {
                    watch_for_changes: ChangeWatcher::with_delay(Duration::from_secs_f32(0.5)),
                    ..default()
                }),
            WorldInspectorPlugin::new().run_if(input_toggle_active(false, KeyCode::Grave)),
            RapierPhysicsPlugin::<NoUserData>::default(),
            // RapierDebugRenderPlugin::default(),
            ThirdPersonCameraPlugin,
            VirtualJoystickPlugin::<JoystickControllerID>::default(),
        ))
        .add_plugins((
            LifetimePlugin,
            CameraPlugin,
            WorldPlugin,
            PlayerPlugin,
            UIPlugin,
        ))
        .add_systems(Startup, lock_and_hide_cursor)
        .add_systems(
            Update,
            (close_on_esc, lock_and_hide_cursor_on_window_focused),
        )
        .run();

    Ok(())
}

pub fn lock_and_hide_cursor(mut window_query: Query<&mut Window, With<PrimaryWindow>>) {
    let mut window = window_query.get_single_mut().unwrap();
    window.cursor.grab_mode = CursorGrabMode::Locked;
    window.cursor.visible = false;
}

pub fn lock_and_hide_cursor_on_window_focused(
    mut window_query: Query<&mut Window, With<PrimaryWindow>>,
    mut reader: EventReader<WindowFocused>,
) {
    for event in reader.iter() {
        if event.focused {
            if let Ok(mut window) = window_query.get_mut(event.window) {
                window.cursor.grab_mode = CursorGrabMode::Locked;
                window.cursor.visible = false;
            }
        }
    }
}
