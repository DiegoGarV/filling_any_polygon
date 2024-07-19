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
    let fill_color_p4 = Color::from_hex("green", 0x00FF00);

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

    // Cuarto polígono
    let polygon4 = vec![
        vec3(413.0, 177.0, 0.0),
        vec3(448.0, 159.0, 0.0),
        vec3(502.0, 88.0, 0.0),
        vec3(553.0, 53.0, 0.0),
        vec3(535.0, 36.0, 0.0),
        vec3(676.0, 37.0, 0.0),
        vec3(660.0, 52.0, 0.0),
        vec3(750.0, 145.0, 0.0),
        vec3(761.0, 179.0, 0.0),
        vec3(672.0, 192.0, 0.0),
        vec3(659.0, 214.0, 0.0),
        vec3(615.0, 214.0, 0.0),
        vec3(632.0, 230.0, 0.0),
        vec3(580.0, 230.0, 0.0),
        vec3(597.0, 215.0, 0.0),
        vec3(552.0, 214.0, 0.0),
        vec3(517.0, 144.0, 0.0),
        vec3(466.0, 180.0, 0.0),
    ];
    let polygon5 = vec![
        vec3(682.0, 175.0, 0.0),
        vec3(708.0, 120.0, 0.0),
        vec3(735.0, 148.0, 0.0),
        vec3(739.0, 170.0, 0.0),
    ];
    framebuffer.draw_polygon(&polygon4, &outline_color, &fill_color_p4);
    framebuffer.draw_polygon(&polygon5, &bg_color, &bg_color);

    framebuffer.render_buffer("out.bmp").expect("Error al renderizar el buffer a BMP");

    println!("Framebuffer rendered to out.bmp"); 
}