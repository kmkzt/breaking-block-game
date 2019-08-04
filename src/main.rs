extern crate piston_window;
extern crate rand;
extern crate input;

use piston_window::*;
// use std::fmt::Debug;
// use std::io;

mod ball;
mod block;
mod controller;

use ball::Ball;
use block::Block;
use controller::Controller;

fn main() {
    let width: f64 = 640.0;
    let height: f64 = 480.0;
    let mut window: PistonWindow =
        WindowSettings::new("Breaking blocks", [width as u32, height as u32])
        .exit_on_esc(true).build().unwrap();
    let mut block = Block::new_rand(width, height / 2.0);
    let mut ball = Ball::new(width, height);
    let mut controller = Controller::new(width, height);
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
            && ball.y == controller.y
            && controller.x <= ball.x 
            && ball.x <= controller.x + controller.w
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
            block.rand(width, height / 2.0);
        }  

        // Draw a screen.
        window.draw_2d(&e, |c, g, _device| {
            clear([1.0; 4], g);
            block.draw(c.transform, g);
            ball.draw(c.transform, g);
            controller.draw(c.transform, g);
        });

        // Coordinate change of ball.
        ball.effect();

        // Mouse Event Handler
        if let Some(ref args) = e.mouse_cursor_args() {
            println!("{:?}", *args);
            let [mouse_x, _mouse_y] = *args;
            controller.x = mouse_x;
        }

        // Key Event Handler
        if let Some(ref args) = e.press_args() {
            use piston_window::Button::Keyboard;
            
            if *args == Keyboard(Key::Left) {
                controller.x -= controller.move_speed;
                println!("left: {}", controller.x);
            }
            if *args == Keyboard(Key::Right) {
                controller.x += controller.move_speed;
                println!("right: {}", controller.x);
            }
            if *args == Keyboard(Key::Space) {
                ball = Ball::new(width, height);
                println!("Restart!!");
            }
        }

        // Window Resize Event Handler
        if let Some(ref args) = e.resize_args() {
            println!("Update Window Size: {:?}", *args);
            let [w, h] = args.window_size;
            controller = Controller::new(w, h);
            block.rand(width, height / 2.0);
        }

    }
}


// fn get_keyboard_event() -> io::Result<String> {
//     let mut input = String::new();

//     io::stdin().read_line(&mut input)?;

//     println!("You typed: {}", input.trim());
//     Ok(input)
// }