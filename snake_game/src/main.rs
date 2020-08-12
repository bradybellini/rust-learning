extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;

struct Game {
    gl: GlGraphics,
    snake: Snake,
}

impl Game {
    fn render(&mut self, arg: &RenderArgs) {
        use graphics::*;
        
        let GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        
        self.gl.draw(arg.viewport(), |_c, gl| {
            graphics::clear(GREEN, gl);
        });
    }
}

struct Snake {
    pos_x: i32,
    pos_y: i32,
}

impl Snake {
    fn render(&self, gl: &mut GlGraphics, arg: &RenderArgs) {
        use graphics;
        
    }
}

fn main() {
    let opengl = OpenGL::V2_1;
    
    let mut window: GlutinWindow = WindowSettings::new(
        "Snake Game",
        [200, 200]
    ).graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();
        
    let mut game = Game {
        gl: GlGraphics::new(opengl)
    };
        
    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            game.render(&r);
        }
    }
}
