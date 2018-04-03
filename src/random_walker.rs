#![deny(missing_docs)]
//! "walks" around the screen choosing random directions.

// TODO: Remove
#![allow(dead_code)]

use piston::window;
use piston::window::WindowSettings;
use piston::event_loop::{Events, EventSettings, EventLoop};
use piston::input::{GenericEvent, RenderEvent};
use graphics::{Context, Graphics};
use glutin_window::GlutinWindow;
use opengl_graphics::{OpenGL, GlGraphics};
use rand;
use rand::Rng;

/// Runs the random walker example with the specified settings
pub fn run(amount: i32) {
    let opengl = OpenGL::V3_2;
    let settings = WindowSettings::new("Random Walker", [512; 2])
        .exit_on_esc(true).opengl(opengl);
    let mut window: GlutinWindow = settings.build().expect("Could not build window");

    let mut events = Events::new(EventSettings::new());
    let mut gl = GlGraphics::new(opengl);

    let mut walkers: Vec<Walker> = Vec::new();
    for _ in 0..amount {
        let mut walker = Walker::random(settings.get_size());
        walkers.push(walker);
    }

    while let Some(e) = events.next(&mut window) {

        for mut walker in walkers.iter_mut() {
            walker.event(&e);
        }

        if let Some(args) = e.render_args() {
            gl.draw(args.viewport(), |c, g|{
                for walker in walkers.iter() {
                    walker.draw(&c, g)
                }
            })
        }
    }
}

struct Walker {
    x: f64,
    y: f64,
    color: [f32; 4]
}

impl Walker {
    fn random(size: window::Size) -> Walker {
        let mut rng = rand::thread_rng();

        let x = rng.gen_range(0.0, size.width as f64);
        let y = rng.gen_range(0.0, size.height as f64);

        let mut color = [0.0; 4];
        for mut val in color.iter_mut() {
            *val = rng.gen_range(0.0, 1.0);
        }

        Walker {x, y, color}
    }

    fn event<E: GenericEvent>(&mut self, e: &E) {
        if let Some(_) = e.update_args() {
            match Direction::random() {
                Direction::Left => self.x += -1.0,
                Direction::Right => self.x += 1.0,
                Direction::Up => self.y += 1.0,
                Direction::Down => self.y += -1.0
            }
        }
    }

    fn draw<G: Graphics>(&self, c: &Context, g: &mut G) {
        use graphics::Rectangle;

        Rectangle::new(self.color)
            .draw([self.x, self.y, 1.0, 1.0], &c.draw_state, c.transform, g);
    }
}

enum Direction {
    Left,
    Up,
    Right,
    Down
}

impl Direction {
    fn random() -> Direction {
        let mut rng = rand::thread_rng();

        match rng.gen_range(0, 4){
            0 => Direction::Left,
            1 => Direction::Right,
            2 => Direction::Up,
            3 => Direction::Down,
            _ => Direction::Left // To stop rusts complaints that it is non-exhaustive
        }
    }
}