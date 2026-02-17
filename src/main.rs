use glium::{Surface};
use std::time::{Instant, Duration};
use std::thread;
mod application;


fn main() {
    let event_loop = winit::event_loop::EventLoop::builder().build().unwrap();
    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
    .with_title("OpenGL FPS Limit")
    .build(&event_loop);

    // --- FPS ---
    let target_fps = 60;
    let frame_duration = Duration::from_millis(1000 / target_fps);
    // ---------------------

    let start_time = Instant::now();

    event_loop.run(move |event, window_target| {

        let frame_start = Instant::now();

        match event {

            winit::event::Event::WindowEvent { event: winit::event::WindowEvent::CloseRequested, .. } => {
                window_target.exit();
            },
            winit::event::Event::
            winit::event::Event::AboutToWait => {

            },
            _ => (),
        }

        let elapsed = start_time.elapsed().as_secs_f32();
        let mut target = display.draw();
        target.clear_color(0.1, elapsed.sin().abs(), 0.4, 1.0);
        target.finish().unwrap();

        let elapsed_time = frame_start.elapsed();
        if elapsed_time < frame_duration {
            thread::sleep(frame_duration - elapsed_time);
        }


    }).unwrap();
}
