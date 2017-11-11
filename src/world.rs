use particle::Particle;
use physic_property::PhysicProperty;
use point3::Point3;
use rectangle::Rectangle;

use rayon::prelude::*;
use rayon::iter::IntoParallelRefMutIterator;

pub struct World {
    pub particles: Vec<Particle>,
    pub properties: Vec<Box<PhysicProperty>>,
    pub boundaries: Rectangle,

    particle_creation_point: Point3,
    create_particle_fun: Box<Fn(Point3) -> Particle + Sync + Send>,

    cpt: i16

}

impl World {
    pub fn new(pr: Vec<Box<PhysicProperty>>, b: Rectangle, c: Point3, f: Box<Fn(Point3) -> Particle + Sync + Send>) -> World {
        World {
            particles: vec![],
            properties: pr,
            boundaries: b,
            particle_creation_point: c,
            create_particle_fun: f,
            cpt: 0
        }
    }
    pub fn update(&mut self) {
        // let mut cpt = 0;
        // par_iter_mut
        let prop = &self.properties;
        let bound = &self.boundaries;
        self.particles.par_iter_mut().for_each(|p: &mut Particle| {
            //for prop in &self.properties {
            for prop in prop.iter() {
                *p = prop.update_particle(p)
            }
            p.update();
            if ! bound.is_in_bound(&p.position) {
                p.alive = false
            };
        });
        self.cpt += 1;
        if self.cpt > 100 {
            self.particles.retain(|&x| x.alive);
            self.cpt = 0;
        }
    }

    pub fn create_particle(&mut self) {
        self.particles.push((self.create_particle_fun)(self.particle_creation_point).clone());
    }
}
