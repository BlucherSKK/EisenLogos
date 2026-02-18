use glium::glutin::surface::WindowSurface;
use glium::texture::RawImage2d;
use glium::Surface;

use crate::combaine_layers;
mod fragmenter;


pub fn main_loop(display: &Display<glium::glutin::surface::WindowSurface>) {
    // 1. Загружаем первую картинку (как Texture2d)
    let img1 = image::open("background.png").unwrap().to_rgba8();
    let dims1 = img1.dimensions();
    let raw_img1 = RawImage2d::from_raw_rgba_layout(img1.into_raw(), dims1);
    let tex1 = glium::Texture2d::new(display, raw_img1).unwrap();

    // 2. Загружаем вторую картинку (оставим как RawImage2d для теста макроса)
    let img2 = image::open("overlay.png").unwrap().to_rgba8();
    let dims2 = img2.dimensions();
    let raw_img2 = RawImage2d::from_raw_rgba_layout(img2.into_raw(), dims2);

    // В цикле отрисовки:
    let mut target = display.draw();
    target.clear_color(0.0, 0.0, 0.0, 1.0);

    // НАКЛАДЫВАЕМ ДРУГ НА ДРУГА
    combaine_layers!(
        display,
        &mut target,
        ":2d: transparent=100% :2d: transparent=50%",
        tex1,     // Слой 1: Фон (полная видимость)
        raw_img2  // Слой 2: Оверлей (50% прозрачности)
    );

    target.finish().unwrap();
}

pub fn render_generated_texture(display: &glium::Display<WindowSurface>, target: &mut glium::Frame) {
    // 1. Получаем размеры окна
    let (width, height) = display.get_framebuffer_dimensions();

    // 2. Генерируем данные пикселей (например, простая шахматка или градиент)
    // В реальном приложении лучше кешировать текстуру, а не создавать каждый кадр!
    let mut pixels: Vec<u8> = Vec::with_capacity((width * height * 4) as usize);
    for y in 0..height {
        for x in 0..width {
            pixels.push(((x as f32 / width as f32) * 255.0) as u8); // R
            pixels.push(((y as f32 / height as f32) * 255.0) as u8); // G
            pixels.push(128);                                        // B
            pixels.push(255);// A
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
