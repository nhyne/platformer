use ncollide2d::shape::{Shape, ShapeHandle};
use nphysics2d::object::{Body, BodyHandle, ColliderDesc, RigidBodyDesc};

pub struct Sprite {
    // TODO: need to have a collider of some kind as well
    //pub shape: Shape<f64>,
    pub body_handle: BodyHandle,
}

