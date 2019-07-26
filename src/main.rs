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
    let mut ball_x = width / 2.0;
    let mut ball_y = height - 50.0;
    let mut ball_move_x = -1.0;
    let mut ball_move_y = -1.0;
    let mut controller_position_x = width / 2.0;
    let controller_move_speed = 15.0;
    while let Some(e) = window.next() {
        if (ball_x < 0.0 && ball_move_x < 0.0) || (ball_x > width && ball_move_x > 0.0)  {
            ball_move_x *= -1.0;
        }
        if (ball_y < 0.0 && ball_move_y < 0.0) || (ball_y > height && ball_move_y > 0.0)  {
            ball_move_y *= -1.0;
        }

        window.draw_2d(&e, |c, g, _device| {
            clear([1.0; 4], g);
            rectangle(block_color,
                      [block_x, block_y, block_width, block_height],
                      c.transform,
                      g);
            ellipse([0.0, 0.0, 0.5, 1.0], [ball_x, ball_y, 20.0, 20.0], c.transform, g);
            rectangle([0.0, 0.0, 1.0, 1.0], // red
                    [controller_position_x, height - 20.0, 100.0, 20.0],
                    c.transform,
                    g);
        });
        ball_x += ball_move_x;
        ball_y += ball_move_y;
        if let Some(ref args) = e.press_args() {
            use piston_window::Button::Keyboard;
            
            if *args == Keyboard(Key::Left) {
                controller_position_x -= controller_move_speed;
                println!("left -> {}", controller_position_x);
            }
            if *args == Keyboard(Key::Right) {
                controller_position_x += controller_move_speed;
                println!("right -> {}", controller_position_x);
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