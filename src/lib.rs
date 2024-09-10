#![allow(clippy::type_complexity)]

mod actions;
mod audio;
mod loading;
mod menu;
mod player;
mod table;

use actions::{Actions, ActionsPlugin};
use audio::InternalAudioPlugin;
use loading::LoadingPlugin;
use menu::MenuPlugin;
use player::{Tool, User, UserPlugin};
use table::TablePlugin;

use bevy::app::App;
#[cfg(debug_assertions)]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::ecs::system::SystemParam;
use bevy::prelude::*;

// This example game uses States to separate logic
// See https://bevy-cheatbook.github.io/programming/states.html
// Or https://github.com/bevyengine/bevy/blob/main/examples/ecs/state.rs
#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
enum AppState {
    // During the loading State the LoadingPlugin will load our assets
    #[default]
    Loading,
    // During this State the actual app logic is executed
    Running,
    Cleanup,
}
#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
pub enum UserState {
    #[default]
    Inactive,
    // User cursor is hovering over the Sidebar
    Sidebar,
    // Here the Sidebar is drawn and waiting for player interaction
    // User is using the canvas
    Drawing,
}

/// Systems in this set run when the user is interacting with the canvas
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct CanvasSet;
/// Systems in this set run when the user is possibly finishing an action
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct WhenActionDoneSet;
/// Systems in this set run when the user is interacting with the UI
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct UISet;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppState>()
            .init_state::<UserState>()
            .add_plugins((
                LoadingPlugin,
                MenuPlugin,
                ActionsPlugin,
                InternalAudioPlugin,
                UserPlugin,
                TablePlugin,
            ))
            .add_systems(OnEnter(AppState::Running), canvas_start)
            .configure_sets(
                Update,
                (
                    UISet.run_if(not(in_state(UserState::Drawing))),
                    CanvasSet.run_if(in_state(UserState::Drawing)),
                    WhenActionDoneSet
                        .run_if(in_state(UserState::Drawing))
                        .run_if(mouse_just_released),
                )
                    .chain()
                    .run_if(in_state(AppState::Running)),
            );

        #[cfg(debug_assertions)]
        {
            app.add_plugins((
                    // FrameTimeDiagnosticsPlugin, 
                    LogDiagnosticsPlugin::default(),
                ));
        }
    }
}

#[derive(SystemParam)]
pub struct MousePosQueries<'w, 's> {
    windows: Query<'w, 's, &'static Window>,
    camera_q: Query<'w, 's, (&'static Camera, &'static GlobalTransform)>,
}
impl MousePosQueries<'_, '_> {
    pub fn mouse_pos(&self) -> Vec2 {
        // check if the cursor is inside the window and get its position
        // then, ask bevy to convert into world coordinates, and truncate to discard Z
        self.windows
            .single()
            .cursor_position()
            .and_then(|cursor| {
                let (camera, camera_transform) = self.camera_q.single();
                camera.viewport_to_world(camera_transform, cursor)
            })
            .map(|ray| ray.origin.truncate())
            .unwrap_or(Vec2::NAN)
    }
}

pub fn canvas_start(mut next_user_state: ResMut<NextState<UserState>>) {
    next_user_state.set(UserState::Drawing);
}

pub fn performing_actions(actions: Res<Actions>) -> bool {
    actions.button_push.is_some()
}

pub fn mouse_just_released(input: Res<ButtonInput<MouseButton>>) -> bool {
    input.any_just_released([MouseButton::Left, MouseButton::Right, MouseButton::Middle])
}

pub fn using_tool(tool: Tool) -> impl FnMut(Query<&User>, Res<Actions>) -> bool {
    move |user, actions| performing_actions(actions) && user.single().current_tool == tool
}

pub fn run_state_transitions(world: &mut World) {
    let _ = world.try_run_schedule(StateTransition);
}
