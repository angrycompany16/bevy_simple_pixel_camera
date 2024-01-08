use bevy::{
    prelude::*,
    render::{
        camera::*,
        render_resource::{
            Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages
        },
        view::RenderLayers,
    },
    math::*,
};

#[derive(Resource, Deref, DerefMut)]
pub struct RenderTexLayer(u8);

pub struct PixelCameraPlugin {
    pub pixel_scale_factor: f32,
    
    pub screen_width: f32,
    pub screen_height: f32,
    
    pub render_layer_index: u8
}

impl Plugin for PixelCameraPlugin {
    fn build(&self, app: &mut App) {

        let i_size = Extent3d {
            width: (self.screen_width / self.pixel_scale_factor) as u32, 
            height: (self.screen_height / self.pixel_scale_factor) as u32, 
            ..default()
        };
        
        let mut image = Image {
            texture_descriptor: TextureDescriptor {
                label: None,
                size: i_size,
                dimension: TextureDimension::D2,
                format: TextureFormat::Bgra8UnormSrgb,
                mip_level_count: 1,
                sample_count: 1,
                usage: TextureUsages::TEXTURE_BINDING
                    | TextureUsages::COPY_DST
                    | TextureUsages::RENDER_ATTACHMENT,
                view_formats: &[],
            },
            ..default()
        };
    
        image.resize(i_size);
        
        let mut images = app.world.get_resource_mut::<Assets<Image>>().unwrap();

        let image_handle = images.add(image);
    
        app.world.spawn((
            Camera2dBundle {
                camera: Camera {
                    // Render before Render Tex camera
                    order: -1,
                    target: RenderTarget::Image(image_handle.clone()),
                    ..default()
                },
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                ..default()
            },
        ));

        app.world.spawn((
            SpriteBundle {
                texture: image_handle,
                transform: Transform { 
                    scale: vec3(self.pixel_scale_factor, self.pixel_scale_factor, 1.0),
                    ..default()
                },
                ..default()
            },
            RenderLayers::layer(self.render_layer_index)
        ));

        app.insert_resource(RenderTexLayer(self.render_layer_index));
    }
}
