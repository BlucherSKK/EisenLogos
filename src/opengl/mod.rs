use glium::glutin::surface::WindowSurface;

pub struct OpenGlinterface {
    display: Option<glium::Display<WindowSurface>>,
}
