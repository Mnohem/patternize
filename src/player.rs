use crate::AppState;
use bevy::prelude::*;

pub struct UserPlugin;

/// Scaling used for Table Cells
#[derive(Clone, Debug)]
pub struct UserConfig {
    pub cell_dimensions: Vec2,
    pub table_text_color: Color,
    pub table_bg_color: Handle<ColorMaterial>,
    pub cell_mesh: Handle<Mesh>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tool {
    Table,
}
#[derive(Component, Debug)]
pub struct User {
    pub current_tool: Tool,
    pub current_config: UserConfig,
}

/// This plugin handles user related stuff like movement
/// User logic is only active during the State `GameState::Playing`
impl Plugin for UserPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Running), spawn_user);
    }
}

fn spawn_user(
    mut commands: Commands,
    mut color_assets: ResMut<Assets<ColorMaterial>>,
    mut mesh_assets: ResMut<Assets<Mesh>>,
) {
    let bg_material = ColorMaterial {
        color: Color::BLACK,
        ..Default::default()
    };
    let bg_handle = color_assets.add(bg_material);

    let cell_mesh = mesh_assets.add(Rectangle::new(1.0, 1.0));

    commands.spawn(User {
        current_tool: Tool::Table,
        current_config: UserConfig {
            cell_dimensions: Vec2::splat(20.0),
            table_text_color: Color::WHITE,
            table_bg_color: bg_handle,
            cell_mesh,
        },
    });
}
