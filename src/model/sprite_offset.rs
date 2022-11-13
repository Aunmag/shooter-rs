use bevy::math::Vec2;
use bevy::prelude::Image;
use bevy::sprite::Anchor;

pub struct SpriteOffset {
    pub x: Option<f32>,
    pub y: Option<f32>,
}

impl SpriteOffset {
    pub const fn new(x: Option<f32>, y: Option<f32>) -> Self {
        return Self { x, y };
    }

    pub fn to_anchor(&self, image: &Image) -> Anchor {
        let size = image.size();
        return Anchor::Custom(Vec2::new(
            self.x.map_or(0.0, |x| x / size.x - 0.5),
            self.y.map_or(0.0, |y| y / size.y - 0.5),
        ));
    }
}
