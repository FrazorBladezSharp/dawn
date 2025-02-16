
use specs::prelude::*;
use specs_derive::Component;

#[derive(Component, Default)]
pub struct Position{
    pub x: f32,
    pub y: f32,
}
/*
#[derive(Component, Default)]
pub struct Velocity{
    pub x: f32,
    pub y: f32,
}
*/
#[derive(Component, Default)]
pub struct Size{
    pub width: f32,
    pub height: f32,
}
