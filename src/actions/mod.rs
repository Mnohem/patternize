use bevy::{input::common_conditions::*, prelude::*};

use crate::GameState;

pub struct ActionsPlugin;

// This plugin listens for keyboard input and converts the input into Actions
// Actions can then be used as a resource in other systems to act on the player input.
impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Actions>().add_systems(
            Update,
            (maintain_actions, actions_effect)
                .chain()
                .run_if(in_state(GameState::Drawing)),
        );
    }
}

#[derive(Default, Resource)]
pub struct Actions {
    pub tool_button_push: Option<MouseButton>,
    from: Vec2,
    to: Vec2,
}

pub fn actions_effect(action: Res<Actions>, q_user: Query<&crate::player::User>) {
    if let Actions {
        tool_button_push: Some(button),
        from,
        to,
    } = *action
    {
        let user = q_user.single();
        info!("{user:?} with tool being used with {button:?} spanning {from} to {to}");
    }
}

pub fn maintain_actions(
    mouse_q: crate::MousePosQueries,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mut action: ResMut<Actions>,
) {
    match &mut *action {
        Actions {
            tool_button_push: Some(b),
            to,
            ..
        } if mouse_buttons.pressed(*b) => *to = mouse_q.mouse_pos(),
        Actions {
            tool_button_push: button,
            from,
            ..
        } => {
            *from = mouse_q.mouse_pos();
            *button = mouse_buttons.get_pressed().next().copied();
        }
    }
}
