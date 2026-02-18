use glium::Display;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};
use glium::glutin::surface::WindowSurface;
use glium::Surface;
use std::time::{Duration, Instant};
use crate::opengl::{self, render_generated_texture};
#[derive(Default)]
pub struct App<'a> {
    pub window: Option<Window>,
    pub title: &'a str,
    pub render_surface: Option<glium::Display<WindowSurface>>,
    pub frame_duration: Option<Duration>,
    pub init_time: Option<Instant>,
    pub images: Option<Vec<&'a str>>
}


impl<'a> ApplicationHandler for App<'a> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let attributes = Window::default_attributes().with_title(self.title);

        self.window = Some(event_loop.create_window(attributes).unwrap());
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            },
            WindowEvent::RedrawRequested => {
                if let Some(display) = self.render_surface.as_ref()
                    &&
                    let Some(window) = self.window.as_ref()
                    {
                    // --- ГЛОБАЛЬНЫЙ ЦИКЛ РИСОВАНИЯ ---
                    let mut target = display.draw(); // Создаем фрейм

                    // Очищаем экран цветом (ваш код с sin)
                    // Очищаем фон
                    target.clear_color(0.0, 0.0, 0.0, 1.0);

                    // ВЫЗОВ НАШЕЙ ФУНКЦИИ
                    opengl::

                    target.finish().unwrap();

                    if let Some(frame_duration) = self.frame_duration {
                        let next_frame_time = Instant::now() + frame_duration;

                        event_loop.set_control_flow(ControlFlow::WaitUntil(next_frame_time));
                    } else {
                        // Если ограничений нет — рисуем максимально быстро
                        event_loop.set_control_flow(ControlFlow::Poll);
                    }

                    // Важно: запрашиваем перерисовку снова
                    window.request_redraw();
                }
            }
            _ => (),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::{ manual_test};

    #[test]
    fn test_rendering() {

        let event_loop = EventLoop::new().unwrap();

        event_loop.set_control_flow(ControlFlow::Poll);
        let (_window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
                .with_title("OpenGL FPS Limit")
                .build(&event_loop);

        let mut app = App::default();
        app.frame_duration = Some(Duration::from_millis(1000 / 60 as u64));
        app.render_surface = Some(display);
        app.init_time = Some(Instant::now());

        event_loop.run_app(&mut app).unwrap();

        manual_test("Появилось ли окно приложения?", "интерфейс создания приложения, окна");
    }
}
