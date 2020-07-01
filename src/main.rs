#![allow(dead_code)]

extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

mod app;

use glutin_window::GlutinWindow;
use opengl_graphics::GlGraphics;
use opengl_graphics::OpenGL;
use piston::event_loop::{EventSettings, Events};
use piston::input::{PressEvent, ReleaseEvent, RenderEvent, UpdateEvent};
use piston::window::WindowSettings;

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: GlutinWindow = WindowSettings::new("Pong", [1280, 720])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut game = app::Game {
        graphics: GlGraphics::new(opengl),
        paddle: app::Paddle::new(),
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            game.render(&args);
        }

        if let Some(args) = e.update_args() {
            game.update(&args);
        }

        if let Some(args) = e.press_args() {
            game.pressed(&args);
        }

        if let Some(args) = e.release_args() {
            game.released(&args);
        }
    }
}
