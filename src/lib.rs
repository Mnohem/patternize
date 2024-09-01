#![allow(clippy::type_complexity)]

mod actions;
mod audio;
mod loading;
mod menu;
mod player;

use actions::ActionsPlugin;
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
enum GameState {
    // During the loading State the LoadingPlugin will load our assets
    #[default]
    Loading,
    // During this State the actual game logic is executed
    // Here the Sidebar is drawn and waiting for player interaction
    Drawing,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>().add_plugins((
            LoadingPlugin,
            MenuPlugin,
            ActionsPlugin,
            InternalAudioPlugin,
            UserPlugin,
        ));

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
