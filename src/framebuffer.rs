use crate::color::Color;
use std::io;
use crate::bmp::write_bmp_file;

pub struct Framebuffer {
    width: usize,
    height: usize,
    buffer: Vec<Color>,
    background_color: Color,
    current_color: Color,
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Framebuffer {
        // Crear colores predeterminados para el fondo y el color actual
        let background_color = Color::from_hex("Black", 0x000000);
        let current_color = Color::from_hex("White", 0xFFFFFF);

        // Inicializar el buffer con el color de fondo
        let buffer = vec![background_color.clone(); width * height];

        Framebuffer {
            width,
            height,
            buffer,
            background_color,
            current_color,
        }
    }

    pub fn clear(&mut self) {
        for pixel in self.buffer.iter_mut() {
            *pixel = self.background_color.clone();
        }
    }

    pub fn point(&mut self, x: isize, y: isize) {
        if x >= 0 && y >= 0 && (x as usize) < self.width && (y as usize) < self.height {
            let index = (y as usize) * self.width + (x as usize);
            self.buffer[index] = self.current_color.clone();
        }
    }

    // pub fn color_on_coordinate(&self, x: usize, y: usize) -> Option<&Color> {
    //     if x < self.width && y < self.height {
    //         let index = y * self.width + x;
    //         Some(&self.buffer[index])
    //     } else {
    //         None
    //     }
    // }

    pub fn set_background_color(&mut self, color: Color) {
        self.background_color = color;
    }

    pub fn set_current_color(&mut self, color: Color) {
        self.current_color = color;
    }

    pub fn render_buffer(&self, file_path: &str) -> io::Result<()> {
        write_bmp_file(file_path, &self.buffer, self.width, self.height)?;
        Ok(())
    }
}
