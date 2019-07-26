extern crate piston_window;
extern crate rand;
extern crate input;

use rand::prelude::*;
use piston_window::*;
// use std::io;

fn main() {
    let width: f64 = 640.0;
    let height: f64 = 480.0;
    let mut window: PistonWindow =
        WindowSettings::new("Breaking blocks", [width as u32, height as u32])
        .exit_on_esc(true).build().unwrap();
    let block_width = random::<f64>() * 100.0;
    let block_height = random::<f64>() * 100.0;
    let block_x = random::<f64>() * (width);
    let block_y = random::<f64>() * (height / 2.0);
    let block_color = get_rand_rgba();
    let ball_x = width / 2.0;
    let ball_y = height - 50.0;
    let mut controler_position_x = width / 2.0;
    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g, _device| {
            clear([1.0; 4], g);
            rectangle(block_color,
                      [block_x, block_y, block_width, block_height],
                      c.transform,
                      g);
            ellipse([0.0, 0.0, 0.5, 1.0], [ball_x, ball_y, 20.0, 20.0], c.transform, g);
            rectangle([0.0, 0.0, 1.0, 1.0], // red
                    [controler_position_x, height - 20.0, 100.0, 20.0],
                    c.transform,
                    g);
        });
        if let Some(ref args) = e.press_args() {
            use piston_window::Button::Keyboard;
            
            if *args == Keyboard(Key::Left) {
                controler_position_x = controler_position_x - 5.0;
                println!("left -> {}", controler_position_x);
            }
            if *args == Keyboard(Key::Right) {
                controler_position_x = controler_position_x + 5.0;
                println!("right -> {}", controler_position_x);
            }
            
        }
    }
}


fn get_rand_rgba() -> types::Color {
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