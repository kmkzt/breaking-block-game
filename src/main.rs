extern crate piston_window;
extern crate rand;

use rand::prelude::*;
use piston_window::*;
// use std::io;

fn main() {
    let width = 640;
    let height = 480;
    let mut window: PistonWindow =
        WindowSettings::new("Breaking blocks", [width, height])
        .exit_on_esc(true).build().unwrap();
    while let Some(event) = window.next() {
        let rand_width = random::<f64>() * 100.0;
        let rand_height = random::<f64>() * 100.0;
        let rand_x = random::<f64>() * (width as f64);
        let rand_y = random::<f64>() * (height as f64);

        window.draw_2d(&event, |c, g, _device| {
            clear([1.0; 4], g);
            rectangle(get_rand_rgba(), // red
                      [rand_x, rand_y, rand_width, rand_height],
                      c.transform,
                      g);
        });
    }
}


fn get_rand_rgba() -> [f32; 4] {
    [
        random::<f32>(),
        random::<f32>(),
        random::<f32>(),
        random::<f32>()
    ]
}
// fn get_keyboard_event() -> io::Result<String> {
//     let mut input = String::new();

//     io::stdin().read_line(&mut input)?;

//     println!("You typed: {}", input.trim());
//     Ok(input)
// }