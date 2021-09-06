use macroquad::prelude::Vec2;

#[derive(Debug)]
pub struct Bounds {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
}

pub trait Collider {
    fn pos(&self) -> Vec2;
    fn dim(&self) -> Vec2;

    fn bounds(&self) -> Bounds {
        let pos = self.pos();
        let dim = self.dim();

        Bounds {
            left: pos.x,
            right: pos.x + dim.x,
            top: pos.y,
            bottom: pos.y + dim.y,
        }
    }

    fn collides_with(&self, other: &impl Collider) -> bool {
        let self_bounds = self.bounds();
        let other_bounds = other.bounds();

        self_bounds.left < other_bounds.right
            && self_bounds.right > other_bounds.left
            && self_bounds.top < other_bounds.bottom
            && self_bounds.bottom > other_bounds.top
    }
}
