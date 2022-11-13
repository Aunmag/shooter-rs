pub mod ext;
pub mod math;
mod timer;

pub use self::timer::*;
use bevy::prelude::Image;
use bevy::render::render_resource::Extent3d;
use bevy::render::render_resource::TextureDescriptor;
use bevy::render::render_resource::TextureDimension;
use bevy::render::render_resource::TextureFormat;
use bevy::render::render_resource::TextureUsages;

pub fn create_empty_image(size_x: u32, size_y: u32) -> Image {
    let size = Extent3d {
        width: size_x,
        height: size_y,
        ..Default::default()
    };

    let mut image = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Bgra8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
        },
        ..Default::default()
    };

    image.resize(size);

    return image;
}
