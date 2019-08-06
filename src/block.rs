extern crate piston_window;
extern crate rand;

use rand::prelude::*;
use piston_window::*;
use crate::ball::Ball;

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

    pub fn touch(&self, ball: &Ball) -> bool {
        let left = ball.x - self.x;
        let right = self.x + self.w - ball.x;
        let top =  ball.y - self.y;
        let bottom = self.y + self.h - ball.y;

        left >= 0.0 && right >= 0.0 && top >= 0.0 && bottom >= 0.0
        
        // if left >0 
        //     && ball.x <= self.x + self.w
        //     && self.y <= ball.y
        //     && ball.y <= self.y + self.h 
        // {
        //     println!("touch: ");
        //     if self.x <= ball.x {
        //         Some(Hit::Left)
        //     }
        //     if self.x + self.w >= ball.x {
        //         Some(Hit::Right)
        //     }
        //     if self.y <= ball.y {
        //         Some(Hit::Top)
        //     }
        //     if self.y + self.h >= ball.y {
        //         Some(Hit::Bottom)
        //     }
        // }
        // None
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