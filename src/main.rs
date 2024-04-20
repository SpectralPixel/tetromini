use bevy::{prelude::*, window::{WindowMode, WindowResolution}};
use bevy_pixel_buffer::prelude::*;

const MAP_DIMS: PixelBufferSize = PixelBufferSize {
    size: UVec2::new(10, 20),
    pixel_size: UVec2::new(20, 20), 
};

const _ARRAY_LENGTH: usize = (MAP_DIMS.size.x * MAP_DIMS.size.y) as usize;

const UPDATE_RATE: f64 = 1.;

fn main() {
    let window_plugin = WindowPlugin {
        primary_window: Some(Window {
            title: String::from("Tetromini"),
            mode: WindowMode::Windowed,
            resolution: WindowResolution::new((MAP_DIMS.size.x * MAP_DIMS.pixel_size.x) as f32, (MAP_DIMS.size.y * MAP_DIMS.pixel_size.y) as f32),
            resizable: false,
            ..default()
        }),
        ..default()
    };

    App::new()
        .add_plugins(DefaultPlugins.set(window_plugin))
        .add_systems(Startup, setup_simulation)
        // .add_systems(
        //     PostStartup,
        //     update_simulation,
        // )
        // .add_systems(
        //     FixedUpdate,
        //     update_simulation,
        // )
        .add_systems(
            Update,
            bevy::window::close_on_esc,
        )
        .insert_resource(
            Time::<Fixed>::from_seconds(UPDATE_RATE),
        )
        .run();
}

fn setup_simulation(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    PixelBufferBuilder::new()
        .with_size(MAP_DIMS)
        .spawn(&mut commands, &mut images);
}
