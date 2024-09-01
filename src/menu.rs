use crate::loading::TextureAssets;
use crate::player::{Tool, ToolConfig, User};
use crate::GameState;
use bevy::prelude::*;

pub struct MenuPlugin;

/// This plugin is responsible for the game menu (containing only one button...)
/// The menu is only drawn during the State `GameState::Menu` and is removed when that state is exited
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Drawing), setup_menu)
            .add_systems(
                Update,
                click_play_button.run_if(in_state(GameState::Drawing)),
            )
            .add_systems(OnExit(GameState::Drawing), cleanup_menu);
    }
}

#[derive(Component)]
struct ButtonColors {
    normal: Color,
    hovered: Color,
}

impl Default for ButtonColors {
    fn default() -> Self {
        ButtonColors {
            normal: Color::linear_rgb(0.15, 0.15, 0.15),
            hovered: Color::linear_rgb(0.25, 0.25, 0.25),
        }
    }
}

#[derive(Component)]
struct Sidebar;

fn setup_menu(mut commands: Commands, textures: Res<TextureAssets>) {
    info!("menu");
    commands.spawn(Camera2dBundle::default());
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::SpaceAround,
                    top: Val::Px(5.),
                    width: Val::Percent(100.),
                    position_type: PositionType::Absolute,
                    display: Display::Grid,
                    ..default()
                },
                ..default()
            },
            Sidebar,
        ))
        .with_children(|children| {
            for _ in 0..3 {
                children
                    .spawn((
                        ButtonBundle {
                            style: Style {
                                width: Val::Px(30.0),
                                height: Val::Px(30.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                padding: UiRect::all(Val::Px(5.)),
                                ..Default::default()
                            },
                            background_color: Color::NONE.into(),
                            ..Default::default()
                        },
                        ButtonColors::default(),
                        ChangeTool(ToolConfig { scale: 20.0 }, Tool::Sheet),
                    ))
                    .with_children(|parent| {
                        parent.spawn(ImageBundle {
                            image: textures.bevy.clone().into(),
                            style: Style {
                                width: Val::Px(30.),
                                ..default()
                            },
                            ..default()
                        });
                    });
            }
        });
}

#[derive(Component)]
struct ChangeTool(ToolConfig, Tool);

fn click_play_button(
    mut user: Query<&mut User>,
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &ButtonColors,
            &ChangeTool,
        ),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, button_colors, change_tool) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                let ChangeTool(config, tool) = change_tool;
            }
            Interaction::Hovered => {
                *color = button_colors.hovered.into();
            }
            Interaction::None => {
                *color = button_colors.normal.into();
            }
        }
    }
}

fn cleanup_menu(mut commands: Commands, menu: Query<Entity, With<Sidebar>>) {
    for entity in menu.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
