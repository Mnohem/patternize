use crate::actions::Actions;
use crate::GameState;
use bevy::prelude::*;

pub struct UserPlugin;

#[derive(Clone, Debug)]
pub struct ToolConfig {
    pub scale: f32,
}
#[derive(Debug)]
pub enum Tool {
    Sheet,
}
#[derive(Component, Debug)]
pub struct User {
    pub current_tool: Tool,
    pub current_config: ToolConfig,
}

/// This plugin handles user related stuff like movement
/// User logic is only active during the State `GameState::Playing`
impl Plugin for UserPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Drawing), spawn_user);
    }
}

fn spawn_user(mut commands: Commands) {
    commands.spawn(User {
        current_tool: Tool::Sheet,
        current_config: ToolConfig { scale: 20.0 },
    });
}
