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
    let outline_color = Color::from_hex("white", 0xFFFFFF);
    let fill_color_p1 = Color::from_hex("yellow", 0xFFFF00);
    let fill_color_p2 = Color::from_hex("blue", 0x0000FF);
    let fill_color_p3 = Color::from_hex("red", 0xFF0000);

    framebuffer.set_background_color(bg_color.clone());
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
    framebuffer.draw_polygon(&polygon1, &outline_color, &fill_color_p1);

    // Segundo polígono
    let polygon2 = vec![
        vec3(321.0, 335.0, 0.0),
        vec3(288.0, 286.0, 0.0),
        vec3(339.0, 251.0, 0.0),
        vec3(374.0, 302.0, 0.0),
    ];
    framebuffer.draw_polygon(&polygon2, &outline_color, &fill_color_p2);

    // Tercer polígono
    let polygon3 = vec![
        vec3(377.0, 249.0, 0.0),
        vec3(411.0, 197.0, 0.0),
        vec3(436.0, 249.0, 0.0),
    ];
    framebuffer.draw_polygon(&polygon3, &outline_color, &fill_color_p3);

    // Crea el bitmap
    framebuffer.render_buffer("out.bmp").expect("Error al renderizar el buffer a BMP");

    println!("Framebuffer rendered to out.bmp"); 
}