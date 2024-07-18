mod color;
mod framebuffer;
mod bmp;
mod line;

use crate::color::Color;
use crate::framebuffer::Framebuffer;
use line::Line;
use nalgebra_glm::vec3;

fn main() {
    let mut framebuffer = Framebuffer::new(800, 600);
    let bg_color = Color::from_hex("black", 0x000000);
    let outline_color_p1 = Color::from_hex("white", 0xFFFFFF);
    let fill_color_p1 = Color::from_hex("yellow", 0xFFFF00);

    framebuffer.set_background_color(bg_color);
    framebuffer.clear();
    
    // Primer polígono 
    let polygon1 = vec![
        vec3(165.0, 380.0, 0.0),
        vec3(185.0, 360.0, 0.0), 
        vec3(180.0, 330.0, 0.0),
        vec3(207.0, 345.0, 0.0),
        vec3(233.0, 330.0, 0.0),
        vec3(230.0, 360.0, 0.0),
        vec3(250.0, 380.0, 0.0),
        vec3(220.0, 385.0, 0.0),
        vec3(205.0, 410.0, 0.0),
        vec3(193.0, 383.0, 0.0),
    ];

    

    framebuffer.draw_polygon(&polygon1, &outline_color_p1, &fill_color_p1);

    framebuffer.render_buffer("out.bmp").expect("Error al renderizar el buffer a BMP");

    println!("Framebuffer rendered to out.bmp"); 
}