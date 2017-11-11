use particle::Particle;
use vec3::Vec3;
use vec3::unit_vector;
use point3::Point3;
use drawable::Drawable;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::surface::Surface;
use sdl2::gfx::primitives::DrawRenderer;

pub trait PhysicProperty : Send + Sync {
    fn update_particle(&self, p: &Particle) -> Particle;
    fn as_drawable(&self) -> Option<&Drawable>;
}
pub struct Gravity {
}

impl PhysicProperty for Gravity {
    fn update_particle(&self, p: &Particle) -> Particle {
        let mut tmp = p.clone();
        tmp.apply_force(Vec3::new(0.0, 1.0, 0.0));
        tmp
    }
    fn as_drawable(&self) -> Option<&Drawable> {
        None
    }
}
pub struct Wind {
}

impl PhysicProperty for Wind {
    fn update_particle(&self, p: &Particle) -> Particle {
        let mut tmp = p.clone();
        tmp.apply_force(Vec3::new(-0.25, 0.0, 0.0));
        tmp
    }
    fn as_drawable(&self) -> Option<&Drawable> {
        None
    }
}
pub struct AirResistance {
}

impl PhysicProperty for AirResistance {
    fn update_particle(&self, p: &Particle) -> Particle {
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
        let mut tmp = p.clone();
        tmp.apply_force(-1.0 * f * unit_v);
        tmp
    }
    fn as_drawable(&self) -> Option<&Drawable> {
        None
    }
}
#[derive(Debug, Default, PartialEq, Copy, Clone)]
pub struct GravityWell {
    pub position: Point3,
    pub strength: f64,
    pub area_of_effect: f64,
}
impl GravityWell {
    pub fn new(p: Point3, s: f64, aoe: f64) -> GravityWell {
        GravityWell {
            position: p,
            strength: s,
            area_of_effect: aoe
        }
    }
}
impl Drawable for GravityWell {
    fn draw_surface(&self, c: &mut Canvas<Surface>) {
        c.filled_circle(self.position.x as i16,
                        self.position.y as i16,
                        (self.area_of_effect * 3.0) as i16,
                        (0, 0, 255, 100)
        ).unwrap();
        c.filled_circle(self.position.x as i16,
                        self.position.y as i16,
                        (self.area_of_effect * 2.0) as i16,
                        (0, 0, 255, 150)
        ).unwrap();
        c.filled_circle(self.position.x as i16,
                        self.position.y as i16,
                        self.area_of_effect as i16,
                        (0, 0, 255, 200)
        ).unwrap();
        c.filled_circle(self.position.x as i16,
                        self.position.y as i16,
                        5,
                        (0, 0, 255, 255)
        ).unwrap();
    }
    fn draw_window(&self, c: &mut Canvas<Window>) {
        c.filled_circle(self.position.x as i16,
                        self.position.y as i16,
                        (self.area_of_effect * 3.0) as i16,
                        (0, 0, 255, 100)
        ).unwrap();
        c.filled_circle(self.position.x as i16,
                        self.position.y as i16,
                        (self.area_of_effect * 2.0) as i16,
                        (0, 0, 255, 150)
        ).unwrap();
        c.filled_circle(self.position.x as i16,
                        self.position.y as i16,
                        self.area_of_effect as i16,
                        (0, 0, 255, 200)
        ).unwrap();
        c.filled_circle(self.position.x as i16,
                        self.position.y as i16,
                        5,
                        (0, 0, 255, 255)
        ).unwrap();
    }
}
impl PhysicProperty for GravityWell {
    fn update_particle(&self, p: &Particle) -> Particle {
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

        let mut tmp = p.clone();
        if dist < (aoe * aoe) {
            tmp.apply_force(vec * -self.strength);
        } else if dist < (aoe2 * aoe2) {
            tmp.apply_force(vec * -(self.strength/2.0));
        } else if dist < (aoe3 * aoe3) {
            tmp.apply_force(vec * -(self.strength/3.0));
        }
        tmp
    }
    fn as_drawable(&self) -> Option<&Drawable> {
        Some(self)
    }
}
