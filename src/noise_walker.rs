#![deny(missing_docs)]
//! The noise walker example. Same as the random_walker except using perlin (simplex) noise

use utils;

use piston::window;
use piston::window::WindowSettings;
use piston::event_loop::{Events, EventSettings};
use piston::input::{GenericEvent, RenderEvent};
use graphics::{Context, Graphics};
use opengl_graphics::{OpenGL, GlGraphics};
use glutin_window::GlutinWindow;

use noise::{NoiseFn, Seedable, Perlin};

use rand;
use rand::Rng;

/// Runs the noise walker example with the specified amount of walkers
pub fn run(amount: usize) {
    let opengl = OpenGL::V3_2;
    let settings = WindowSettings::new("Noise Walker", [512; 2])
                                                .exit_on_esc(true).opengl(opengl);
    let mut window: GlutinWindow = settings.build().expect("Problem building window");

    let mut events = Events::new(EventSettings::new());
    let mut gl = GlGraphics::new(opengl);

    let mut walkers: Vec<NoiseWalker> = Vec::new();
    for _ in 0..amount {
        walkers.push(NoiseWalker::random(settings.get_size()))
    }

    while let Some(e) = events.next(&mut window) {

        for mut walker in walkers.iter_mut() {
            walker.event(&e)
        }

        if let Some(args) = e.render_args() {
            gl.draw(args.viewport(), |c, g| {

                for walker in walkers.iter() {
                    walker.draw(&c, g);
                }

            })
        }
    }
}

struct NoiseWalker {
    x: f64,
    y: f64,
    xt: f64,
    yt: f64,
    color: [f32; 4],
    seed: u32
}

impl NoiseWalker {
    fn random(size: window::Size) -> NoiseWalker {
        let mut rng = rand::thread_rng();

        let xt: f64 = rng.gen();
        let yt: f64 = rng.gen();

        let perlin = Perlin::new();
        let seed = rng.gen();
        perlin.set_seed(seed);

        let mut x = perlin.get([xt, seed as f64]);
        let mut y = perlin.get([seed as f64, yt]);

        utils::map_to_range(&mut x, 0.0, 1.0, 0.0, size.width as f64);
        utils::map_to_range(&mut y, 0.0, 1.0, 0.0, size.height as f64);

        let mut color = [0.0; 4];
        for mut col in color.iter_mut() {
            *col = rng.gen_range(0.0, 1.0)
        }
        if let Some(alpha) = color.last_mut() {
            *alpha = 1.0;
        }

        NoiseWalker {x, y, xt, yt, color, seed}
    }

    fn event<E: GenericEvent>(&mut self, e: &E) {
        if let Some(args) = e.update_args() {
            self.xt += args.dt / 2.0;
            self.yt += args.dt / 2.0;

            let perlin = Perlin::new();
            perlin.set_seed(self.seed);
            self.x = perlin.get([self.xt, self.seed as f64]);
            self.y = perlin.get([self.seed as f64, self.yt]);

            utils::map_to_range(&mut self.x, -1.0, 1.0, 0.0, 512.0);
            utils::map_to_range(&mut self.y, -1.0, 1.0, 0.0, 512.0);
        }
    }

    fn draw<G: Graphics>(&self, c: &Context, g: &mut G) {
        use graphics::Rectangle;

        Rectangle::new(self.color).draw(
            [self.x, self.y, 1.0, 1.0],
            &c.draw_state,
            c.transform,
            g
        )
    }
}