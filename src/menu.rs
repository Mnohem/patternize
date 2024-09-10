use crate::loading::TextureAssets;
use crate::player::{Tool, User};
use crate::{AppState, UserState};
use bevy::input::common_conditions::input_just_pressed;
use bevy::prelude::*;

pub struct MenuPlugin;

/// This plugin is responsible for the game menu (containing only one button...)
/// The menu is only drawn during the State `GameState::Menu` and is removed when that state is exited
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Running), setup_menu)
            .add_systems(
                Update,
                (
                    sidebar_buttons,
                    check_if_in_ui.run_if(
                        input_just_pressed(MouseButton::Left)
                            .or_else(input_just_pressed(MouseButton::Right)),
                    ),
                    crate::run_state_transitions.after(check_if_in_ui),
                )
                    .run_if(in_state(AppState::Running))
                    .before(crate::CanvasSet)
                    .before(crate::UISet),
            )
            .add_systems(OnEnter(AppState::Cleanup), cleanup_menu);
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
                        ChangeTool(Tool::Table),
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
struct ChangeTool(Tool);

fn sidebar_buttons(
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
                let ChangeTool(tool) = change_tool;
                let mut user = user.single_mut();
                user.current_tool = *tool;
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

fn check_if_in_ui(
    interaction_query: Query<Entity, With<Sidebar>>,
    children: Query<&Children>,
    child_interact: Query<&Interaction>,
    mut next_state: ResMut<NextState<UserState>>,
) {
    let sidebar = interaction_query.single();

    for child in children.iter_descendants(sidebar) {
        let Ok(c_interact) = child_interact.get(child) else {
            continue;
        };
        match c_interact {
            Interaction::Pressed | Interaction::Hovered => {
                return next_state.set(UserState::Sidebar)
            }
            Interaction::None => (),
        }
    }
    next_state.set(UserState::Drawing);
}

fn cleanup_menu(mut commands: Commands, menu: Query<Entity, With<Sidebar>>) {
    for entity in menu.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
