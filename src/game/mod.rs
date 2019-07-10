mod enemy;
mod player;

extern crate nalgebra;
extern crate ncollide2d;
extern crate nphysics2d;
extern crate piston_window;

use piston_window::math::Matrix2d;
use piston_window::*;

use crate::entities::sprite;

use nalgebra::{Isometry2, Vector2};
use ncollide2d::shape::{Cuboid, ShapeHandle};
use nphysics2d::material::{BasicMaterial, MaterialHandle};
use nphysics2d::object::{BodyPartHandle, ColliderDesc};
use nphysics2d::world::World;

use self::ncollide2d::events::ContactEvent;
use std::collections::HashSet;

const FLOOR_WIDTH: f64 = 800.0;
const FLOOR_HEIGHT: f64 = 10.0;
const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

pub struct Game {
    world: World<f64>,
    player: player::Player,
    keys_pressed: HashSet<piston_window::Key>,
    enemies: Vec<enemy::Enemy>,
}

impl Game {
    pub fn new() -> Game {
        let mut world: World<f64> = World::new();
        world.set_gravity(Vector2::y() * 90.81);

        let player = player::Player::new(&mut world, (50.0, 200.0));

        let enemies = vec![enemy::Enemy::new(&mut world, (50.0, 250.0))];

        Game::init_ground(&mut world);

        Game {
            world,
            player,
            keys_pressed: HashSet::new(),
            enemies,
        }
    }

    pub fn update(&mut self) {
        self.player.update(&mut self.world, &self.keys_pressed);

        for contact_event in self.world.contact_events() {
            self.handle_contact_event(contact_event);
        }

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

    pub fn render<G: Graphics>(&self, context: Context, transform: Matrix2d, graphics: &mut G) {
        clear([0.8, 0.8, 0.8, 1.0], graphics);
        graphics.clear_stencil(0);

        self.player
            .render(context, transform, graphics, &self.world);

        for enemy in &self.enemies {
            enemy.render(context, transform, graphics, &self.world);
        }

        self.render_ground(context, transform, graphics);
    }

    fn handle_contact_event(&self, contact_event: &ContactEvent) {
        if let &ContactEvent::Started(collider1, collider2) = contact_event {
            // logic for the actual collision
            // need to figure out how to handle the collisions without just iterating over type that it could be like "bullet, enemy, or spell"
            println!("there was a collision");
        }
    }

    fn init_ground(world: &mut World<f64>) {
        // do ground stuff
        let wall_shape = ShapeHandle::new(Cuboid::new(Vector2::new(
            FLOOR_WIDTH / 2.0,
            FLOOR_HEIGHT / 2.0,
        )));
        ColliderDesc::new(wall_shape)
            .material(MaterialHandle::new(BasicMaterial::new(0.0, 0.0)))
            .position(Isometry2::translation(200.0, 400.0))
            .build_with_parent(BodyPartHandle::ground(), world);
    }

    fn render_ground<G: Graphics>(&self, context: Context, transform: Matrix2d, graphics: &mut G) {
        let rectangle = Rectangle::new(BLACK);
        rectangle.draw(
            [0.0, 395.0, FLOOR_WIDTH, FLOOR_HEIGHT],
            &context.draw_state,
            transform,
            graphics,
        );
    }
}
