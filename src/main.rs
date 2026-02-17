use glium::{Surface};
use std::time::{Instant, Duration};
use std::thread;
mod application;
mod opengl;
mod test;
use glium::Display;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};
use glium::glutin::surface::WindowSurface;
use crate::opengl::{OpenGlinterface};


fn main() {
    let event_loop = EventLoop::new().unwrap();

    event_loop.set_control_flow(ControlFlow::Poll);
    let (_window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
    .with_title("OpenGL FPS Limit")
    .build(&event_loop);

    let mut app = application::App::default();
    app.frame_duration = Some(Duration::from_millis(1000 / 60 as u64));
    app.render_surface = Some(display);
    app.init_time = Some(Instant::now());

    event_loop.run_app(&mut app).unwrap();
}
