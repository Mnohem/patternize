use std::f32::consts::FRAC_PI_2;

use crate::actions::{finish_actions, maintain_actions, Actions, Preview};
use crate::player::{Tool, User, UserConfig};
use crate::{using_tool, CanvasSet, WhenActionDoneSet};
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;

pub struct TablePlugin;

/// This plugin is responsible for dealing with and constructing tables
impl Plugin for TablePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                (make_table, table_outline)
                    .in_set(CanvasSet)
                    .run_if(using_tool(Tool::Table))
                    .after(maintain_actions),
                cleanup_empty_tables
                    .in_set(WhenActionDoneSet)
                    .before(finish_actions),
            ),
        );
    }
}

// Table position is top left of table
#[derive(Component)]
pub struct TableHead {
    num_rows: u32,
    num_columns: u32,
    cell_heights: Vec<f32>,
    cell_widths: Vec<f32>,
}
impl TableHead {
    pub fn with_transform(t: Transform) -> (Self, Preview, SpatialBundle) {
        (
            Self {
                num_rows: 0,
                num_columns: 0,
                cell_heights: vec![],
                cell_widths: vec![],
            },
            Preview,
            SpatialBundle::from_transform(t),
        )
    }
}

#[derive(Component)]
pub struct Cell;
impl Cell {
    pub fn new(
        self,
        transform: Transform,
        mesh: Handle<Mesh>,
        text: String,
        text_color: Color,
        bg_color: Handle<ColorMaterial>,
    ) -> (Self, MaterialMesh2dBundle<ColorMaterial>) {
        (
            self,
            MaterialMesh2dBundle {
                material: bg_color,
                transform,
                mesh: mesh.into(),
                ..Default::default()
            },
        )
    }
}

pub fn make_table(
    mut cmd: Commands,
    mut table_head_q: Query<(Entity, &mut TableHead), With<Preview>>,
    user_q: Query<&User>,
    children_q: Query<&Children>,
    transform_q: Query<&Transform>,
    actions: Res<Actions>,
) {
    let anchor = actions.from;
    let user = user_q.single();
    let scale = user.current_config.cell_dimensions;

    let Ok((id, mut table_head_mut)) = table_head_q.get_single_mut() else {
        let table_head_bundle =
            TableHead::with_transform(Transform::from_translation(anchor.extend(0.0)));
        cmd.spawn(table_head_bundle);
        return;
    };

    let action_dimensions = actions.to - anchor;
    let num_rows = (-action_dimensions.y / scale.y).floor() as u32;
    let num_columns = (action_dimensions.x / scale.x).floor() as u32;
    let prev_num_rows = table_head_mut.num_rows;
    let prev_num_columns = table_head_mut.num_columns;

    table_head_mut.num_rows = num_rows;
    table_head_mut.num_columns = num_columns;

    // Making new cells
    // This will iterate over all new cells by coordinates, avoids creating dupes
    for (row, column) in (prev_num_rows..num_rows)
        .flat_map(|x| std::iter::repeat(x).zip(0..prev_num_columns))
        .chain(
            (0..prev_num_rows)
                .flat_map(|x| std::iter::repeat(x).zip(prev_num_columns..num_columns)),
        )
        .chain(
            (prev_num_rows..num_rows)
                .flat_map(|x| std::iter::repeat(x).zip(prev_num_columns..num_columns)),
        )
    {
        let cell_width = scale.x;
        let cell_height = scale.y;
        // TODO
        // if cell dimensions can change while making table we have to change this to sum previous
        // dimensions
        let x_offset = cell_width / 2.0 + cell_width * column as f32;
        let y_offset = -(cell_height / 2.0 + cell_width * row as f32);

        let tform = Transform {
            translation: Vec3::new(x_offset, y_offset, 0.0),
            scale: scale.extend(1.0),
            ..default()
        };

        // Spawn as child of table_head
        cmd.entity(id).with_children(|c_cmd| {
            c_cmd.spawn(Cell::new(
                Cell,
                tform,
                user.current_config.cell_mesh.clone(),
                "".to_owned(),
                user.current_config.table_text_color,
                user.current_config.table_bg_color.clone(),
            ));
        });
    }

    // Early return, no need to delete if we have the same or more cells
    if prev_num_rows <= num_rows && prev_num_columns <= num_columns {
        return;
    }

    // TODO
    // if cell dimensions can change while making table we have to change this to sum previous dimensions
    let max_x_offset = scale.x * num_columns as f32;
    let max_y_offset = scale.y * num_rows as f32;
    // Deleting cells
    // This will iterate over all cells to be deleted by coordinates
    for child in children_q.iter_descendants(id) {
        let Ok(Transform { translation, .. }) = transform_q.get(child) else {
            unreachable!()
        };
        if translation.x >= max_x_offset || -translation.y >= max_y_offset {
            cmd.entity(id).remove_children(&[child]);
            cmd.entity(child).despawn();
        }
    }
}

fn table_outline(mut gizmos: Gizmos, action: Res<Actions>, user: Query<&User>) {
    let &User {
        current_config: UserConfig {
            cell_dimensions, ..
        },
        ..
    } = user.single();

    let action_dimensions = action.to - action.from;
    let cell_count = UVec2 {
        y: (action_dimensions.x / cell_dimensions.x).floor() as u32,
        x: (-action_dimensions.y / cell_dimensions.y).floor() as u32,
    };

    gizmos
        .grid_2d(
            (2.0 * action.from.rotate(Vec2::from_angle(FRAC_PI_2))
                + (cell_count.yx().as_vec2() * cell_dimensions).yx())
                / 2.0,
            -FRAC_PI_2,
            cell_count,
            cell_dimensions,
            Color::srgb(0.7, 0.3, 0.3),
        )
        .outer_edges();
}

pub fn cleanup_empty_tables(
    mut cmd: Commands,
    table_head: Query<(Entity, &TableHead), With<Preview>>,
) {
    let Ok((id, table_head)) = table_head.get_single() else {
        return;
    };

    if table_head.num_rows == 0 || table_head.num_columns == 0 {
        cmd.entity(id).despawn_recursive();
    }
}
