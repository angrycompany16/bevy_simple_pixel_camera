
use bevy::{
    prelude::*,
    render::view::RenderLayers,
    math::*,
    window::*
};

pub mod pixel_camera;

use pixel_camera::*;
use board::*;

fn main() {
    App::new()
        .add_plugins(DefaulPlugins).set(
            ImagePlugin::default_nearest()
        )
        .add_plugins(PixelCameraPlugin {
                pixel_scale_factor: 4,
                screen_width: 1200,
                screen_height: 800,
                render_layer_index: 1
        })
        .add_plugins(WorldInspectorPlugin::new())
        .insert_resource(Msaa::Off)
        .add_systems(Startup, setup)
        .add_systems(Update, rotate)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    render_tex_layer: Res<RenderTexLayer>
) {
    // Renders the texture 
    commands.spawn((
        Camera2dBundle::default(),
        RenderLayers::layer(**render_tex_layer)
    ));

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("png/test-tile-bg.png"),
            transform: Transform { 
                translation: vec3(0.0, 0.0, 0.0), 
                rotation: Quat::from_euler(EulerRot::XYZ, 0.0, 0.0, 0.0), 
                scale: vec3(1.0, 1.0, 1.0) 
            },
            ..default()
        },
        Square
    ));

    commands.spawn(
        Board {
            tiles: vec![
                vec![TileType::Empty, TileType::Ground, TileType::Ground], 
                vec![TileType::Empty, TileType::Empty, TileType::Ground], 
                vec![TileType::Empty, TileType::Empty, TileType::Ground]
            ]
        }
    );
}

fn rotate(
    mut query: Query<&mut Transform, With<Square>>,
    time: Res<Time>
) {
    for mut transform in &mut query {
        transform.rotate_axis(vec3(0.0, 0.0, 1.0), time.delta_seconds());
    }
}
