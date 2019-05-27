extern crate nalgebra;
extern crate nphysics2d;

use piston_window::*;

use core::borrow::Borrow;
use nalgebra::Vector2;
use nphysics2d::object::{BodyHandle, Body};
use nphysics2d::world::World;
use nphysics2d::algebra::{Force2, ForceType};
use std::collections::HashSet;

const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
const PLAYER_WIDTH: f64 = 15.0;
const PLAYER_HEIGHT: f64 = 15.0;

pub struct Player {
    pub shape: Rectangle,
    pub body: BodyHandle,
}

impl Player {
    pub fn render<G: Graphics>(&self, context: Context, graphics: &mut G, world: &World<f64>) {
        let player_body = world.rigid_body(self.body);
        match player_body {
            None => {}
            Some(b) => {
                let player_body = b.borrow();
                let pos = player_body.position().translation.vector;
                self.shape.draw(
                    [pos[0], pos[1], PLAYER_WIDTH, PLAYER_HEIGHT],
                    &context.draw_state,
                    context.transform,
                    graphics,
                )
            }
        }
    }

    pub fn new(body: BodyHandle) -> Player {
        Player {
            body,
            shape: Rectangle::new(BLACK),
        }
    }

    pub fn update(&mut self, world: &mut World<f64>, keys_pressed: &HashSet<Key>) {
        if keys_pressed.contains(&Key::Space) {
            self.jump(world);
        }
    }

    fn jump(&self, world: &mut World<f64>) {
        if let Some(body) = world.rigid_body_mut(self.body) {
            let jump_force = self.jump_factory();
            body.apply_force(1, &jump_force, ForceType::VelocityChange, false);
        }
    }

    //pub fn change_velocity(&mut self, world: &mut World<f64>) {
    //    let player_body = world.rigid_body_mut(self.body);
    //    match player_body {
    //        None => {},
    //        Some(b) => {

    //        }
    //    }
    //}

    fn jump_factory(&self) -> Force2<f64> {
        Force2::linear(Vector2::new(0.0, -10.0))
    }
}

#[cfg(test)]
mod tests {
    use crate::game::player::{Player, PLAYER_SPEED};
    use nalgebra::Vector2;
    use ncollide2d::shape::{Cuboid, ShapeHandle};
    use nphysics2d::object::{BodyStatus, ColliderDesc, RigidBodyDesc};
    use nphysics2d::world::World;

    fn init_player(world: &mut World<f64>) -> Player {
        let player_shape = ShapeHandle::new(Cuboid::new(Vector2::new(7.5, 25.0)));
        let player_collider = ColliderDesc::new(player_shape);
        let player_rb_desc = RigidBodyDesc::new().collider(&player_collider);

        let player_rigid_body = player_rb_desc.status(BodyStatus::Kinematic).build(world);
        let player_handle = player_rigid_body.handle();

        Player::new(player_handle)
    }

    #[test]
    fn player_move_down() {
        let mut world = World::new();
        let mut player = init_player(&mut world);

        let initial_pos = if let Some(body) = world.rigid_body(player.body) {
            body.position().translation.vector[1]
        } else {
            0.0
        };

        player.move_down(&mut world);

        let new_pos = if let Some(body) = world.rigid_body(player.body) {
            body.position().translation.vector[1]
        } else {
            0.0
        };

        assert_eq!(initial_pos + PLAYER_SPEED, new_pos);
    }

    #[test]
    fn player_move_up() {
        let mut world = World::new();
        let mut player = init_player(&mut world);

        let initial_pos = if let Some(body) = world.rigid_body(player.body) {
            body.position().translation.vector[1]
        } else {
            0.0
        };

        player.move_up(&mut world);

        let new_pos = if let Some(body) = world.rigid_body(player.body) {
            body.position().translation.vector[1]
        } else {
            0.0
        };

        assert_eq!(initial_pos - PLAYER_SPEED, new_pos);
    }
}
