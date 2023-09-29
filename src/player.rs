use std::f32::consts::PI;

use crate::lifetime::Lifetime;
use crate::ui::{DashButton, JumpButton};
use crate::virtual_joystick::VirtualJoystickEvent;
use crate::{third_person_camera::ThirdPersonCameraTarget, JoystickControllerID};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use rand::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerDash>()
            .add_systems(Startup, spawn_player)
            .add_systems(Update, (player_movement, spawn_particles_on_player_dash));
    }
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct MoveSpeed(f32);

#[derive(Event)]
pub struct PlayerDash {
    player_entity: Entity,
    dash_start_position: Vec3,
    dash_direction: Vec3,
}

#[derive(Component)]
pub struct DashSpeed(f32);

#[derive(Component)]
pub struct Jump {
    pub jump_power: f32,
}

impl Jump {
    pub fn from_jump_power(jump_power: f32) -> Self {
        Self {
            jump_power,
            ..default()
        }
    }
}

impl Default for Jump {
    fn default() -> Self {
        Self { jump_power: 1.0 }
    }
}

#[derive(Component)]
pub struct RotateSpeed(f32);

pub fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn((
            Name::new("Player"),
            PbrBundle {
                transform: Transform::from_xyz(0.0, 0.5, 0.0),
                mesh: meshes.add(Mesh::from(shape::Cube::new(1.0))),
                material: materials.add(Color::BLUE.into()),
                ..default()
            },
            ThirdPersonCameraTarget,
        ))
        .insert((
            RigidBody::Dynamic,
            LockedAxes::ROTATION_LOCKED,
            Collider::cuboid(0.5, 0.5, 0.5),
            Velocity::default(),
            ExternalForce::default(),
            ExternalImpulse::default(),
            ColliderMassProperties::Mass(1.0),
            Damping {
                linear_damping: 2.0,
                ..default()
            },
            KinematicCharacterController { ..default() },
            GravityScale(8.0),
        ))
        .insert((
            Player,
            MoveSpeed(0.1),
            DashSpeed(32.0),
            Jump::from_jump_power(32.0),
            RotateSpeed(16.0),
        ));
}

pub fn player_movement(
    mut player_query: Query<
        (
            Entity,
            &mut KinematicCharacterController,
            &mut Transform,
            &mut ExternalImpulse,
        ),
        With<Player>,
    >,
    move_speed_query: Query<&MoveSpeed>,
    dash_speed_query: Query<&DashSpeed>,
    dash_button_q: Query<&Interaction, (Changed<Interaction>, With<DashButton>)>,
    jump_query: Query<&Jump>,
    jump_button_q: Query<&Interaction, (Changed<Interaction>, With<JumpButton>)>,
    rotate_speed_query: Query<&RotateSpeed>,
    camera_query: Query<&Transform, (With<Camera3d>, Without<Player>)>,
    input: Res<Input<KeyCode>>,
    mut virtual_joystick: EventReader<VirtualJoystickEvent<JoystickControllerID>>,
    time: Res<Time>,
    rapier_ctx: Res<RapierContext>,
    mut dash_event_writer: EventWriter<PlayerDash>,
) {
    for (player_entity, mut controller, mut transform, mut impulse) in player_query.iter_mut() {
        let camera = camera_query.get_single().expect("Could not find camera");

        let mut move_input = Vec2::ZERO;
        if input.any_pressed([KeyCode::W, KeyCode::Up]) {
            move_input += Vec2::Y;
        }
        if input.any_pressed([KeyCode::A, KeyCode::Left]) {
            move_input += Vec2::NEG_X;
        }
        if input.any_pressed([KeyCode::S, KeyCode::Down]) {
            move_input += Vec2::NEG_Y;
        }
        if input.any_pressed([KeyCode::D, KeyCode::Right]) {
            move_input += Vec2::X;
        }
        for joystick in virtual_joystick.iter() {
            match joystick.id() {
                JoystickControllerID::MoveJoystick => move_input += joystick.axis(),
            }
        }
        let mut move_direction = move_input.y * camera.forward() + move_input.x * camera.right();
        move_direction.y = 0.0;
        move_direction = move_direction.normalize_or_zero();

        let move_speed = move_speed_query
            .get(player_entity)
            .unwrap_or(&MoveSpeed(1.0))
            .0;
        let move_displacement = move_speed * move_direction;
        controller.translation = Some(move_displacement);

        let mut button_requested_dash = false;
        if let Ok(dash_button_interaction) = dash_button_q.get_single() {
            if (*dash_button_interaction) == Interaction::Pressed {
                button_requested_dash = true;
            }
        }

        if input.any_just_pressed([KeyCode::ShiftLeft]) || button_requested_dash {
            if move_direction.length_squared() > 0.0 {
                dash_event_writer.send(PlayerDash {
                    player_entity: player_entity,
                    dash_start_position: transform.translation,
                    dash_direction: move_direction,
                });

                let dash_speed = dash_speed_query
                    .get(player_entity)
                    .unwrap_or(&DashSpeed(1.0))
                    .0;
                impulse.impulse += dash_speed * move_direction;
            }
        }

        let is_grounded = rapier_ctx
            .cast_shape(
                transform.translation,
                Quat::IDENTITY,
                Vec3::NEG_Y,
                &Collider::cuboid(0.5, 0.5, 0.5),
                0.5,
                QueryFilter::new().exclude_rigid_body(player_entity),
            )
            .is_some();

        let mut button_requested_jump = false;
        if let Ok(jump_button_interaction) = jump_button_q.get_single() {
            if (*jump_button_interaction) == Interaction::Pressed {
                button_requested_jump = true;
            }
        }

        if (input.any_just_pressed([KeyCode::Space]) || button_requested_jump) && is_grounded {
            let jump_power = jump_query
                .get(player_entity)
                .unwrap_or(&Jump::from_jump_power(1.0))
                .jump_power;
            impulse.impulse += jump_power * Vec3::Y;
        }

        if move_direction.length_squared() > 0.0 {
            let rotate_speed = rotate_speed_query
                .get(player_entity)
                .unwrap_or(&RotateSpeed(1.0))
                .0;
            let forward = transform.forward();
            transform.look_to(
                forward.lerp(move_direction, time.delta_seconds() * rotate_speed),
                Vec3::Y,
            );
        }
    }
}

fn spawn_particles_on_player_dash(
    mut commands: Commands,
    mut dash_event_reader: EventReader<PlayerDash>,
    dash_speed_query: Query<&DashSpeed>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut rng = rand::thread_rng();

    for dash_event in dash_event_reader.iter() {
        if let Ok(dash_speed) = dash_speed_query.get(dash_event.player_entity) {
            for _i in 0..dash_speed.0.floor() as i32 {
                commands.spawn((
                    Name::new("Player Dash Particle"),
                    PbrBundle {
                        transform: Transform::from_translation(dash_event.dash_start_position),
                        mesh: meshes.add(Mesh::from(shape::Cube::new(0.1))),
                        material: materials.add(Color::ANTIQUE_WHITE.into()),
                        ..default()
                    },
                    RigidBody::Dynamic,
                    Velocity::linear(
                        dash_speed.0
                            * (Quat::from_euler(
                                EulerRot::XYZ,
                                rng.gen::<f32>() * PI / 6.0,
                                rng.gen::<f32>() * PI / 6.0,
                                rng.gen::<f32>() * PI / 6.0,
                            ) * -dash_event.dash_direction)
                                .normalize(),
                    ),
                    ColliderMassProperties::Mass(1.0),
                    Damping {
                        linear_damping: 0.5,
                        angular_damping: 0.5,
                        ..default()
                    },
                    GravityScale(16.0),
                    Lifetime::from_seconds(1.0),
                ));
            }
        }
    }
}
