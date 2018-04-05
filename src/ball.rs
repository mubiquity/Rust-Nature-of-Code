//#![deny(missing_docs)]
//! Balls that bounce around with acceleration

use utils::{Drawable, Updatable};

use piston::window;
use piston::window::WindowSettings;
use piston::event_loop::{Events, EventSettings};
use piston::input::{GenericEvent, RenderEvent, RenderArgs};
use opengl_graphics::{OpenGL, GlGraphics};
use graphics::{Context, Graphics};
use glutin_window::GlutinWindow;

use rulinalg::vector::Vector;

use rand;
use rand::Rng;

/// Runs the example with the specified number of balls
pub fn run(amount: usize) {
    let opengl = OpenGL::V3_2;
    let settings = WindowSettings::new("Bouncing Balls", [512; 2])
                                    .exit_on_esc(true).opengl(opengl);
    let mut window: GlutinWindow = settings.build().expect("Unable to build window");

    let mut events = Events::new(EventSettings::new());
    let mut gl = GlGraphics::new(opengl);

    let mut balls: Vec<Ball> = Vec::new();
    for _ in 0..amount {
        balls.push(Ball::random(settings.get_size()));
    }

    while let Some(e) = events.next(&mut window) {

        for mut ball in balls.iter_mut() {
            ball.update(&e);
        }

        if let Some(args) = e.render_args() {
            gl.draw(args.viewport(), |c, g| {
                use graphics::clear;

                for mut ball in balls.iter() {
                    ball.draw(&args, &c, g);
                }

                clear([0.0, 0.0, 0.0, 1.0], g);
            })
        }
    }

}

struct Ball {
    pos: Vector<f64>,
    velocity: Vector<f64>,
    acceleration: Vector<f64>,
    color: [f32; 4],
    screen_size: [u32; 2]
}

impl Ball {
    fn random(size: window::Size) -> Ball {
        let mut rng = rand::thread_rng();

        let pos = vector![rng.gen_range(0.0, size.width as f64), rng.gen_range(0.0, size.height as f64)];
        let velocity = Vector::zeros(2);
        let acceleration = Vector::from_fn(2, |_| rng.gen_range(-0.5, 0.5));

        let mut color: [f32; 4] = [0.0; 4];
        for mut col in color.iter_mut() {
            *col = rng.gen_range(0.0, 1.0);
        }

        let screen_size = [size.width, size.height];

        Ball {pos, velocity, acceleration, color, screen_size}
    }
}

impl Drawable for Ball {
    fn draw<G: Graphics>(&self, _args: &RenderArgs, c: &Context, g: &mut G) {
        use graphics::Ellipse;

        Ellipse::new(self.color)
            .draw([self.pos[0], self.pos[1], 10.0 * self.color[3] as f64, 10.0 * self.color[3] as f64], &c.draw_state, c.transform, g);
    }
}

// TODO: Make a struct for .x and .y access to elements
impl Updatable for Ball {
    fn update<E: GenericEvent>(&mut self, e: &E) {
        if let Some(args) = e.update_args() {
            self.velocity += &self.acceleration * args.dt;
            self.pos += &self.velocity;

            if self.pos[0] > self.screen_size[0] as f64 || self.pos[0] < 0.0 {
                self.velocity[0] = -1.0 * &self.velocity[0];
                self.acceleration[0] = -1.0 * &self.acceleration[0];
            }
            if self.pos[1] > self.screen_size[1] as f64 || self.pos[1] < 0.0 {
                self.velocity[1] = -1.0 * &self.velocity[1];
                self.acceleration[1] = -1.0 * &self.acceleration[1];
            }


        } else if let Some(size) = e.resize_args() {
            self.screen_size = size;
        }
    }

}