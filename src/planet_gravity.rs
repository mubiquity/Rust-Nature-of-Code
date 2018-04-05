use utils::{Updatable, Drawable};

use piston::window;
use piston::window::WindowSettings;
use piston::event_loop::{Events, EventSettings};
use piston::input::{GenericEvent, RenderEvent, RenderArgs};
use glutin_window::GlutinWindow;
use opengl_graphics::{OpenGL, GlGraphics};
use graphics::{Context, Graphics};

use rand;
use rand::Rng;
use cgmath::prelude::*;
use cgmath::Vector2;
use snowflake::ProcessUniqueId;

pub fn run(amount: usize) {
    let opengl = OpenGL::V3_2;
    let settings = WindowSettings::new("Planet Gravity", [1280, 720])
        .exit_on_esc(true).opengl(opengl);
    let mut window: GlutinWindow = settings.build().expect("Failed to build window");

    let mut events = Events::new(EventSettings::new());
    let mut gl = GlGraphics::new(opengl);

    let mut planets = PlanetPool::random(amount, settings.get_size());

    while let Some(e) = events.next(&mut window) {

        planets.update(&e);

        if let Some(args) = e.render_args() {
            gl.draw(args.viewport(), |c, g| {
                use graphics::clear;

                planets.draw(&args, &c, g);

                clear([0.0, 0.0, 0.0, 1.0], g);
            })
        }
    }
}

struct PlanetPool {
    planets: Vec<Planet>
}

impl PlanetPool {
    fn random(amount: usize, size: window::Size) -> PlanetPool {
        let mut planets = Vec::new();
        for _ in 0..amount {
            planets.push(Planet::random(size));
        }

        planets.push(Planet::debug(size.width as f64 / 2.0, size.height as f64 / 2.0, 100.0));

        PlanetPool { planets }

    }
}

impl Updatable for PlanetPool {
    fn update<E: GenericEvent>(&mut self, e: &E) {
        // TODO: Better way than cloning??
        let parent_clone = self.planets.clone();
        if let Some(_args) = e.update_args() {
            for planet_a in self.planets.iter_mut() {
                for planet_b in parent_clone.iter() {
                    if planet_a.id != planet_b.id {
                        planet_a.add_planet_force(planet_b);
                    }
                }
            }
        }

        for mut planet in self.planets.iter_mut() {
            planet.update(e);
        }
    }
}

impl  Drawable for PlanetPool {
    fn draw<G: Graphics>(&self, args: &RenderArgs, c: &Context, g: &mut G) {
        for planet in self.planets.iter() {
            planet.draw(args, c, g);
        }
    }
}

#[derive(Clone)]
struct Planet {
    id: ProcessUniqueId,
    pos: Vector2<f64>,
    vel: Vector2<f64>,
    acc: Vector2<f64>,
    force: Vector2<f64>,
    mass: f64,
    color: [f32; 4],
}

impl Planet {
    fn debug(x: f64, y: f64, mass: f64) -> Planet {
        Planet {
            id: ProcessUniqueId::new(),
            pos: Vector2::new(x, y),
            vel: Vector2::zero(),
            acc: Vector2::zero(),
            force: Vector2::zero(),
            mass,
            color: [1.0; 4],
        }
    }

    fn random(size: window::Size) -> Planet {
        let mut rng = rand::thread_rng();

        let pos = Vector2::new(rng.gen_range(0.0, size.width as f64), rng.gen_range(0.0, size.height as f64));
        let vel = Vector2::zero();
        let acc = Vector2::zero();
        let force = Vector2::zero();

        let mut color = [1.0; 4];
        for mut col in color.iter_mut() {
            *col = rng.gen_range(0.0, 1.0)
        }
        if let Some(col) = color.last_mut() {
            *col = 1.0;
        }


        Planet {
            id: ProcessUniqueId::new(),
            pos,
            vel,
            acc,
            force,
            mass: rng.gen_range(1.0, 15.0),
            color,
        }

    }

    fn add_planet_force(&mut self, other: &Planet) {
        let self_to_other = other.pos - self.pos;
//        let self_to_other = Vector2::new(1280.0/2.0, 720.0/2.0) - self.pos;
        let mut m2 = self_to_other.magnitude2();

        if m2 < 25.0 {m2 = 25.0} else if m2 > 625.0 {m2 = 625.0}
        println!("{}", m2);


        let force_mag = (self.mass * other.mass) / m2;

        self.force += self_to_other.normalize_to(force_mag);
    }
}

impl Updatable for Planet {
    fn update<E: GenericEvent>(&mut self, e: &E) {
        if let Some(args) = e.update_args() {
            self.acc += self.force / self.mass;
            self.vel += self.acc * args.dt;
            self.pos += self.vel;
            self.force = Vector2::zero();
        }
    }
}

impl Drawable for Planet {
    fn draw<G: Graphics>(&self, args: &RenderArgs, c: &Context, g: &mut G) {
        use graphics::Ellipse;

        Ellipse::new(self.color)
            .draw([self.pos.x - self.mass / 2.0, self.pos.y - self.mass / 2.0, self.mass, self.mass], &c.draw_state, c.transform, g);
    }
}

