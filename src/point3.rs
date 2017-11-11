
use vec3::Vec3;

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
