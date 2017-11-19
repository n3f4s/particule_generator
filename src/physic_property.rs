use particle::Particle;
use vec3::Vec3;
use vec3::unit_vector;
use point3::Point3;
use drawable::Drawable;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::surface::Surface;
use sdl2::gfx::primitives::DrawRenderer;
//use sdl2::render::RenderTarget;
use std::f64::consts::PI;

pub trait PhysicProperty : Send + Sync {
    //type DrawableEntity: Drawable;
    fn update_particle(&self, p: &Particle) -> Particle;
    //fn as_drawable(&self) -> &Self::DrawableEntity;
    fn as_drawable(&self) -> Option<&Drawable>;
}
// struct Void {}
// impl Drawable for Void {
//     fn draw<T: RenderTarget>(&self, canvas: Canvas<T>) {}
// }

pub struct Gravity {}
impl PhysicProperty for Gravity {
    //type DrawableEntity = Void;
    fn update_particle(&self, p: &Particle) -> Particle {
        let mut tmp = p.clone();
        tmp.apply_force(Vec3::new(0.0, 1.0 * (p.get_mass() as f64), 0.0));
        tmp
    }
    fn as_drawable(&self) -> Option<&Drawable> {
        None
    }
}

pub struct Wind {}
impl PhysicProperty for Wind {
    // type DrawableEntity = Void;
    fn update_particle(&self, p: &Particle) -> Particle {
        let mut tmp = p.clone();
        tmp.apply_force(Vec3::new(-0.25, 0.0, 0.0));
        tmp
    }
    // fn as_drawable(&self) -> &Self::DrawableEntity {
    //     Void {}
    // }
    fn as_drawable(&self) -> Option<&Drawable> {
        None
    }
}

pub struct AirResistance {}
impl PhysicProperty for AirResistance {
    //type DrawableEntity = Void;
    fn update_particle(&self, p: &Particle) -> Particle {
        let density = 1.0; // air density
        let drag = 0.020; // drag coeficient (magic number here)
        let area = PI * (p.get_radius() as f64);// area affected by the air resistance, compute using radius of sphere
        let mut next_point = p.copy();
        next_point.update();
        let speed = (((next_point.get_position().x - p.get_position().x) *
                      (next_point.get_position().x - p.get_position().x)) +
                     ((next_point.get_position().y - p.get_position().y) *
                      (next_point.get_position().y - p.get_position().y))).sqrt();
        let f = ((density * drag * area) / 2.0) * speed;
        let unit_v = unit_vector(p.get_direction());
        let mut tmp = p.copy(); //p.clone();
        tmp.apply_force(-1.0 * f * unit_v);
        tmp
    }
    // fn as_drawable(&self) -> &Self::DrawableEntity {
    //     Void {}
    // }
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
    //fn draw<T: RenderTarget>(&self, c: &mut Canvas<T>) {
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
}
impl PhysicProperty for GravityWell {
    //type DrawableEntity = GravityWell;
    fn update_particle(&self, p: &Particle) -> Particle {
        let dist = ((self.position.x - p.get_position().x) *
                    (self.position.x - p.get_position().x)) +
                   ((self.position.y - p.get_position().y) *
                   (self.position.y - p.get_position().y));
        let aoe = self.area_of_effect;
        let aoe2 = aoe * 2.0;
        let aoe3 = aoe * 3.0;

        let vec = Vec3{ x: p.get_position().x - self.position.x,
                        y: p.get_position().y - self.position.y,
                        z: p.get_position().z - self.position.z};

        let mut tmp = p.clone();
        if dist < (aoe * aoe) {
            tmp.apply_force(vec * -self.strength * p.get_mass());
        } else if dist < (aoe2 * aoe2) {
            tmp.apply_force(vec * -(self.strength/2.0));
        } else if dist < (aoe3 * aoe3) {
            tmp.apply_force(vec * -(self.strength/3.0));
        }
        tmp
    }
    // fn as_drawable(&self) -> &Self::DrawableEntity {
    //     self
    // }
    fn as_drawable(&self) -> Option<&Drawable> {
        Some(self)
    }
}

#[derive(Debug, Default, PartialEq, Copy, Clone)]
pub struct BigGravityWell {
    pub position: Point3,
    pub strength: f64,
    pub area_of_effect: f64,
    pub layers: u64,
}
impl BigGravityWell {
    pub fn new(p: Point3, s: f64, aoe: f64, l: u64) -> BigGravityWell {
        BigGravityWell {
            position: p,
            strength: s,
            area_of_effect: aoe,
            layers: l
        }
    }
}
impl Drawable for BigGravityWell {
    //fn draw<T: RenderTarget>(&self, c: &mut Canvas<T>) {
    fn draw_window(&self, c: &mut Canvas<Window>) {
        let mut alpha = 55;
        let mut aoe = self.area_of_effect;
        for i in 0..self.layers {
            let green = std::cmp::min(255, i*12) as u8;
            c.filled_circle(self.position.x as i16,
                            self.position.y as i16,
                            (aoe) as i16,
                            (0, green, 255, alpha)
            ).unwrap();
            alpha += 20;
            aoe = (aoe * 1.5) + self.area_of_effect;
        }
    }
    fn draw_surface(&self, c: &mut Canvas<Surface>) {
        let mut alpha = 255;
        let mut aoe = self.area_of_effect;
        for _ in 0..self.layers {
            // let green = std::cmp::min(255, i*12) as u8;
            c.filled_circle(self.position.x as i16,
                            self.position.y as i16,
                            (aoe) as i16,
                            (0, 0, 255, alpha as u8)
            ).unwrap();
            alpha -= 255/(self.layers+1);
            aoe = (aoe * 1.5) + self.area_of_effect;
        }
    }
}
impl PhysicProperty for BigGravityWell {
    //type DrawableEntity = BigGravityWell;
    fn update_particle(&self, p: &Particle) -> Particle {
        let dist = ((self.position.x - p.get_position().x) *
                    (self.position.x - p.get_position().x)) +
                   ((self.position.y - p.get_position().y) *
                   (self.position.y - p.get_position().y));
        let mut aoe = self.area_of_effect;

        let vec = Vec3{ x: p.get_position().x - self.position.x,
                        y: p.get_position().y - self.position.y,
                        z: p.get_position().z - self.position.z};

        let mut tmp = p.clone();
        for i in 1..(self.layers+1) {
            let j = i as f64;
            if dist < (aoe * aoe) {
                tmp.apply_force(vec * (-self.strength / j) * p.get_mass());
                break
            }
            aoe = (aoe * 1.5) + self.area_of_effect;
        }
        tmp
    }
    // fn as_drawable(&self) -> &Self::DrawableEntity {
    //     self
    // }
    fn as_drawable(&self) -> Option<&Drawable> {
        Some(self)
    }
}
