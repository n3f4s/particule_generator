
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::surface::Surface;

pub trait Drawable {
    //fn draw<T: RenderTarget>(&self, canvas: Canvas<T>);
    fn draw_window(&self, canvas: &mut Canvas<Window>);
    fn draw_surface(&self, canvas: &mut Canvas<Surface>);
}
