mod envelope;
pub mod ext;
mod interpolation;
pub mod math;
#[cfg(test)]
pub mod test;
mod timer;

pub use self::{envelope::*, interpolation::*, timer::*};
use bevy::{prelude::Image, render::render_resource::Extent3d};

pub fn create_empty_image(size_x: u32, size_y: u32) -> Image {
    let mut image = Image::default();

    image.resize(Extent3d {
        width: size_x,
        height: size_y,
        ..Default::default()
    });

    return image;
}
