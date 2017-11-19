extern crate rand;
extern crate sdl2;
extern crate rayon;

mod vec3;
mod point3;
mod particle;
mod physic_property;
mod world;
mod rectangle;
mod drawable;

use vec3::Vec3;
use point3::Point3;
use particle::ParticleBuilder;
use physic_property::{Gravity, GravityWell, AirResistance, Wind, BigGravityWell};
use rectangle::Rectangle;
use world::World;
use drawable::Drawable;

use std::boxed::Box;
use std::env;

use rand::{thread_rng, Rng};
use sdl2::image::{INIT_PNG, INIT_JPG};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::pixels::Color;
use sdl2::render::SurfaceCanvas;
use sdl2::surface::Surface;
use sdl2::pixels::PixelFormatEnum;
use sdl2::gfx::framerate::FPSManager;
use sdl2::rect::Rect;


// TODO add more flexibility (more complex gravity well, particle mass, ...)
// TODO add collision ???
// FIXME better drawing of physic properties
// TODO other physic property (accelerator tube, ...)

fn main() {
    let args : Vec<String> = env::args().collect();
    let width: u32  = args[1].parse().unwrap();
    let height: u32 = args[2].parse().unwrap();
    let parts_by_frame: u32 = args[3].parse().unwrap();
    let bound = Rectangle{
        up_left_corner: Point3::new(0.0,0.0,0.0),
        height: height as f64,
        width: width as f64,
        depth: 0.0
    };
    let mut world = World::new(vec![Box::new(Gravity{}),
                                    Box::new(Wind{}),
                                    Box::new(AirResistance::new()),
                                    Box::new(GravityWell::new(Point3{
                                        x: bound.center().x - 200.0,
                                        y: bound.center().y - 300.0,
                                        z: bound.center().z}, 7.0, 10.0)),
                                    Box::new(GravityWell::new(Point3{
                                        x: bound.center().x + 100.0,
                                        y: bound.center().y,
                                        z: bound.center().z}, 7.0, 10.0)),
                                    Box::new(BigGravityWell::new(Point3{
                                        x: bound.center().x - 500.0,
                                        y: bound.center().y,
                                        z: bound.center().z}, 1.0, 1.0, 13)),
                                    // Box::new(GravityWell::new(Point3{
                                    //     x: bound.center().x + 130.0,
                                    //     y: bound.center().y,
                                    //     z: bound.center().z}, 7.0, 10.0)),
                                    Box::new(GravityWell::new(Point3{
                                        x: bound.center().x + 160.0,
                                        y: bound.center().y,
                                        z: bound.center().z}, 7.0, 10.0))],
                               bound,
                               bound.center(),
                               Box::new(move |p: Point3| {
                                   // meh
                                   let mut rng = thread_rng();
                                   ParticleBuilder::new(p,
                                                        Vec3::new(
                                                            rng.gen_range(-100.0, 100.0),
                                                            rng.gen_range(-200.0, 0.0),
                                                            0.0))
                                       .with_radius(rng.gen_range(1, 10))
                                       .with_lifetime(rng.gen_range(50, 1000))
                                       .create()
                               }));
    // SDL
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let _image_context = sdl2::image::init(INIT_PNG | INIT_JPG).unwrap();
    let window = video_subsystem.window("Particle generator", width, height)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();

    let fps_counter = FPSManager::new();
    let ttf_context = sdl2::ttf::init().unwrap();
    let mut font = ttf_context.load_font("/usr/share/wesnoth/fonts/DejaVuSans.ttf", 12).unwrap();
    font.set_style(sdl2::ttf::STYLE_BOLD);
    let texture_creator = canvas.texture_creator();

    let mut cpt = 0;

    'mainloop: loop {
        let mut surface_canvas = SurfaceCanvas::from_surface(
            Surface::new(1900, 1060, PixelFormatEnum::RGBA4444).unwrap()
        ).unwrap();

        for event in sdl_context.event_pump().unwrap().poll_iter() {
            match event {
                Event::Quit{..} |
                Event::KeyDown {keycode: Option::Some(Keycode::Escape), ..} =>
                    break 'mainloop,
                Event::KeyDown {keycode: Option::Some(Keycode::Space), ..} => {
                    for _ in 0..1 {
                        world.create_particle();
                    }
                }
                _ => {}
            }
        }
        if cpt == 0 {
            world.update();
        } else {
            canvas.set_draw_color(Color::RGB(0, 0, 0));
            canvas.clear();
            surface_canvas.set_draw_color(Color::RGB(0, 0, 0));
            surface_canvas.clear();
            // Point where the particle are created
            surface_canvas.filled_circle((bound.center().x) as i16,
                                         bound.center().y as i16,
                                         1,
                                         (255, 255, 255, 255)
            ).unwrap();
            for p in &world.properties {
                match p.as_drawable() {
                    None => {},
                    Some(d) => d.draw_surface(&mut surface_canvas)
                }
            }
            for p in &world.particles {
                if p.is_alive() {
                    p.draw_surface(&mut surface_canvas);
                }
            }
            world.boundaries.draw_surface(&mut surface_canvas);
            let surface = font.render(&fps_counter.get_framerate().to_string())
                .blended(Color::RGBA(255, 0, 0, 255)).unwrap();
            let surface2 = font.render(&world.particles
                                       .iter()
                                       .filter(|p| p.is_alive())
                                       .count()
                                       .to_string())
                .blended(Color::RGBA(255, 0, 0, 255)).unwrap();
            let surface3 = font.render(&world.particles.len().to_string())
                .blended(Color::RGBA(255, 0, 0, 255)).unwrap();
            let texture = texture_creator.create_texture_from_surface(&surface).unwrap();
            let texture2 = texture_creator.create_texture_from_surface(&surface2).unwrap();
            let texture3 = texture_creator.create_texture_from_surface(&surface3).unwrap();
            let texture_creator = canvas.texture_creator();
            canvas.copy(&texture_creator.create_texture_from_surface(
                surface_canvas.into_surface()
            ).unwrap(),
                        None,
                        None).unwrap();
            canvas.copy(&texture, None, Some(Rect::new(0, 0, 50, 50))).unwrap();
            canvas.copy(&texture2, None, Some(Rect::new(0, 55, 50, 50))).unwrap();
            canvas.copy(&texture3, None, Some(Rect::new(0, 110, 50, 50))).unwrap();
            canvas.present();
            for _ in 0..parts_by_frame {
                world.create_particle();
            }
        }
        cpt = 1 - cpt;
    }
}
