
use point3::Point3;

#[derive(Debug, Default, PartialEq, Copy, Clone)]
pub struct Rectangle {
    pub up_left_corner: Point3,
    pub height: f64,
    pub width: f64,
    pub depth: f64
}

impl Rectangle {
    pub fn is_in_bound(&self, p: &Point3) -> bool {
        p.x >= self.up_left_corner.x &&
            p.y >= self.up_left_corner.y &&
            p.z >= self.up_left_corner.z &&
            p.x <= self.up_left_corner.x + self.width &&
            p.y <= self.up_left_corner.y + self.height &&
            p.z <= self.up_left_corner.z + self.depth
    }

    pub fn center(&self) -> Point3 {
        Point3 {
            x: self.up_left_corner.x + self.width/2.0,
            y: self.up_left_corner.y + self.height/2.0,
            z: self.up_left_corner.z + self.depth/2.0
        }
    }
}
