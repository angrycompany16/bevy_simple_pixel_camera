use bevy::{
    prelude::*,
    render::view::RenderLayers,
    window::*, sprite::MaterialMesh2dBundle
};

use bevy_simple_pixel_camera::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(
            ImagePlugin::default_nearest()
        ).set(
            WindowPlugin {
                primary_window: Some(Window {
                    resolution: WindowResolution::new(
                        1200.0,
                        800.0,
                    ),
                    ..default()
                }),
            ..default()
        }
        ))
        .add_plugins(PixelCameraPlugin {
                pixel_scale_factor: 4.0,
                screen_width: 1200.0,
                screen_height: 800.0,
                render_layer_index: 1
        })
        .insert_resource(Msaa::Off)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    
    render_tex_layer: Res<RenderTexLayer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Camera2dBundle::default(),
        RenderLayers::layer(**render_tex_layer)
    ));

    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(shape::Circle::new(25.).into()).into(),
        material: materials.add(ColorMaterial::from(Color::rgb(0.9, 0.7, 0.7))),
        transform: Transform::from_translation(Vec3::new(-50., 0., 0.)),
        ..default()
    });

    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(shape::RegularPolygon::new(25., 6).into()).into(),
        material: materials.add(ColorMaterial::from(Color::rgb(0.7, 0.9, 0.7))),
        transform: Transform::from_translation(Vec3::new(50., 0., 0.)),
        ..default()
    });
}