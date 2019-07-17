use nphysics2d::world::World;
use piston_window::math::Matrix2d;
use piston_window::*;

pub trait Sprite {
    fn render<G: Graphics>(
        &self,
        context: Context,
        transform: Matrix2d,
        graphics: &mut G,
        world: &World<f64>,
    );
}

