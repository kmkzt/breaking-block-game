extern crate piston_window;

use piston_window::{ellipse, math, G2d};
use crate::draw::Draw2d;

#[derive(Debug)]
pub struct Ball {
    pub x: f64,
    pub y: f64,
    pub dx: f64,
    pub dy: f64,
    pub size: f64
}

impl Draw2d for Ball {
    fn draw2d(&self, t: math::Matrix2d, g: &mut G2d) {
        ellipse([0.0, 0.0, 0.5, 1.0], [self.x, self.y, self.size, self.size], t, g);
    }
}

impl Ball {
    pub fn new(width: f64, height: f64) -> Self {
        Ball {
            x: width / 2.0,
            y: height - 50.0,
            dx: -1.0,
            dy: -1.0,
            size: 20.0
        }
    }

    pub fn effect(&mut self) {
        self.x += self.dx;
        self.y += self.dy;
    }

    pub fn bounce_frame(&mut self, width: f64, _height: f64) {
         if (self.x < 0.0 && self.dx < 0.0) || (self.x + self.size > width && self.dx > 0.0)  {
            self.dx *= -1.0;
        }
        if self.y < 0.0 && self.dy < 0.0 {
            self.dy *= -1.0;
        }
    }
    pub fn draw(&self, t: math::Matrix2d, g: &mut G2d) {
        self.draw2d(t, g);
    }
}
