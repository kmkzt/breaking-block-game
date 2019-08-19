extern crate piston_window;
extern crate rand;
extern crate input;

use piston_window::*;
// use std::fmt::Debug;
// use std::io;

mod ball;
mod block;
mod controller;
mod linear_node;
mod draw;
mod stage;

use ball::Ball;
use block::{Hit};
use controller::Controller;
use stage::Stage;
// use linear_node::LinearNode;

const CURSOR_OFFSET_VERTICAL: f64 = 20.0;

enum Status {
    Stop,
    Action,
}
fn main() {
    let width: f64 = 640.0;
    let height: f64 = 480.0;
    let mut window: PistonWindow =
        WindowSettings::new("Breaking blocks", [width as u32, height as u32])
        .exit_on_esc(true).build().unwrap();
    let mut stage = Stage::new(1);
    let mut blocks = stage.gen_blocks(width, height);
    // blocks.add(Block::new_rand(width, height / 2.0));
    let mut ball = Ball::new(width, height);
    let mut controller = Controller::new(width, height);
    let mut status = Status::Stop;
    while let Some(e) = window.next() {
        // Bounce Frame 
        ball.bounce_frame(width, height);
        
        // Bounce Controller
        match controller.touch(&ball) {
            Some(bounce) => {
                ball.dy *= bounce.dy;
                ball.dx = bounce.dx;
            },
            None => {}
        }
        
        // Bounce Block
        for (index, block) in blocks.iter_mut().enumerate() {
            match block.touch(&ball) {
                Some(Hit::Bottom) | Some(Hit::Top) => {
                    ball.dy *= -1.0;
                    // Infinity Block Mode
                    // block.rand(width, height);
                    blocks.remove(index);
                    break;
                },
                Some(Hit::Right) | Some(Hit::Left) => {
                    ball.dx *= -1.0;
                    // Infinity Block Mode
                    // block.rand(width, height);
                    blocks.remove(index);
                    break;
                }
                _ => {}
            }
        }

        // Coordinate change of ball.
        match status {
            Status::Action => {
                ball.effect();
            },
            _ => {}
        }
       

        // Mouse Event Handler
        match e.mouse_cursor_args() {
            Some([mouse_x, mouse_y]) => {
                let window_size = window.size();
                if mouse_x < 1.0 || mouse_y < CURSOR_OFFSET_VERTICAL || mouse_y > window_size.height - CURSOR_OFFSET_VERTICAL || mouse_x > window_size.width - 1.0 {
                    status = Status::Stop;
                } else {
                    status = Status::Action;
                }
                controller.handle_move(mouse_x);
            },
            _ => {}
        }

        // Key Event Handler
        match e.press_args() {
            Some(Button::Keyboard(Key::Up)) => {
                stage.level_up(1);
                blocks = stage.gen_blocks(width, height);
            },
            Some(Button::Keyboard(Key::Down)) => {
                stage.level_down(1);
                blocks = stage.gen_blocks(width, height);
            },
            Some(Button::Keyboard(Key::Left)) => {
                controller.x -= controller.move_speed;
            },
            Some(Button::Keyboard(Key::Right)) => {
                controller.x += controller.move_speed;
            },
            Some(Button::Keyboard(Key::Space)) => {
                ball = Ball::new(width, height);
            }
            _ => {}
        }

        // Window Resize Event Handler
        match e.resize_args() {
            Some(ResizeArgs {
                window_size: [w, h], 
                draw_size: _draw_size 
            }) => {
                controller = Controller::new(w, h);
                for block in &mut blocks {
                    block.rand(width, height / 2.0)
                }
            }
            _ => {}
        }

        // Draw a screen.
        window.draw_2d(&e, |c, g, _device| {
            clear([1.0; 4], g);
             for block in &blocks {
                block.draw(c.transform, g)
            }
            ball.draw(c.transform, g);
            controller.draw(c.transform, g);
            match status {
                Status::Action => {},
                Status::Stop => {}
            }
        });
    }
}


// fn get_keyboard_event() -> io::Result<String> {
//     let mut input = String::new();

//     io::stdin().read_line(&mut input)?;

//     println!("You typed: {}", input.trim());
//     Ok(input)
// }