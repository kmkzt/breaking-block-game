use piston_window::*;

pub struct App {
    width: u32,
    height: u32,
    window: PistonWindow
}

pub impl App {
    pub fn new(&self) -> App {
        App {
            width: self.width,
            height: self.height,
            window: WindowSettings::new("Breaking blocks", [self.width, self.height])
                .exit_on_esc(true).build().unwrap()
        }
    }
}