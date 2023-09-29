use crate::virtual_joystick::*;
use crate::JoystickControllerID;
use bevy::prelude::*;
use bevy::ui::FocusPolicy;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, create_ui)
            .add_systems(Update, (toggle_ui_on_tab, show_ui_on_any_touch));
    }
}

#[derive(Component)]
pub struct JumpButton;

#[derive(Component)]
pub struct DashButton;

fn create_ui(mut cmd: Commands, asset_server: Res<AssetServer>) {
    cmd.spawn((
        Name::new("Virtual Joystick"),
        VirtualJoystickBundle::new(VirtualJoystickNode {
            border_image: asset_server.load("Outline.png"),
            knob_image: asset_server.load("Knob.png"),
            knob_size: Vec2::new(80., 80.),
            dead_zone: 80.,
            id: JoystickControllerID::MoveJoystick,
            axis: VirtualJoystickAxis::Both,
            behaviour: VirtualJoystickType::Fixed,
        })
        .set_visibility(Visibility::Hidden)
        .set_focus_policy(FocusPolicy::Block)
        .set_color(TintColor(Color::WHITE))
        .set_style(Style {
            width: Val::Px(150.),
            height: Val::Px(150.),
            position_type: PositionType::Absolute,
            left: Val::Percent(15.),
            bottom: Val::Percent(50.),
            ..default()
        }),
        VirtualJoystickInteractionArea,
    ));
    cmd.spawn((
        Name::new("Jump Button"),
        ButtonBundle {
            style: Style {
                width: Val::Px(150.),
                height: Val::Px(50.),
                position_type: PositionType::Absolute,
                right: Val::Percent(15.),
                bottom: Val::Percent(60.),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: Color::DARK_GRAY.into(),
            visibility: Visibility::Hidden,
            ..default()
        },
        JumpButton,
    ))
    .remove::<Button>()
    .with_children(|parent| {
        parent
            .spawn((TextBundle::from_section(
                "Jump",
                TextStyle {
                    font_size: 32.,
                    ..default()
                },
            )
            .with_style(Style {
                width: Val::Auto,
                ..default()
            })
            .with_text_alignment(TextAlignment::Center),))
            .insert(Visibility::Hidden);
    });
    cmd.spawn((
        Name::new("Dash Button"),
        ButtonBundle {
            style: Style {
                width: Val::Px(150.),
                height: Val::Px(50.),
                position_type: PositionType::Absolute,
                right: Val::Percent(15.),
                bottom: Val::Percent(50.),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: Color::DARK_GRAY.into(),
            visibility: Visibility::Hidden,
            ..default()
        },
        DashButton,
    ))
    .remove::<Button>()
    .with_children(|parent| {
        parent
            .spawn((TextBundle::from_section(
                "Dash",
                TextStyle {
                    font_size: 32.,
                    ..default()
                },
            )
            .with_style(Style {
                width: Val::Auto,
                ..default()
            })
            .with_text_alignment(TextAlignment::Center),))
            .insert(Visibility::Hidden);
    });
}

fn toggle_ui_on_tab(
    mut commands: Commands,
    input: Res<Input<KeyCode>>,
    mut node_q: Query<(Entity, &mut Visibility), With<Node>>,
    mut joystick_q: Query<&mut VirtualJoystickNode<JoystickControllerID>>,
    button_q: Query<&Button>,
    control_button_q: Query<Or<(With<JumpButton>, With<DashButton>)>>,
) {
    if input.any_just_pressed([KeyCode::Tab]) {
        for (entity, mut visibility) in node_q.iter_mut() {
            (*visibility) = if (*visibility) == Visibility::Inherited {
                Visibility::Hidden
            } else {
                Visibility::Inherited
            };

            if let Ok(mut joystick) = joystick_q.get_mut(entity) {
                joystick.dead_zone = if joystick.dead_zone == 0. { 80. } else { 0. };
            }

            if control_button_q.contains(entity) {
                if button_q.contains(entity) {
                    commands.entity(entity).remove::<Button>();
                } else {
                    commands.entity(entity).insert(Button);
                }
            }
        }
    }
}

fn show_ui_on_any_touch(
    mut commands: Commands,
    touches: Res<Touches>,
    mut node_q: Query<(Entity, &mut Visibility), With<Node>>,
    mut joystick_q: Query<&mut VirtualJoystickNode<JoystickControllerID>>,
    control_button_q: Query<Or<(With<JumpButton>, With<DashButton>)>>,
) {
    if touches.any_just_pressed() {
        for (entity, mut visibility) in node_q.iter_mut() {
            (*visibility) = Visibility::Inherited;

            if let Ok(mut joystick) = joystick_q.get_mut(entity) {
                joystick.dead_zone = 0.;
            }

            if control_button_q.contains(entity) {
                commands.entity(entity).insert(Button);
            }
        }
    }
}
