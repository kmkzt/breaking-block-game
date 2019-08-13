extern crate piston_window;
use piston_window::{math, G2d};

pub trait Draw2d {
    fn draw2d(&self, t: math::Matrix2d, g: &mut G2d);
}