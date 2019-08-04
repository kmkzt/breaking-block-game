use piston_window::{ellipse, math, G2d};

pub struct Ball {
    pub x: f64,
    pub y: f64,
    pub dx: f64,
    pub dy: f64
}

impl Ball {
    pub fn new(width: f64, height: f64) -> Self {
        Ball {
            x: width / 2.0,
            y: height - 50.0,
            dx: -1.0,
            dy: -1.0
        }
    }

    pub fn draw(&self, t: math::Matrix2d, g: &mut G2d) {
        ellipse([0.0, 0.0, 0.5, 1.0], [self.x, self.y, 20.0, 20.0], t, g);
    }

    pub fn effect(&mut self) {
        self.x += self.dx;
        self.y += self.dy;
    }
}
