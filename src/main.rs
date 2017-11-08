extern crate rand;
extern crate sdl2;

mod vec3;

use vec3::Vec3;
use vec3::unit_vector;
use std::vec::Vec;
use std::boxed::Box;

use rand::{thread_rng, Rng};
use sdl2::image::{INIT_PNG, INIT_JPG};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::pixels::Color;

// TODO add more flexibility (more complex gravity well, particle mass, ...)
// TODO add collision ???
// TODO draw some properties

#[derive(Debug, Default, PartialEq, Copy, Clone)]
pub struct Point3 {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl Point3 {
    pub fn new(x: f64, y: f64, z: f64) -> Point3 {
        Point3 {
            x: x,
            y: y,
            z: z
        }
    }
    pub fn apply_vec(&mut self, v: Vec3) {
        self.x += v.x;
        self.y += v.y;
        self.z += v.z;
    }
}

#[derive(Debug, Default, PartialEq, Copy, Clone)]
pub struct Particle {
    pub position: Point3,
    pub direction: Vec3,
    pub alive: bool,
    pub lifetime: u64 //tick
    //TODO lifetime of the particle ?
    //TODO raduis
    //TODO mass
}

impl Particle {
    fn new(p: Point3, d: Vec3) -> Particle {
        Particle {
            position: p,
            direction: d,
            alive: true,
            lifetime: 100
        }
    }
    fn update(&mut self) {
        if self.alive {
            self.position.apply_vec(self.direction/10.0);
            self.lifetime -= 1;
            if self.lifetime == 0 {
                self.alive = false;
            }
        }
    }
    fn apply_force(&mut self, f: Vec3) {
        self.direction += f
    }
}

pub trait PhysicProperty {
    fn update_particle(&self, p: &mut Particle);
}

#[derive(Debug, Default, PartialEq, Copy, Clone)]
pub struct Rectangle {
    pub up_left_corner: Point3,
    pub height: f64,
    pub width: f64,
    pub depth: f64
}

impl Rectangle {
    fn is_in_bound(&self, p: &Point3) -> bool {
        p.x >= self.up_left_corner.x &&
            p.y >= self.up_left_corner.y &&
            p.z >= self.up_left_corner.z &&
            p.x <= self.up_left_corner.x + self.width &&
            p.y <= self.up_left_corner.y + self.height &&
            p.z <= self.up_left_corner.z + self.depth
    }

    fn center(&self) -> Point3 {
        Point3 {x: self.up_left_corner.x + self.width/2.0, y: self.up_left_corner.y + self.height/2.0, z: self.up_left_corner.z + self.depth/2.0}
    }
}

pub struct World {
    pub particles: Vec<Particle>,
    pub properties: Vec<Box<PhysicProperty>>,
    pub boundaries: Rectangle,

    particle_creation_point: Point3,
    create_particle_fun: Box<Fn(Point3) -> Particle>
}

impl World {
    fn new(pr: Vec<Box<PhysicProperty>>, b: Rectangle, c: Point3, f: Box<Fn(Point3) -> Particle>) -> World {
        World {
            particles: vec![],
            properties: pr,
            boundaries: b,
            particle_creation_point: c,
            create_particle_fun: f
        }
    }
    fn update(&mut self) {
        let mut cpt = 0;
        for mut p in &mut self.particles {
            for prop in &self.properties {
                prop.update_particle(&mut p)
            }
            p.update();
            if ! self.boundaries.is_in_bound(&p.position) {
                p.alive = false
            };
            if ! p.alive {
                cpt += 1;
            }
        }
        if cpt >= (self.particles.len() / 2) {
            self.particles.retain(|&x| x.alive);
        }
    }

    fn create_particle(&mut self) {
        self.particles.push((self.create_particle_fun)(self.particle_creation_point).clone());
    }
}

pub struct Gravity {
}

impl PhysicProperty for Gravity {
    fn update_particle(&self, p: &mut Particle) {
        p.apply_force(Vec3::new(0.0, 1.0, 0.0));
    }
}
pub struct Wind {
}

impl PhysicProperty for Wind {
    fn update_particle(&self, p: &mut Particle) {
        p.apply_force(Vec3::new(-0.25, 0.0, 0.0));
    }
}
pub struct AirResistance {
}

impl PhysicProperty for AirResistance {
    fn update_particle(&self, p: &mut Particle) {
        let density = 1.0; // air density
        let drag = 0.20; // drag coeficient (magic number here)
        let area = 1.0; // area affected by the air resistance, compute using radius of sphere
        let mut next_point = p.clone();
        next_point.update();
        let speed = (((next_point.position.x - p.position.x) *
                      (next_point.position.x - p.position.x)) +
                     ((next_point.position.y - p.position.y) *
                      (next_point.position.y - p.position.y))).sqrt();
        let f = ((density * drag * area) / 2.0) * speed;
        let unit_v = unit_vector(p.direction);
        p.apply_force(-1.0 * f * unit_v);
    }
}
#[derive(Debug, Default, PartialEq, Copy, Clone)]
pub struct GravityWell {
    pub position: Point3,
    pub strength: f64,
    pub area_of_effect: f64,
}
impl GravityWell {
    fn new(p: Point3, s: f64, aoe: f64) -> GravityWell {
        GravityWell {
            position: p,
            strength: s,
            area_of_effect: aoe
        }
    }
}
impl PhysicProperty for GravityWell {
    fn update_particle(&self, p: &mut Particle) {
        let dist = ((self.position.x - p.position.x) *
                    (self.position.x - p.position.x)) +
                   ((self.position.y - p.position.y) *
                   (self.position.y - p.position.y));
        let aoe = self.area_of_effect;
        let aoe2 = aoe * 2.0;
        let aoe3 = aoe * 3.0;

        let vec = Vec3{ x: p.position.x - self.position.x,
                        y: p.position.y - self.position.y,
                        z: p.position.z - self.position.z};

        if dist < (aoe * aoe) {
            p.apply_force(vec * -self.strength);
        } else if dist < (aoe2 * aoe2) {
            p.apply_force(vec * -(self.strength/2.0));
        } else if dist < (aoe3 * aoe3) {
            p.apply_force(vec * -(self.strength/3.0));
        }
    }
}

fn main() {
    let bound = Rectangle{up_left_corner: Point3::new(0.0,0.0,0.0), height: 1060.0, width: 1900.0, depth: 0.0};
    let mut world = World::new(vec![Box::new(Gravity{})],
                               bound,
                               bound.center(),
                               Box::new(move |p: Point3| {
                                   // meh
                                   let mut rng = thread_rng();
                                   Particle {
                                       position: p,
                                       direction: Vec3::new(rng.gen_range(-10.0, 10.0),
                                                            rng.gen_range(-15.0, -5.0),
                                                            0.0),
                                       alive: true
                                   }
                               } ));
    // SDL
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let _image_context = sdl2::image::init(INIT_PNG | INIT_JPG).unwrap();
    let window = video_subsystem.window("rust-sdl2 demo: Video", 1900, 1060)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().software().build().unwrap();
    let red = (255, 0, 0, 255);
    let rad = 5;

    //let mut fps = FPSManager::new();
    //fps.set_framerate(10);

    'mainloop: loop {
        for event in sdl_context.event_pump().unwrap().poll_iter() {
            match event {
                Event::Quit{..} |
                Event::KeyDown {keycode: Option::Some(Keycode::Escape), ..} =>
                    break 'mainloop,
                Event::KeyDown {keycode: Option::Some(Keycode::Space), ..} => {
                    world.create_particle();
                    println!("create particle !");
                }
                _ => {}
            }
        }
        world.update();
        canvas.clear();
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        for p in &world.particles {
            if p.alive {
                println!("Draw particle at position {} {}", p.position.x, p.position.y);
                canvas.filled_circle(p.position.x as i16, p.position.y as i16, rad, red);
            }
        }
        canvas.present();
    }
}
