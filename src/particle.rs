use vec3::Vec3;
use point3::Point3;

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
    pub fn new(p: Point3, d: Vec3) -> Particle {
        Particle {
            position: p,
            direction: d,
            alive: true,
            lifetime: 500
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
}
