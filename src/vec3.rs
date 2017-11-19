
use vec3d;
pub use vec3d::unit_vector;

pub type Vec3 = vec3d::Vec3<f64>;

/*
use std::ops::{Neg, AddAssign, SubAssign, MulAssign, DivAssign, Add, Sub, Mul, Div, Index};

#[derive(Debug, Default, PartialEq, Copy, Clone)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl Vec3 {
    pub fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn squared_length(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn make_unit(&mut self) -> Self {
        *self /= self.length();
        *self
    }

    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3{
            x: x,
            y: y,
            z: z
        }
    }
}

pub fn cross(v1: &Vec3, v2: &Vec3) -> Vec3 {
    Vec3::new(
        v1.y * v2.z - v1.z * v2.y,
        -(v1.x * v2.z - v1.z * v2.x),
        v1.x * v2.y - v1.y * v2.x
    )
}

pub fn dot(v1: &Vec3, v2: &Vec3) -> f64 {
    v1.x * v2.x + v1.y * v2.y + v1.z * v2.z
}

pub fn unit_vector(v: Vec3) -> Vec3 {
    let mut tmp = v;
    tmp.make_unit();
    tmp
}

impl Index<i32> for Vec3 {
    type Output = f64;
    fn index(&self, index: i32) -> &f64 {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Tried to access out of bound vector")
        }
    }
}

impl Div for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: Vec3) -> Vec3 {
        Vec3::new(
            self.x / rhs.x,
            self.y / rhs.y,
            self.z / rhs.z,
        )
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, t: f64) -> Vec3 {
        Vec3::new(
            self.x / t,
            self.y / t,
            self.z / t,
        )
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, t: f64) -> Vec3 {
        Vec3::new(
            self.x * t,
            self.y * t,
            self.z * t,
        )
    }
}

impl Mul for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3::new(
            self.x * rhs.x,
            self.y * rhs.y,
            self.z * rhs.z,
        )
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3::new(
            rhs.x * self,
            rhs.y * self,
            rhs.z * self,
        )
    }
}

impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3::new(
            self.x - rhs.x,
            self.y - rhs.y,
            self.z - rhs.z,
        )
    }
}

impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3::new(
            self.x + rhs.x,
            self.y + rhs.y,
            self.z + rhs.z
        )
    }
}

impl DivAssign for Vec3 {
    fn div_assign(&mut self, rhs: Vec3) {
        self.x /= rhs.x;
        self.y /= rhs.y;
        self.z /= rhs.z;
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, t: f64) {
        self.x /= t;
        self.y /= t;
        self.z /= t;
    }
}

impl MulAssign for Vec3 {
    fn mul_assign(&mut self, rhs: Vec3) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, t: f64) {
        self.x *= t;
        self.y *= t;
        self.z *= t;
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Vec3) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}

#[test]
fn vec3_neg() {
    let a = Vec3::new(1.0, 2.0, 3.0);
    let a = -a;
    let c = Vec3::new(-1.0, -2.0, -3.0);
    assert!(a == c);
}

#[test]
fn vec3_add_assign() {
    let mut a = Vec3::new(1.0, 2.0, 3.0);
    a += Vec3::new(2.0, 3.0, 4.0);
    let b = Vec3::new(3.0, 5.0, 7.0);
    assert!(a == b);
}

#[test]
fn vec3_sub_assign() {
    let mut a = Vec3::new(1.0, 2.0, 3.0);
    a -= Vec3::new(2.0, 3.0, 4.0);
    let b = Vec3::new(-1.0, -1.0, -1.0);
    assert!(a == b);
}

#[test]
fn vec3_mul_assign() {
    let mut a = Vec3::new(1.0, 2.0, 3.0);
    a *= Vec3::new(2.0, 3.0, 4.0);
    let b = Vec3::new(2.0, 6.0, 12.0);
    assert!(a == b);
    a *= 2.0;
    assert!(a == Vec3::new(4.0, 12.0, 24.0));
}

#[test]
fn vec3_div_assign() {
    let mut a = Vec3::new(1.0, 2.0, 3.0);
    a /= Vec3::new(2.0, 3.0, 4.0);
    let b = Vec3::new(0.5, 2.0/3.0, 3.0/4.0);
    assert!(a == b);
    a /= 2.0;
    assert!(a == Vec3::new(0.25, 1.0/3.0, 3.0/8.0));
}

#[test]
fn vec3_add() {
    let a = Vec3::new(1.0, 2.0, 3.0);
    assert!(a + a == Vec3::new(2.0, 4.0, 6.0));
}

#[test]
fn vec3_sub() {
    let a = Vec3::new(1.0, 2.0, 3.0);
    assert!(a - a == Default::default());
}

#[test]
fn vec3_mul() {
    let a = Vec3::new(1.0, 2.0, 3.0);
    assert!(a * a == Vec3::new(1.0, 4.0, 9.0));
    assert!(a * 2.0 == Vec3::new(2.0, 4.0, 6.0));
    assert!(2.0 * a == Vec3::new(2.0, 4.0, 6.0));
}

#[test]
fn vec3_div() {
    let a = Vec3::new(1.0, 2.0, 3.0);
    assert!(a / a == Vec3::new(1.0, 1.0, 1.0));
    assert!(a / 2.0 == Vec3::new(0.5, 1.0, 1.5));
}

#[test]
fn vec3_dot() {
    let a = Vec3::new(1.0, 2.0, 3.0);
    let b = Vec3::new(2.0, 3.0, 4.0);
    assert!(dot(&a, &b) == 20.0);
}

#[test]
fn vec3_cross() {
    let a = Vec3::new(1.0, 2.0, 3.0);
    let b = Vec3::new(2.0, 3.0, 4.0);
    assert!(cross(&a, &b) == Vec3::new(-1.0, 2.0, -1.0));
}

#[test]
fn vec3_length() {
    let a = Vec3::new(1.0, 1.0, 1.0);
    assert!(a.length() == 3f64.sqrt());
}

#[test]
fn vec3_squared_length() {
    let a = Vec3::new(1.0, 1.0, 1.0);
    assert!(a.squared_length() == 3.0);
}

#[test]
fn vec3_make_unit() {
    let mut a = Vec3::new(1.0, 0.0, 0.0);
    a.make_unit();
    assert!(a.length() == 1.0);
    let mut a = Vec3::new(0.0, 1.0, 0.0);
    a.make_unit();
    assert!(a.length() == 1.0);
    let mut a = Vec3::new(0.0, 0.0, 1.0);
    a.make_unit();
    assert!(a.length() == 1.0);
}

#[test]
fn vec3_unit_vector() {
    let a = Vec3::new(1.0, 1.0, 1.0);
    let a = unit_vector(a);
    assert!(a.length() == 1.0);
    let mut a = Vec3::new(1.0, 1.0, 1.0);
    a.make_unit();
    assert!(a.length() == 1.0);
}

#[test]
fn vec3_access() {
    let a = Vec3::new(1.0, 2.0, 3.0);
    assert!(a[0] == 1.0);
    assert!(a[1] == 2.0);
    assert!(a[2] == 3.0);
    let result = ::std::panic::catch_unwind(|| a[3]);
    assert!(result.is_err());
}
*/
