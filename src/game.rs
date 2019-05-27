mod player;

extern crate nalgebra;
extern crate ncollide2d;
extern crate nphysics2d;
extern crate piston_window;

use piston_window::*;

use nalgebra::{Isometry2, Vector2};
use ncollide2d::shape::{Cuboid, ShapeHandle};
use nphysics2d::material::{BasicMaterial, MaterialHandle};
use nphysics2d::object::{BodyPartHandle, ColliderDesc, RigidBodyDesc};
use nphysics2d::world::World;

use std::collections::HashSet;

const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

pub struct Game {
    world: World<f64>,
    player: player::Player,
    keys_pressed: HashSet<piston_window::Key>,
}

impl Game {
    pub fn new() -> Game {
        let mut world: World<f64> = World::new();
        world.set_gravity(Vector2::y() * 90.81);

        let player = Game::init_player(&mut world, (50.0, 200.0));
        Game::init_ground(&mut world);

        Game {
            world,
            player,
            keys_pressed: HashSet::new(),
        }
    }

    pub fn update(&mut self) {
        self.player.update(&mut self.world, &self.keys_pressed);

        self.world.step();
    }

    pub fn handle_keyboard_event(&mut self, key: ButtonArgs) {
        match key.state {
            ButtonState::Press => {
                if let Button::Keyboard(key) = key.button {
                    self.keys_pressed.insert(key);
                }
            }
            ButtonState::Release => {
                if let Button::Keyboard(key) = key.button {
                    self.keys_pressed.remove(&key);
                }
            }
        }
    }

    pub fn render<G: Graphics>(&self, context: Context, graphics: &mut G) {
        clear([0.8, 0.8, 0.8, 1.0], graphics);
        graphics.clear_stencil(0);

        self.player.render(context, graphics, &self.world);

        self.render_ground(context, graphics)
    }

    fn init_player(
        world: &mut World<f64>,
        position: (f64, f64),
    ) -> player::Player {
        let player_shape = ShapeHandle::new(Cuboid::new(Vector2::new(7.5, 25.0)));
        let player_collider = ColliderDesc::new(player_shape)
            .density(1.0)
            .material(MaterialHandle::new(BasicMaterial::new(0.0, 0.0)));
        let mut player_rb_desc = RigidBodyDesc::new()
            .collider(&player_collider)
            .position(Isometry2::translation(position.0, position.1));

        let player_rigid_body = player_rb_desc.build(world);
        let player_handle = player_rigid_body.handle();

        player::Player::new(player_handle)
    }
     fn init_ground(world: &mut World<f64>) {
         // do ground stuff
         let wall_shape = ShapeHandle::new(Cuboid::new(Vector2::new(
             100.0,
             10.0,
         )));
         ColliderDesc::new(wall_shape)
             .material(MaterialHandle::new(BasicMaterial::new(0.0, 0.0)))
             .position(Isometry2::translation(
                 0.0,
                 400.0,
             ))
             .build_with_parent(BodyPartHandle::ground(), world);
     }

    fn render_ground<G: Graphics>(&self, context: Context, graphics: &mut G) {
        let empty_transform = context.transform.trans(0.0, 0.0);
        let rectangle = Rectangle::new(BLACK);
        rectangle.draw(
            [
                0.0,
                400.0,
                100.0,
                10.0,
            ],
            &context.draw_state,
            empty_transform,
            graphics,
        );
    }
}
