extern crate piston_window;
extern crate rand;

use std::iter::*;
use rand::prelude::*;
use piston_window::*;
use crate::ball::Ball;
use crate::draw::Draw2d;

const BLOCK_WIDTH: f64 = 100.0;
const BLOCK_HEIGHT: f64 = 20.0;

pub struct Block {
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
    pub color: types::Color
}

pub enum Hit {
    Top = 0,
    Right = 1,
    Bottom = 2,
    Left = 3,
}

impl Draw2d for Block {
    fn draw2d(&self, t: math::Matrix2d, g: &mut G2d) {
        rectangle(self.color, [self.x, self.y, self.w, self.h], t, g);
    }
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
            w: BLOCK_WIDTH,
            h: BLOCK_HEIGHT,
            color: get_rand_rgba()
        }
    }

    pub fn draw(&self, t: math::Matrix2d, g: &mut G2d) {
        self.draw2d(t, g);
    }

    pub fn rand(&mut self, mx: f64, my: f64) {
        *self = Block::new_rand(mx, my);
    }

    pub fn touch(&self, ball: &Ball) -> Option<Hit> {
        let hit_info = [ball.x - self.x, self.x + self.w - ball.x, ball.y - self.y, self.y + self.h - ball.y];
        let mut near_index = 4;
        let mut near_point = 5.0;
        
        for (i, &p) in hit_info.iter().enumerate() {
            if p < 0.0 {
                near_index = 4;
                break;
            }

            if near_point > p {
                near_point = p;
                near_index = i;
            }
            
        }
        
        match near_index {
            0 => Some(Hit::Left),
            1 => Some(Hit::Right),
            2 => Some(Hit::Top),
            3 => Some(Hit::Bottom),
            _ => None
        }
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