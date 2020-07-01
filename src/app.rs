use opengl_graphics::GlGraphics;
use piston::input::{Button, Key, RenderArgs, UpdateArgs};

const BACKGROUND: [f32; 4] = [0.18, 0.23, 0.28, 1.0];
const FOREGROUND: [f32; 4] = [0.46, 0.31, 0.32, 1.0];
const PADDLE_SPEED: i32 = 15;
const BALL_SPEED: i32 = 9;
const BALL_START_X: i32 = 50;
const BALL_START_Y: i32 = 50;

pub struct Game {
    pub graphics: GlGraphics,
    pub paddle: Paddle,
    pub ball: Ball,
}

impl Game {
    pub fn render(&mut self, rend_args: &RenderArgs) {
        self.graphics.draw(rend_args.viewport(), |_context, graph| {
            graphics::clear(BACKGROUND, graph);
        });
        self.paddle.render(&mut self.graphics, rend_args);
        self.ball.render(&mut self.graphics, rend_args);
    }

    pub fn update(&mut self, _args: &UpdateArgs) {
        self.paddle.update(&self.ball);
        self.ball.update(&self.paddle);
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

trait PosSharer {
    fn share_x(&self) -> i32;
    fn share_y(&self) -> i32;
}

trait Renderer {
    fn render(&mut self, graph: &mut GlGraphics, rend_args: &RenderArgs);
    fn update<T: PosSharer>(&mut self, object: &T);
}

pub struct Ball {
    pub x_pos: i32,
    pub y_pos: i32,
    pub x_velocity: i32,
    pub y_velocity: i32,
    pub max_dist_x: i32,
    pub max_dist_y: i32,
}

impl Ball {
    pub fn new() -> Ball {
        Ball {
            x_pos: 0,
            y_pos: 0,
            x_velocity: BALL_SPEED,
            y_velocity: BALL_SPEED,
            max_dist_x: 100,
            max_dist_y: 100,
        }
    }
}

impl PosSharer for Ball {
    fn share_x(&self) -> i32 {
        self.x_pos
    }

    fn share_y(&self) -> i32 {
        self.y_pos
    }
}

impl Renderer for Ball {
    fn render(&mut self, graph: &mut GlGraphics, rend_args: &RenderArgs) {
        use graphics::rectangle;
        self.max_dist_x = rend_args.window_size[0] as i32;
        self.max_dist_y = rend_args.window_size[1] as i32;
        let square = graphics::rectangle::square(self.x_pos as f64, self.y_pos as f64, 20.0);

        graph.draw(rend_args.viewport(), |context, graph| {
            let transform = context.transform;
            rectangle(FOREGROUND, square, transform, graph);
        });
    }

    fn update<T: PosSharer>(&mut self, object: &T) {
        self.x_pos += self.x_velocity;
        self.y_pos += self.y_velocity;
        if self.y_pos > self.max_dist_y {
            let pos_difference = self.x_pos - object.share_x();
            if pos_difference.abs() < 75 {
                self.y_velocity = -self.y_velocity;
            } else {
                self.x_pos = BALL_START_X;
                self.y_pos = BALL_START_Y;
            }
        }
        if self.x_pos < 1 || self.x_pos > self.max_dist_x {
            self.x_velocity = -self.x_velocity;
        }

        if self.y_pos < 1 {
            self.y_velocity = -self.y_velocity;
        }
    }
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
}

impl PosSharer for Paddle {
    fn share_x(&self) -> i32 {
        self.x_pos
    }

    fn share_y(&self) -> i32 {
        self.y_pos
    }
}

impl Renderer for Paddle {
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

    fn update<T: PosSharer>(&mut self, _object: &T) {
        if (self.x_velocity == PADDLE_SPEED && self.x_pos < self.max_dist)
            || (self.x_velocity == -PADDLE_SPEED && self.x_pos >= 1)
        {
            self.x_pos += self.x_velocity;
        }
    }
}
