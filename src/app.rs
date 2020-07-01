use opengl_graphics::GlGraphics;
use piston::input::{Button, Key, RenderArgs, UpdateArgs};

const BACKGROUND: [f32; 4] = [0.0, 0.5, 0.5, 1.0];
const FOREGROUND: [f32; 4] = [0.0, 0.0, 1.0, 1.0];
const PADDLE_SPEED: i32 = 10;

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

    pub fn update(&mut self, _args: &UpdateArgs) {
        self.paddle.update();
    }

    pub fn pressed(&mut self, args: &Button) {
        if let &Button::Keyboard(key) = args {
            match key {
                Key::Left => {
                    self.paddle.x_velocity = -PADDLE_SPEED;
                }
                Key::Right => {
                    self.paddle.x_velocity = PADDLE_SPEED;
                }
                _ => {}
            }
        }
    }

    pub fn released(&mut self, args: &Button) {
        if let &Button::Keyboard(key) = args {
            match key {
                Key::Left => {
                    self.paddle.x_velocity = 0;
                }
                Key::Right => {
                    self.paddle.x_velocity = 0;
                }
                _ => {}
            }
        }
    }
}

pub struct Ball {
    // TODO
}

pub struct Paddle {
    pub x_pos: i32,
    pub y_pos: i32,
    pub x_velocity: i32,
    pub max_dist: i32,
}

impl Paddle {
    pub fn new() -> Paddle {
        Paddle {
            x_pos: 0,
            y_pos: 0,
            x_velocity: 0,
            max_dist: 100,
        }
    }

    fn render(&mut self, graph: &mut GlGraphics, rend_args: &RenderArgs) {
        use graphics::{rectangle, Transformed};

        self.max_dist = rend_args.window_size[0] as i32;
        self.y_pos = (rend_args.window_size[1] + 20.0) as i32;
        let rect = graphics::rectangle::square(self.x_pos as f64, self.y_pos as f64, 100.0);

        graph.draw(rend_args.viewport(), |context, graph| {
            let transform = context.transform.trans(-50.0, -38.0);
            rectangle(FOREGROUND, rect, transform, graph);
        });
    }

    fn update(&mut self) {
        if (self.x_velocity == PADDLE_SPEED && self.x_pos < self.max_dist)
            || (self.x_velocity == -PADDLE_SPEED && self.x_pos >= 1)
        {
            self.x_pos += self.x_velocity;
        }
    }
}
