
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::surface::Surface;
// use sdl2::render::RenderTarget;

pub trait Drawable {
    //fn draw<T: RenderTarget>(&self, &mut canvas: Canvas<T>);
    fn draw_window(&self, canvas: &mut Canvas<Window>);
    fn draw_surface(&self, canvas: &mut Canvas<Surface>);
}
