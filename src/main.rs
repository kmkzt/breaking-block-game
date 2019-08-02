extern crate piston_window;
extern crate rand;
extern crate input;

use rand::prelude::*;
use piston_window::*;
// use std::fmt::Debug;
// use std::io;

mod ball;
use ball::Ball;

struct Block {
    x: f64,
    y: f64,
    w: f64,
    h: f64,
    color: types::Color
}


fn main() {
    let mut width: f64 = 640.0;
    let mut height: f64 = 480.0;
    let mut window: PistonWindow =
        WindowSettings::new("Breaking blocks", [width as u32, height as u32])
        .exit_on_esc(true).build().unwrap();
    let block = &mut gen_rand_block(width, height);
    let ball = &mut Ball::new(width, height);
    let controller_height = 20.0;
    let controller_width = 100.0;
    let mut controller_position_y = height - controller_height;
    let mut controller_position_x = width / 2.0;
    let controller_move_speed = 30.0;
    while let Some(e) = window.next() {
        // Bounce Frame 
        if (ball.x < 0.0 && ball.dx < 0.0) || (ball.x > width && ball.dx > 0.0)  {
            ball.dx *= -1.0;
        }
        if ball.y < 0.0 && ball.dy < 0.0 {
            ball.dy *= -1.0;
        }
        
        // Bounce Controller
        if ball.dy > 0.0 
            && ball.y == controller_position_y 
            && controller_position_x <= ball.x 
            && ball.x <= controller_position_x + controller_width 
        {
            ball.dy *= -1.0;
        }
        
        // Bounce Block
        if block.x <= ball.x 
            && ball.x <= block.x + block.w
            && block.y <= ball.y
            && ball.y <= block.y + block.h
        {
            ball.dy *= -1.0;
            ball.dx *= -1.0;
            *block = gen_rand_block(width, height);
        }  

        // Draw a screen.
        window.draw_2d(&e, |c, g, _device| {
            clear([1.0; 4], g);
            rectangle(block.color,
                      [block.x, block.y, block.w, block.h],
                      c.transform,
                      g);
            ball.draw(c.transform, g);
            rectangle([0.0, 0.0, 1.0, 1.0], // red
                    [controller_position_x, controller_position_y, controller_width, controller_height],
                    c.transform,
                    g);
        });

        // Coordinate change of ball.
        ball.effect();

        // Mouse Event Handler
        if let Some(ref args) = e.mouse_cursor_args() {
            println!("{:?}", *args);
            let [mouse_x, _mouse_y] = *args;
            controller_position_x = mouse_x;
        }

        // Key Event Handler
        if let Some(ref args) = e.press_args() {
            use piston_window::Button::Keyboard;
            
            if *args == Keyboard(Key::Left) {
                controller_position_x -= controller_move_speed;
                println!("left: {}", controller_position_x);
            }
            if *args == Keyboard(Key::Right) {
                controller_position_x += controller_move_speed;
                println!("right: {}", controller_position_x);
            }
            if *args == Keyboard(Key::Space) {
                *ball = Ball::new(width, height);
                println!("Restart!!");
            }
        }

        // Window Resize Event Handler
        if let Some(ref args) = e.resize_args() {
            println!("Update Window Size: {:?}", *args);
            let [w, h] = args.window_size;
            width = w;
            height = h;
            controller_position_y = height - controller_height;
            controller_position_x = width / 2.0;
            *block = gen_rand_block(width, height);
        }

    }
}

fn gen_rand_block( x: f64, y: f64) -> Block {
    Block {
        x: random::<f64>() * x,
        y: random::<f64>() * y / 2.0,
        w: 100.0,
        h: 20.0,
        color: get_rand_rgba()
    }
}


fn get_rand_rgba() -> types::Color {
    [
        random::<f32>(),
        random::<f32>(),
        random::<f32>(),
        1.0
    ]
}

// fn get_keyboard_event() -> io::Result<String> {
//     let mut input = String::new();

//     io::stdin().read_line(&mut input)?;

//     println!("You typed: {}", input.trim());
//     Ok(input)
// }