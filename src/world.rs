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
    create_particle_fun: Box<Fn(Point3) -> Particle>,

    cpt: i16

}

impl World {
    pub fn new(pr: Vec<Box<PhysicProperty>>, b: Rectangle, c: Point3, f: Box<Fn(Point3) -> Particle>) -> World {
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
        self.particles.par_iter_mut().for_each(|p| {
            for prop in &self.properties {
                p = &mut prop.update_particle(p)
            }
            p.update();
            if ! self.boundaries.is_in_bound(&p.position) {
                p.alive = false
            };
        });
        // for p in self.particles.iter_mut() {
        //     for prop in &self.properties {
        //         prop.update_particle(p)
        //     }
        //     p.update();
        //     if ! self.boundaries.is_in_bound(&p.position) {
        //         p.alive = false
        //     };
            // if ! p.alive {
            //     cpt += 1;
            // }
        // }
        // if cpt >= (self.particles.len() / 2) {
        self.cpt += 1;
        if self.cpt > 10 {
            self.particles.retain(|&x| x.alive);
            self.cpt = 0;
        }
    }

    pub fn create_particle(&mut self) {
        self.particles.push((self.create_particle_fun)(self.particle_creation_point).clone());
    }
}
