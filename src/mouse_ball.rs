use utils::{Drawable, Updatable};

use piston::window;
use piston::window::WindowSettings;
use piston::input::{GenericEvent, RenderEvent, RenderArgs};
use piston::event_loop::{Events, EventSettings};
use graphics::{Graphics, Context};
use opengl_graphics::{OpenGL, GlGraphics};
use glutin_window::GlutinWindow;

use cgmath::prelude::*;
use cgmath::Vector2;

use rand;
use rand::distributions::{Normal, IndependentSample};
use rand::Rng;

pub fn run(amount: usize) {
    let opengl = OpenGL::V3_2;
    let settings = WindowSettings::new("Mouse Ball", [512; 2])
        .exit_on_esc(true).opengl(opengl);
    let mut window: GlutinWindow = settings.build().expect("Could not build window");

    let mut events = Events::new(EventSettings::new());
    let mut gl = GlGraphics::new(opengl);

    let mut balls: Vec<MouseBall> = Vec::new();
    for _ in 0..amount {
        balls.push(MouseBall::random(settings.get_size()));
    }

    while let Some(e) = events.next(&mut window) {

        for mut ball in balls.iter_mut() {
            ball.update(&e);
        }

        if let Some(args) = e.render_args() {
            gl.draw(args.viewport(), |c, g| {
                use graphics;

                for ball in balls.iter() {
                    ball.draw(&args, &c, g)
                }

                graphics::clear([0.0, 0.0, 0.0, 1.0], g);
            })
        }
    }
}

struct MouseBall {
    position: Vector2<f64>,
    velocity: Vector2<f64>,
    acceleration: Vector2<f64>,
    force: Vector2<f64>,
    mass: f64,
    color: [f32; 4],
    cursor_pos: Vector2<f64>
}

impl MouseBall {
    fn random(size: window::Size) -> MouseBall{
        let mut rng = rand::thread_rng();
        let normal = Normal::new(12.0, 8.0);

        let position = Vector2::new(rng.gen_range(0.0, size.width as f64), rng.gen_range(0.0, size.height as f64));

        let velocity = Vector2::zero();
        let acceleration = Vector2::zero();
        let force = Vector2::zero();
        let mass = normal.ind_sample(&mut rng).abs();

        let mut color = [0.0; 4];
        for mut col in color.iter_mut() {
            *col = rng.gen_range(0.0, 1.0);
        }
        if let Some(alpha) = color.last_mut() {
            *alpha = 1.0;
        }


        MouseBall {position, velocity, acceleration, force, mass, color, cursor_pos: Vector2::zero()}
    }
}

impl Drawable for MouseBall {
    fn draw<G: Graphics>(&self, _args: &RenderArgs, c: &Context, g: &mut G) {
        use graphics::Ellipse;

        Ellipse::new(self.color)
            .draw([self.position.x, self.position.y, self.mass, self.mass], &c.draw_state, c.transform, g);
    }
}

impl  Updatable for MouseBall {
    fn update<E: GenericEvent>(&mut self, e: &E) {
        if let Some(pos) = e.mouse_cursor_args() {
            self.cursor_pos = Vector2::new(pos[0], pos[1]);
        } else if let Some(args) = e.update_args() {
            self.force =  (self.cursor_pos - self.position) * 10.0;

            self.acceleration = self.force / self.mass;
            self.velocity += self.acceleration * args.dt;
            if self.velocity.magnitude() > 500.0 {
                self.velocity = self.velocity.normalize_to(500.0);
            }
            self.position += self.velocity * args.dt;
        }

    }
}