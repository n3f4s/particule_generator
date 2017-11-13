
use point3::Point3;
use drawable::Drawable;

use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::surface::Surface;
use sdl2::gfx::primitives::DrawRenderer;

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

impl Drawable for Rectangle {
    fn draw_surface(&self, c: &mut Canvas<Surface>) {
        let r = Rect::new(self.up_left_corner.x as i32,
                          self.up_left_corner.y as i32,
                          self.width as u32,
                          self.height as u32);
        c.draw_rect(r).unwrap();
    }
    fn draw_window(&self, c: &mut Canvas<Window>) {
        let r = Rect::new(self.up_left_corner.x as i32,
                          self.up_left_corner.y as i32,
                          self.width as u32,
                          self.height as u32);
        c.draw_rect(r).unwrap();
    }
}
