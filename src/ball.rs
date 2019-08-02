pub struct Ball {
    pub x: f64,
    pub y: f64,
    pub dx: f64,
    pub dy: f64
}

impl Ball {
    pub fn new(width: f64, height: f64) -> Self {
        Ball::init_ball_position(width, height)
    }
    pub fn init_ball_position(width: f64, height: f64) -> Self {
        Ball {
            x: width / 2.0,
            y: height - 50.0,
            dx: -1.0,
            dy: -1.0
        }
    }
}
