use bevy::{prelude::Image, render::render_resource::Extent3d};

pub trait ImageExt {
    fn blank(size_x: u32, size_y: u32) -> Self;
    fn size_x(&self) -> u32;
    fn size_y(&self) -> u32;
}

impl ImageExt for Image {
    fn blank(size_x: u32, size_y: u32) -> Self {
        let mut image = Self::default();

        image.resize(Extent3d {
            width: size_x,
            height: size_y,
            ..Default::default()
        });

        return image;
    }

    fn size_x(&self) -> u32 {
        return self.texture_descriptor.size.width;
    }

    fn size_y(&self) -> u32 {
        return self.texture_descriptor.size.height;
    }
}
