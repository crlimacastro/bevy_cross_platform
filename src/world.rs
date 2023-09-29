use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_world, spawn_lighting));
    }
}

fn spawn_lighting(mut commands: Commands) {
    commands.spawn((
        DirectionalLightBundle {
            directional_light: DirectionalLight {
                shadows_enabled: true,
                ..default()
            },
            transform: Transform::from_rotation(Quat::from_euler(
                EulerRot::XYZ,
                -45.0_f32.to_radians(),
                0.0,
                0.0,
            )),
            ..default()
        },
        Name::new("Directional Light"),
    ));
}

fn spawn_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Name::new("Floor"),
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane::from_size(256.0))),
            material: materials.add(Color::SEA_GREEN.into()),
            ..default()
        },
        Collider::halfspace(Vec3::Y).unwrap(),
        RigidBody::Fixed,
    ));

    let mut create_platform =
        |translation: Vec3, scale: Vec3| -> (Name, PbrBundle, Collider, RigidBody) {
            (
                Name::new("Platform"),
                PbrBundle {
                    transform: Transform::from_translation(translation),
                    mesh: meshes.add(Mesh::from(shape::Box::new(scale.x, scale.y, scale.z))),
                    material: materials.add(Color::YELLOW_GREEN.into()),
                    ..default()
                },
                Collider::cuboid(scale.x / 2.0, scale.y / 2.0, scale.z / 2.0),
                RigidBody::Fixed,
            )
        };

    commands.spawn(create_platform(
        Vec3::new(4.0, 2.0, -8.0),
        Vec3::new(4.0, 1.0, 4.0),
    ));
    commands.spawn(create_platform(
        Vec3::new(16.0, 4.0, -8.0),
        Vec3::new(4.0, 1.0, 4.0),
    ));
    commands.spawn(create_platform(
        Vec3::new(32.0, 6.0, -8.0),
        Vec3::new(4.0, 1.0, 4.0),
    ));
}
