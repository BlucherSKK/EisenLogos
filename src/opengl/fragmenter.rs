use glium::{Surface, Display, texture::RawImage2d};
use std::io::Cursor;


// --- ТРЕЙТ ДЛЯ УНИФИКАЦИИ ТИПОВ ---
pub trait ToTexture {
    fn draw_to(&self, display: &Display<glium::glutin::surface::WindowSurface>, target: &mut glium::Frame, alpha: f32);
}

// Реализация для готовой текстуры
impl ToTexture for glium::Texture2d {
    fn draw_to(&self, display: &Display<glium::glutin::surface::WindowSurface>, target: &mut glium::Frame, alpha: f32) {
        render_with_shader(display, target, self, alpha);
    }
}

// Реализация для RawImage (создает временную текстуру)
impl<'a> ToTexture for RawImage2d<'a, u8> {
    fn draw_to(&self, display: &Display<glium::glutin::surface::WindowSurface>, target: &mut glium::Frame, alpha: f32) {
        let tex = glium::Texture2d::new(display, self.clone()).unwrap();
        render_with_shader(display, target, &tex, alpha);
    }
}
// --- МАКРОС ---
#[macro_export]
macro_rules! combaine_layers {
    ($display:expr, $target:expr, $fmt:expr, $($item:expr),*) => {
        {
            let configs: Vec<&str> = $fmt.split(":2d:").filter(|s| !s.is_empty()).collect();
            let mut idx = 0;
            $(
                if let Some(config) = configs.get(idx) {
                    let alpha = config.find("transparent=")
                    .and_then(|p| {
                        let s = p + "transparent=".len();
                        let e = config[s..].find('%')?;
                        config[s..s+e].trim().parse::<f32>().ok()
                    })
                    .map(|v| v / 100.0).unwrap_or(1.0);

                    // Вызываем метод трейта
                    (&$item as &dyn ToTexture).draw_to($display, $target, alpha);
                }
                idx += 1;
            )*
        }
    };
}

// --- ФУНКЦИЯ ОТРИСОВКИ ЧЕРЕЗ ШЕЙДЕР ---
// Это необходимо, так как стандартный blit плохо работает с прозрачностью слоя
fn render_with_shader(display: &Display<glium::glutin::surface::WindowSurface>, target: &mut glium::Frame, tex: &glium::Texture2d, alpha: f32) {
    let program = glium::Program::from_source(display,
                                              " #version 140 \n in vec2 position; in vec2 tex_coords; out vec2 v_tex_coords; void main() { v_tex_coords = tex_coords; gl_Position = vec4(position, 0.0, 1.0); } ",
                                              " #version 140 \n uniform sampler2D tex; uniform float alpha; in vec2 v_tex_coords; out vec4 color; void main() { vec4 tex_color = texture(tex, v_tex_coords); color = vec4(tex_color.rgb, tex_color.a * alpha); } ",
                                              None).unwrap();

                                              #[derive(Copy, Clone)]
                                              struct Vertex { position: [f32; 2], tex_coords: [f32; 2] }
                                              glium::implement_vertex!(Vertex, position, tex_coords);

                                              let shape = vec![
                                                  Vertex { position: [-1.0, -1.0], tex_coords: [0.0, 0.0] },
                                                  Vertex { position: [ 1.0, -1.0], tex_coords: [1.0, 0.0] },
                                                  Vertex { position: [ 1.0,  1.0], tex_coords: [1.0, 1.0] },
                                                  Vertex { position: [-1.0,  1.0], tex_coords: [0.0, 1.0] },
                                              ];

                                              let vb = glium::VertexBuffer::new(display, &shape).unwrap();
                                              let ib = glium::index::NoIndices(glium::index::PrimitiveType::TriangleFan);

                                              let uniforms = glium::uniform! { tex: tex, alpha: alpha };

                                              // Включаем альфа-смешивание (Alpha Blending)
                                              let params = glium::DrawParameters {
                                                  blend: glium::DrawParameters::default().blend,
                                                  .. Default::default()
                                              };
                                              // Для реального наложения слоев:
                                              let blending = glium::DrawParameters {
                                                  blend: glium::Blend::alpha_blending(),
                                                  .. Default::default()
                                              };

                                              target.draw(&vb, &ib, &program, &uniforms, &blending).unwrap();
}

// --- ПРИМЕР ИСПОЛЬЗОВАНИЯ ---

