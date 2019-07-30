extern crate piston_window;
extern crate rand;
extern crate input;

use rand::prelude::*;
use piston_window::*;
// use std::io;

struct Block {
    x: f64,
    y: f64,
    w: f64,
    h: f64,
    color: types::Color
}

struct Ball {
    x: f64,
    y: f64,
    dx: f64,
    dy: f64
}

fn main() {
    let width: f64 = 640.0;
    let height: f64 = 480.0;
    let mut window: PistonWindow =
        WindowSettings::new("Breaking blocks", [width as u32, height as u32])
        .exit_on_esc(true).build().unwrap();
    let block = &mut gen_rand_block(width, height);
    let ball = &mut init_ball_position(width, height);
    let mut controller_position_x = width / 2.0;
    let controller_move_speed = 15.0;
    while let Some(e) = window.next() {
        if (ball.x < 0.0 && ball.dx < 0.0) || (ball.x > width && ball.dx > 0.0)  {
            ball.dx *= -1.0;
        }
        if ball.y < 0.0 && ball.dy < 0.0  {
            ball.dy *= -1.0;
        }
        // reset
        if ball.y > height && ball.dy > 0.0 {
            ;
        }
        
        window.draw_2d(&e, |c, g, _device| {
            clear([1.0; 4], g);
            rectangle(block.color,
                      [block.x, block.y, block.w, block.h],
                      c.transform,
                      g);
            ellipse([0.0, 0.0, 0.5, 1.0], [ball.x, ball.y, 20.0, 20.0], c.transform, g);
            rectangle([0.0, 0.0, 1.0, 1.0], // red
                    [controller_position_x, height - 20.0, 100.0, 20.0],
                    c.transform,
                    g);
        });
        ball.x += ball.dx;
        ball.y += ball.dy;
        if let Some(ref args) = e.mouse_cursor_args() {
            println!("{:?}", *args);
            let [mouse_x, _mouse_y] = *args;
            controller_position_x = mouse_x;
        }
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

fn gen_rand_block( x: f64, y: f64) -> Block {
    Block {
        x: random::<f64>() * x,
        y: random::<f64>() * y / 2.0,
        w: random::<f64>() * 100.0,
        h: random::<f64>() * 100.0,
        color: get_rand_rgba()
    }
}

fn init_ball_position(width: f64, height: f64) -> Ball {
    Ball {
        x: width / 2.0,
        y: height - 50.0,
        dx: -1.0,
        dy: -1.0
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