#![allow(clippy::type_complexity)]

mod actions;
mod audio;
mod loading;
mod menu;
mod player;

use actions::{Actions, ActionsPlugin};
use audio::InternalAudioPlugin;
use loading::LoadingPlugin;
use menu::MenuPlugin;
use player::UserPlugin;

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
/// Systems in this set run when the user is interacting with the canvas and performing some action
/// on it
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct ActionSet;
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
            ))
            .add_systems(OnEnter(AppState::Running), canvas_start)
            .configure_sets(
                Update,
                (
                    UISet
                        .run_if(in_state(AppState::Running))
                        .run_if(not(in_state(UserState::Drawing)))
                        .before(CanvasSet),
                    CanvasSet
                        .run_if(in_state(AppState::Running))
                        .run_if(in_state(UserState::Drawing))
                        .after(UISet),
                    ActionSet.in_set(CanvasSet).run_if(performing_actions),
                ),
            );

        #[cfg(debug_assertions)]
        {
            app.add_plugins((FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin::default()));
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
    actions.tool_button_push.is_some()
}

pub fn run_state_transitions(world: &mut World) {
    let _ = world.try_run_schedule(StateTransition);
}
