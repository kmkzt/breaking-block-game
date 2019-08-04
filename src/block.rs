extern crate piston_window;
extern crate rand;

use rand::prelude::*;
use piston_window::*;

pub struct Block {
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
    pub color: types::Color
}

impl Block {
    pub fn new(self) -> Self {
        Block {
            x: self.x,
            y: self.y,
            w: self.w,
            h: self.h,
            color: get_rand_rgba()
        }
    }

    pub fn new_rand(mx: f64, my: f64) -> Self {
        Block {
            x: random::<f64>() * mx,
            y: random::<f64>() * my,
            w: 100.0,
            h: 20.0,
            color: get_rand_rgba()
        }
    }

    pub fn draw(&self, t: math::Matrix2d, g: &mut G2d) {
        rectangle(self.color, [self.x, self.y, self.w, self.h], t, g);
    }

    pub fn rand(&mut self, mx: f64, my: f64) {
        *self = Block::new_rand(mx, my);
    }

}

fn get_rand_rgba() -> types::Color {
    [
        random::<f32>(),
        random::<f32>(),
        random::<f32>(),
        1.0
    ]
}