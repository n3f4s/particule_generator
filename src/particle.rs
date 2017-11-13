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
    max_lifetime: u64, //tick
    pub radius: i16
    //TODO mass
}

impl Particle {
    pub fn new(p: Point3, d: Vec3) -> Particle {
        Particle {
            position: p,
            direction: d,
            alive: true,
            lifetime: 250,
            max_lifetime: 250,
            radius: 5
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
                        self.radius,
                        (255, self.compute_green(), 0, self.compute_alpha())).unwrap();
    }
    fn draw_window(&self, c: &mut Canvas<Window>) {
        c.filled_circle(self.position.x as i16,
                        self.position.y as i16,
                        self.radius,
                        (255, self.compute_green(), 0, self.compute_alpha())).unwrap();
    }
}

pub struct ParticleBuilder {
    template: Particle
}

impl<'a> ParticleBuilder {
    pub fn new(start_pos: Point3, start_dir: Vec3) -> ParticleBuilder {
        ParticleBuilder {
            template: Particle::new(start_pos, start_dir)
        }
    }

    pub fn with_radius(&'a mut self, radius: i16) -> &'a mut ParticleBuilder {
        self.template.radius = radius;
        self
    }
    pub fn with_lifetime(&'a mut self, lifetime: u64) -> &'a mut ParticleBuilder {
        self.template.max_lifetime = lifetime;
        self.template.lifetime = lifetime;
        self
    }
    pub fn create(&self) -> Particle {
        self.template.clone()
    }
}
