use bevy::{prelude::*, render::pass::ClearColor};
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use grid::*;
use rand::prelude::*;
pub mod helpers;


struct GridPosition (usize, usize);


struct CellGrid(Grid<i32>);

struct Materials {
    alive_material: Handle<ColorMaterial>,
}

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: "I am a window!".to_string(),
            width: 500.,
            height: 500.,
            vsync: true,
            resizable: false,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_startup_system(setup.system())
        .add_startup_stage("game_setup", SystemStage::single(spawn_cells.system()))
        .add_system(grid_system.system().label("grid-buildup"))
        .add_system(set_cell_system.system().after("grid-buildup"))
        .add_system(bevy::input::system::exit_on_esc_system.system())
        .run();
}

fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

    commands.insert_resource(Materials {
        alive_material: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
    });
}

fn spawn_cells(
    window_settings: Res<WindowDescriptor>,
    mut commands: Commands,
    materials: Res<Materials>,
) {
    let mut rng = thread_rng();
    let height = window_settings.height;
    let width = window_settings.width;
    let cell_size = Vec2::new(2.0, 2.0);
    let padding = Vec3::new(cell_size.x / 2., cell_size.y / 2., 0.);
    let cells_offset = Vec3::new(
        -width / 2. + cell_size.x / 2.,
        -height / 2. + cell_size.y / 2.,
        0.,
    );
    let cell_spacing = 2.;

    let cell_rows = (((height - (2. * padding.y)) / (cell_size.y + cell_spacing)) + 1.) as i32;
    let cell_cols = (((width - (2. * padding.x)) / (cell_size.x + cell_spacing)) + 1.) as i32;

    let mut cells = Grid::new(cell_rows as usize, cell_cols as usize);

    for row in 0..cell_rows {
        let y_position = row as f32 * (cell_size.y + cell_spacing);

        for col in 0..cell_cols {
            // make black border with dead cells so we dont have to check the bounds of grid
            if (col < 1 || col > cell_cols - 2) || (row < 1 || row > cell_rows - 2) {
                cells[row as usize][col as usize] = 0;
            } else {
                cells[row as usize][col as usize] = rng.gen_range(0..=1);
            }

            let cell_position =
                Vec3::new(col as f32 * (cell_size.x + cell_spacing), y_position, 0.0)
                    + cells_offset
                    + padding;
            commands
                .spawn_bundle(SpriteBundle {
                    material: materials.alive_material.clone(),
                    sprite: Sprite::new(cell_size),
                    transform: Transform::from_translation(cell_position),
                    ..Default::default()
                })
                .insert(GridPosition(col as usize, row as usize));
        }
    }

    commands.insert_resource(CellGrid(cells.clone()));
}

fn set_cell_system(cell_grid: Res<CellGrid>, mut query: Query<(&GridPosition, &mut Visible)>) {
    for (grid_pos, mut visible) in &mut query.iter_mut() {
        let GridPosition(x, y) = *grid_pos;
        let col = cell_grid.0[y][x] as f32;
        visible.is_visible = col == 1.;
    }
}

fn grid_system(mut cell_grid: ResMut<CellGrid>) {
    let mut new_grid = cell_grid.0.clone();

    for row in 1..new_grid.rows() as i32 - 1 {
        for col in 1..new_grid.cols() as i32 - 1 {
            let neighbours = helpers::count_neighbours(&cell_grid.0, col, row);
            if neighbours == 3 {
                new_grid[row as usize][col as usize] = 1;
            } else if cell_grid.0[row as usize][col as usize] == 1 {
                if neighbours < 2 || neighbours > 3 {
                    new_grid[row as usize][col as usize] = 0;
                }
            }
        }
    }

    cell_grid.0 = new_grid.clone();
}
