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
    // let rand_width = random::<f64>() * 100.0;
    // let rand_height = random::<f64>() * 100.0;
    // let rand_x = random::<f64>() * (width);
    // let rand_y = random::<f64>() * (height);

    let mut controler_position_x = 0.0;
    while let Some(e) = window.next() {
        // window.draw_2d(&e, |c, g, _device| {
        //     clear([1.0; 4], g);
        //     rectangle(get_rand_rgba(), // red
        //               [rand_x, rand_y, rand_width, rand_height],
        //               c.transform,
        //               g);
        // });
        window.draw_2d(&e, |c, g, _device| {
            clear([1.0; 4], g);
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