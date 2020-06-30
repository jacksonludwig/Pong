use opengl_graphics::GlGraphics;
use piston::input::{
    Button, Key, PressEvent, ReleaseEvent, RenderArgs, RenderEvent, UpdateArgs, UpdateEvent,
};
use std::process;

use super::shapes::Square;

const BACKGROUND: [f32; 4] = [0.0, 0.5, 0.5, 1.0];
const FOREGROUND: [f32; 4] = [0.0, 0.0, 1.0, 1.0];

pub struct App {
    pub graphics: GlGraphics,
    pub square: Square,
}

impl App {
    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        let square = rectangle::square(0.0, 0.0, 50.0);
        let rotation = self.square.rotation;
        let (x, y) = (args.window_size[0] / 2.0, args.window_size[1] / 2.0);

        self.graphics.draw(args.viewport(), |context, graphics| {
            clear(BACKGROUND, graphics);

            let transform = context
                .transform
                .trans(x, y)
                .rot_rad(rotation)
                .trans(-25.0, -25.0);

            rectangle(FOREGROUND, square, transform, graphics);
        });
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        self.square.rotation += 2.0 * args.dt;
    }
}
