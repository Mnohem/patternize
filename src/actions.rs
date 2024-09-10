use std::f32::consts::FRAC_PI_2;

use bevy::prelude::*;

use crate::{
    performing_actions,
    player::{User, UserConfig},
    CanvasSet, UserState, WhenActionDoneSet,
};

/// Represents an action on the canvas
#[derive(Default, Resource)]
pub struct Actions {
    pub button_push: Option<MouseButton>,
    pub from: Vec2,
    pub to: Vec2,
}

/// Tag for Components that are currently being made with a tool, but are displayed as a preview
#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct Preview;

pub struct ActionsPlugin;
// This plugin listens for keyboard input and converts the input into Actions
// Actions can then be used as a resource in other systems to act on the player input.
impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Actions>()
            .add_systems(
                Update,
                (
                    maintain_actions.in_set(CanvasSet),
                    actions_outline
                        .in_set(CanvasSet)
                        .run_if(performing_actions)
                        .after(maintain_actions),
                    finish_actions.in_set(WhenActionDoneSet),
                ),
            )
            .add_systems(OnExit(UserState::Drawing), clear_actions);
    }
}

pub fn actions_outline(mut gizmos: Gizmos, action: Res<Actions>) {
    let action_dimensions = action.to - action.from;

    gizmos.rect_2d(
        action.from + action_dimensions / 2.0,
        0.0,
        action_dimensions,
        Color::srgb(0.7, 0.3, 0.3),
    );
}

pub fn maintain_actions(
    mouse_q: crate::MousePosQueries,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mut action: ResMut<Actions>,
) {
    match (&mut *action, mouse_q.mouse_pos()) {
        (_, m) if m.is_nan() => (),
        (Actions {
            button_push: Some(b),
            to,
            ..
        }, m) if mouse_buttons.pressed(*b) => *to = m,
        (Actions {
            button_push: button,
            from,
            to,
        }, m) => {
            *from = m;
            *to = *from;
            *button = mouse_buttons.get_pressed().next().copied();
        }
    }
}

pub fn clear_actions(
    mut cmd: Commands,
    mut action: ResMut<Actions>,
    previews: Query<Entity, With<Preview>>,
) {
    action.button_push = None;
    action.from = Vec2::ZERO;
    action.to = Vec2::ZERO;
    for entity in &previews {
        cmd.entity(entity).despawn_recursive();
    }
}

// This system should be the last run in WhenActionsDoneSet
pub fn finish_actions(
    mut cmd: Commands,
    mut action: ResMut<Actions>,
    previews: Query<Entity, With<Preview>>,
) {
    action.button_push = None;
    action.from = Vec2::ZERO;
    action.to = Vec2::ZERO;
    for entity in &previews {
        cmd.entity(entity).remove::<Preview>();
    }
}
