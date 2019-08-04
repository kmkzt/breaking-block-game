use piston_window::{rectangle, math, G2d};

pub struct Controller {
    pub w: f64,
    pub h: f64,
    pub x: f64,
    pub y: f64,
    pub move_speed: f64
}

const CONTROLER_WIDTH: f64 = 100.0;
const CONTROLER_HEIGHT: f64 = 20.0;

impl Controller {
    pub fn new(width: f64, height: f64) -> Self {
        Controller {
            w: CONTROLER_WIDTH,
            h: CONTROLER_HEIGHT,
            x: (width - CONTROLER_WIDTH) / 2.0,
            y: height - CONTROLER_HEIGHT,
            move_speed: 30.0
        }
    }

    pub fn draw(&self, t: math::Matrix2d, g: &mut G2d) {
       rectangle([0.0, 0.0, 1.0, 1.0], // red
                    [self.x, self.y, self.w, self.h],
                    t,
                    g);
    }

}