
use std::ops::{Neg, AddAssign, SubAssign, MulAssign, DivAssign, Add, Sub, Mul, Div};
use std::marker::Copy;
use std::clone::Clone;
use std::fmt::Debug;
use std::default::Default;

use vec3d::Vec3;
use num;

trait Num : num::Num
    + Neg
    + AddAssign
    + SubAssign
    + MulAssign
    + DivAssign
    + Add
    + Sub
    + Mul
    + Div
    + Copy
    + Clone
    + Debug
    + Default
    + PartialEq
{}

#[derive(Debug, Default, PartialEq, Copy, Clone)]
pub struct Point3<T: Num> {
    pub x: T,
    pub y: T,
    pub z: T
}

// impl<T: Num> Copy for Point3<T> {}

impl<T: Num> Point3<T> {
    pub fn new(x: T, y: T, z: T) -> Point3<T> {
        Point3 {
            x: x,
            y: y,
            z: z
        }
    }
    pub fn apply_vec(&mut self, v: Vec3<T>) {
        self.x += v.x;
        self.y += v.y;
        self.z += v.z;
    }
}
