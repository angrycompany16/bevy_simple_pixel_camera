use bevy::{
    prelude::*,
    render::view::RenderLayers,
    window::*, sprite::MaterialMesh2dBundle, math::vec3
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
        .add_systems(Update, rotate_square)
        .run();
}

#[derive(Component)]
struct Square;

fn setup(
    mut commands: Commands,
    
    render_tex_layer: Res<RenderTexLayer>,
) {
    commands.spawn((
        Camera2dBundle::default(),
        RenderLayers::layer(**render_tex_layer)
    ));

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.5, 0.5, 0.8),
                ..default()
            },
            transform: Transform::from_scale(vec3(25.0, 25.0, 1.0)),
            ..default()
        },
        Square
    ));
}

fn rotate_square(
    mut square_q: Query<&mut Transform, With<Square>>,
    time: Res<Time>,
) {
    for mut transform in square_q.iter_mut() {
        transform.rotate_axis(Vec3::Z, time.delta_seconds());
    }
}