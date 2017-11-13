use vec3::Vec3;
use point3::Point3;
use drawable::Drawable;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::surface::Surface;
use sdl2::gfx::primitives::DrawRenderer;
//use std::cmp::max;

#[derive(Debug, Default, PartialEq, Copy, Clone)]
pub struct Particle {
    pub position: Point3,
    pub direction: Vec3,
    pub alive: bool,
    pub lifetime: u64, //tick
    max_lifetime: u64 //tick
    //TODO lifetime of the particle ?
    //TODO raduis
    //TODO mass
}

impl Particle {
    pub fn new(p: Point3, d: Vec3) -> Particle {
        Particle {
            position: p,
            direction: d,
            alive: true,
            lifetime: 250,
            max_lifetime: 250
        }
    }
    pub fn update(&mut self) {
        if self.alive {
            self.position.apply_vec(self.direction/10.0);
            self.lifetime -= 1;
            if self.lifetime == 0 {
                self.alive = false;
            }
        }
    }
    pub fn apply_force(&mut self, f: Vec3) {
        self.direction += f
    }
    fn compute_alpha(&self) -> u8 {
        // max(
        //     ((self.lifetime as f64) / (self.max_lifetime as f64) * 255.0) as u8,
        //     125)
        ((self.lifetime as f64) / (self.max_lifetime as f64) * (self.lifetime as f64) / (self.max_lifetime as f64) * 255.0) as u8
    }
    fn compute_green(&self) -> u8 {
        (
            (
                1.0 - (((self.lifetime as f64) / (self.max_lifetime as f64) ) * ( (self.lifetime as f64) / (self.max_lifetime as f64)))
            ) * 255.0
        ) as u8
    }
}

impl Drawable for Particle {
    fn draw_surface(&self, c: &mut Canvas<Surface>) {
        c.filled_circle(self.position.x as i16,
                        self.position.y as i16,
                        5,
                        (255, self.compute_green(), 0, self.compute_alpha())).unwrap();
    }
    fn draw_window(&self, c: &mut Canvas<Window>) {
        c.filled_circle(self.position.x as i16,
                        self.position.y as i16,
                        5,
                        (255, self.compute_green(), 0, self.compute_alpha())).unwrap();
    }
}
