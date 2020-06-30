#![allow(dead_code)]

extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

mod app;
mod shapes;

use glutin_window::GlutinWindow;
use opengl_graphics::GlGraphics;
use opengl_graphics::OpenGL;
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderEvent, UpdateEvent};
use piston::window::WindowSettings;

use app::App;
use shapes::Square;

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: GlutinWindow = WindowSettings::new("Pong", [200, 200])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut app = App {
        graphics: GlGraphics::new(opengl),
        square: Square { rotation: 0.0 },
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }
    }
}
