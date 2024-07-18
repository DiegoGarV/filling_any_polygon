use crate::framebuffer::Framebuffer;
use crate::color::Color;
use nalgebra_glm::Vec3;

pub trait Line {
    fn line(&mut self, start: Vec3, end: Vec3);
    fn draw_polygon(&mut self, points: &[Vec3], outline_color: &Color, fill_color: &Color);
}

impl Line for Framebuffer {
    fn line(&mut self, start: Vec3, end: Vec3) {
        let mut x1 = start.x.round() as isize;
        let mut y1 = start.y.round() as isize;
        let x2 = end.x.round() as isize;
        let y2 = end.y.round() as isize;
        
        let dx = (x2 - x1).abs();
        let dy = (y2 - y1).abs();
        let sx = if x1 < x2 { 1 } else { -1 };
        let sy = if y1 < y2 { 1 } else { -1 };
        let mut err = if dx > dy { dx / 2 } else { -dy / 2 };

        loop {
            self.point(x1, y1);
            if x1 == x2 && y1 == y2 { break; }
            let e2 = err;
            if e2 > -dx {
                err -= dy;
                x1 += sx;
            }
            if e2 < dy {
                err += dx;
                y1 += sy;
            }
        }
    }

    fn draw_polygon(&mut self, points: &[Vec3], outline_color: &Color, fill_color: &Color) {
        if points.len() < 3 {
            return; // Necesitamos al menos tres puntos para dibujar un polígono
        }
        
        // Dibuja los bordes del polígono
        for i in 0..points.len() {
            let start = points[i];
            let end = if i == points.len() - 1 {
                points[0] // Conectar el último punto con el primero
            } else {
                points[i + 1]
            };
            self.line(start, end);
        }

        // Configura el color para el relleno del polígono
        self.set_current_color(fill_color.clone());

        // Rellena el polígono usando el algoritmo de escaneo de líneas
        self.fill_polygon(points);

        // Configura el color para el contorno del polígono
        self.set_current_color(outline_color.clone());
        
        // Dibuja los bordes del polígono
        for i in 0..points.len() {
            let start = points[i];
            let end = if i == points.len() - 1 {
                points[0] // Conectar el último punto con el primero
            } else {
                points[i + 1]
            };
            self.line(start, end);
        }
    }


}

impl Framebuffer {
    fn fill_polygon(&mut self, points: &[Vec3]) {
        let mut ymin = f32::MAX;
        let mut ymax = f32::MIN;
        
        // Encontrar el rango de Y
        for point in points {
            ymin = ymin.min(point.y);
            ymax = ymax.max(point.y);
        }

        for y in ymin.round() as isize..=ymax.round() as isize {
            let mut intersections: Vec<isize> = vec![];
            
            // Encontrar las intersecciones con la línea horizontal y
            for i in 0..points.len() {
                let p1 = points[i];
                let p2 = if i == points.len() - 1 { points[0] } else { points[i + 1] };
                
                // Verificar si la línea entre p1 y p2 intersecta con la línea horizontal y
                if (p1.y <= y as f32 && p2.y > y as f32) || (p2.y <= y as f32 && p1.y > y as f32) {
                    let t = (y as f32 - p1.y) / (p2.y - p1.y);
                    let x = p1.x + t * (p2.x - p1.x);
                    intersections.push(x.round() as isize);
                }
            }
            
            // Ordenar las intersecciones
            intersections.sort();
            
            // Rellenar entre pares de intersecciones
            let mut i = 0;
            while i < intersections.len() {
                let x1 = intersections[i];
                if i + 1 < intersections.len() {
                    let x2 = intersections[i + 1];
                    for x in x1..x2 {
                        self.point(x, y);
                    }
                    i += 2;
                } else {
                    break;
                }
            }
        }
    }
}
