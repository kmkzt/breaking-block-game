pub mod ball {
    pub struct Ball {
        x: f64,
        y: f64,
        dx: f64,
        dy: f64
    }

    pub fn init_ball_position(width: f64, height: f64) -> Ball {
        Ball {
            x: width / 2.0,
            y: height - 50.0,
            dx: -1.0,
            dy: -1.0
        }
    }
}
