use opengl_graphics::GlGraphics;
use piston::input::{
    Button, Key, PressEvent, ReleaseEvent, RenderArgs, RenderEvent, UpdateArgs, UpdateEvent,
};
use std::process;

const BACKGROUND: [f32; 4] = [0.0, 0.5, 0.5, 1.0];
const FOREGROUND: [f32; 4] = [0.0, 0.0, 1.0, 1.0];

pub struct Game {
    pub graphics: GlGraphics,
    pub paddle: Paddle,
}

impl Game {
    pub fn render(&mut self, rend_args: &RenderArgs) {
        self.graphics.draw(rend_args.viewport(), |_context, graph| {
            graphics::clear(BACKGROUND, graph);
        });
        self.paddle.render(&mut self.graphics, rend_args);
    }

    pub fn update(&mut self) {
        self.paddle.update();
    }
}

pub enum Direction {
    Left,
    Right,
}

pub struct Paddle {
    pub x_pos: i32,
    pub y_pos: i32,
    pub x_velocity: i32,
    pub y_velocity: i32,
    pub direction: Direction,
}

impl Paddle {
    pub fn new() -> Paddle {
        Paddle {
            x_pos: 0,
            y_pos: 0,
            x_velocity: 10,
            y_velocity: 10,
            direction: Direction::Right,
        }
    }

    fn render(&self, graph: &mut GlGraphics, rend_args: &RenderArgs) {
        use graphics::{rectangle, Transformed};

        let rect = graphics::rectangle::square(self.x_pos as f64, self.y_pos as f64, 50.0);

        graph.draw(rend_args.viewport(), |context, graph| {
            let transform = context.transform;
            rectangle(
                FOREGROUND,
                rect,
                transform.trans(-40.0, self.x_pos as f64),
                graph,
            );
        });
    }

    fn update(&mut self) {}
}
