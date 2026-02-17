use glium::glutin::surface::WindowSurface;
use glium::texture::RawImage2d;
use glium::Surface;


pub fn render_generated_texture(display: &glium::Display<WindowSurface>, target: &mut glium::Frame, count: &mut u8) {
    // 1. Получаем размеры окна
    let (width, height) = display.get_framebuffer_dimensions();

    // 2. Генерируем данные пикселей (например, простая шахматка или градиент)
    // В реальном приложении лучше кешировать текстуру, а не создавать каждый кадр!
    let mut pixels: Vec<u8> = Vec::with_capacity((width * height * 4) as usize);
    for y in 0..height {
        for x in 0..width {
            pixels.push(((x as f32 / width as f32) * 255.0) as u8); // R
            pixels.push(((y as f32 / height as f32) * 255.0) as u8); // G
            pixels.push(128 * count % 255);                                        // B
            pixels.push(255);// A
            &count += 1;
            if count == 1000 { count = 0;}
        }
    }

    // 3. Создаем RawImage и загружаем в текстуру GPU
    let image = RawImage2d::from_raw_rgba(pixels, (width, height));
    let texture = glium::Texture2d::new(display, image).unwrap();

    // 4. Копируем текстуру во фреймбуфер (на экран)
    // blit_whole_color_to заполнит всё окно этой текстурой
    let target_rect = glium::BlitTarget {
        left: 0,
        bottom: 0,
        width: width as i32,
        height: height as i32,
    };

    texture.as_surface().blit_whole_color_to(
        target,
        &target_rect,
        glium::uniforms::MagnifySamplerFilter::Linear
    );
}
