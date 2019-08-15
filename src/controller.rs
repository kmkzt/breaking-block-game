use piston_window::{rectangle, math, G2d};
use std::option::Option;
use crate::ball::Ball;
use crate::draw::Draw2d;

#[derive(Debug)]
pub struct Controller {
    pub w: f64,
    pub h: f64,
    pub x: f64,
    pub y: f64,
    pub move_speed: f64
}

const CONTROLER_WIDTH: f64 = 100.0;
const CONTROLER_HEIGHT: f64 = 20.0;
const CONTROLER_OFFSET_Y: f64 = CONTROLER_HEIGHT + 30.0;
const SWING_WIDTH: f64 = 1.5;

#[derive(Debug)]
pub struct Bounce {
    pub dx: f64,
    pub dy: f64
}

impl Draw2d for Controller {
    fn draw2d(&self, t: math::Matrix2d, g: &mut G2d) {
        rectangle([0.0, 0.0, 1.0, 1.0], // red
                    [self.x, self.y, self.w, self.h],
                    t,
                    g);
    }
}

impl Controller {
    pub fn new(width: f64, height: f64) -> Self {
        Controller {
            w: CONTROLER_WIDTH,
            h: CONTROLER_HEIGHT,
            x: (width - CONTROLER_WIDTH) / 2.0,
            y: height - CONTROLER_OFFSET_Y,
            move_speed: 30.0
        }
    }

    pub fn draw(&self, t: math::Matrix2d, g: &mut G2d) {
       self.draw2d(t, g)
    }

    pub fn touch(&self, ball: &Ball) -> Option<Bounce> {
        match ball.dy > 0.0 
            && ball.y + (ball.size / 2.0 ) >= self.y
            && ball.y + (ball.size / 2.0 ) <= self.y + CONTROLER_HEIGHT
            && ball.x >= self.x
            && ball.x <= self.x + self.w
        {
            true => {
                let dx = 2.0 * (ball.x - self.x) / self.w - 1.0;
                let bounce = Bounce {
                    dx: dx * SWING_WIDTH,
                    dy: -1.0
                };
                Some(bounce)
            },
            false => None
        }   
        
    }

}