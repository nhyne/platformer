extern crate nalgebra;
extern crate nphysics2d;

use core::borrow::Borrow;
use nalgebra::{Isometry2, Vector2};
use ncollide2d::shape::{Cuboid, ShapeHandle};
use nphysics2d::material::{BasicMaterial, MaterialHandle};
use nphysics2d::object::{BodyHandle, Body, BodyPartHandle, ColliderDesc, RigidBodyDesc};
use nphysics2d::world::World;
use piston_window::*;

const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
const ENEMY_WIDTH: f64 = 15.0;
const ENEMY_HEIGHT: f64 = 15.0;

pub struct Enemy {
    pub shape: Rectangle,
    pub body: BodyHandle
}

impl Enemy {
    pub fn new(world: &mut World<f64>, position: (f64, f64)) -> Enemy {
        let enemy_shape = ShapeHandle::new(Cuboid::new(Vector2::new(7.5, 25.0)));
        let enemy_collider = ColliderDesc::new(enemy_shape)
            .density(1.0)
            .material(MaterialHandle::new(BasicMaterial::new(0.0, 0.0)));
        let mut enemy_rb_desc = RigidBodyDesc::new()
            .collider(&enemy_collider)
            .position(Isometry2::translation(position.0, position.1));

        let enemy_rigid_body = enemy_rb_desc.build(world);
        let enemy_handle = enemy_rigid_body.handle();

        Enemy {
            body: enemy_handle,
            shape: Rectangle::new(RED),
        }
    }

    pub fn render<G: Graphics>(&self, context: Context, graphics: &mut G, world: &World<f64>) {
        if let Some(enemy_body) = world.rigid_body(self.body) {
            let enemy_body = enemy_body.borrow();
            let pos = enemy_body.position().translation.vector;
            self.shape.draw(
                [pos[0], pos[1], ENEMY_WIDTH, ENEMY_HEIGHT],
                &context.draw_state,
                context.transform,
                graphics,
            )
        }
    }
}
