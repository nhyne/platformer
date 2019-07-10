extern crate nalgebra;
extern crate nphysics2d;

use piston_window::math::{Matrix2d, Vec2d};
use piston_window::*;

use core::borrow::Borrow;
use nalgebra::{Isometry2, Vector2};
use ncollide2d::shape::{Cuboid, ShapeHandle};
use nphysics2d::algebra::{Force2, ForceType};
use nphysics2d::material::{BasicMaterial, MaterialHandle};
use nphysics2d::object::{Body, BodyHandle, ColliderDesc, RigidBodyDesc};
use nphysics2d::world::World;
use std::collections::HashSet;

const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
const PLAYER_BODY_WIDTH: f64 = 15.0;
const PLAYER_BODY_HEIGHT: f64 = 15.0;
const PLAYER_RENDER_WIDTH: f64 = PLAYER_BODY_WIDTH * 2.0;
const PLAYER_RENDER_HEIGHT: f64 = PLAYER_BODY_HEIGHT * 2.0;

pub struct Player {
    pub shape: Rectangle,
    pub body: BodyHandle,
}

impl Player {
    pub fn render<G: Graphics>(
        &self,
        context: Context,
        transform: Matrix2d,
        graphics: &mut G,
        world: &World<f64>,
    ) {
        let player_body = world.rigid_body(self.body);
        match player_body {
            None => {}
            Some(b) => {
                let player_body = b.borrow();
                let pos = player_body.position().translation.vector;
                self.shape.draw(
                    [
                        pos[0] - PLAYER_BODY_WIDTH,
                        pos[1] - PLAYER_BODY_HEIGHT,
                        PLAYER_RENDER_WIDTH,
                        PLAYER_RENDER_HEIGHT,
                    ],
                    &context.draw_state,
                    transform,
                    graphics,
                )
            }
        }
    }

    pub fn new(world: &mut World<f64>, position: (f64, f64)) -> Player {
        let player_shape = ShapeHandle::new(Cuboid::new(Vector2::new(
            PLAYER_BODY_WIDTH,
            PLAYER_BODY_HEIGHT,
        )));
        let player_collider = ColliderDesc::new(player_shape)
            .density(1.0)
            .material(MaterialHandle::new(BasicMaterial::new(0.0, 0.0)));
        let mut player_rb_desc = RigidBodyDesc::new()
            .collider(&player_collider)
            .position(Isometry2::translation(position.0, position.1));

        let player_rigid_body = player_rb_desc.build(world);
        let player_handle = player_rigid_body.handle();

        Player {
            body: player_handle,
            shape: Rectangle::new(BLACK),
        }
    }

    pub fn update(&mut self, world: &mut World<f64>, keys_pressed: &HashSet<Key>) {
        if keys_pressed.contains(&Key::Space) {
            self.jump(world);
        }
        if keys_pressed.contains(&Key::A) {
            self.move_left(world);
        }
        if keys_pressed.contains(&Key::D) {
            self.move_right(world);
        }
    }

    fn move_left(&self, world: &mut World<f64>) {
        if let Some(body) = world.rigid_body_mut(self.body) {
            let force = Force2::linear(Vector2::new(-5.0, 0.0));
            body.apply_force(1, &force, ForceType::VelocityChange, false);
        }
    }

    fn move_right(&self, world: &mut World<f64>) {
        if let Some(body) = world.rigid_body_mut(self.body) {
            let force = Force2::linear(Vector2::new(5.0, 0.0));
            body.apply_force(1, &force, ForceType::VelocityChange, false);
        }
    }

    fn jump(&self, world: &mut World<f64>) {
        if let Some(body) = world.rigid_body_mut(self.body) {
            let jump_force = Force2::linear(Vector2::new(0.0, -5.0));
            body.apply_force(1, &jump_force, ForceType::VelocityChange, false);
        }
    }
}
