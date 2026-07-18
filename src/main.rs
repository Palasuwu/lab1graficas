mod framebuffer;

use framebuffer::{Color, Framebuffer};

const BACKGROUND: Color = Color::new(20, 20, 30);

type Point = (i32, i32);

fn main() {
    let mut fb = Framebuffer::new(800, 450, BACKGROUND);

    let poligono1: Vec<Point> = vec![
        (165, 380), (185, 360), (180, 330), (207, 345), (233, 330),
        (230, 360), (250, 380), (220, 385), (205, 410), (193, 383),
    ];
    let poligono2: Vec<Point> = vec![(321, 335), (288, 286), (339, 251), (374, 302)];
    let poligono3: Vec<Point> = vec![(377, 249), (411, 197), (436, 249)];
    let poligono4: Vec<Point> = vec![
        (413, 177), (448, 159), (502, 88), (553, 53), (535, 36), (676, 37), (660, 52),
        (750, 145), (761, 179), (672, 192), (659, 214), (615, 214), (632, 230), (580, 230),
        (597, 215), (552, 214), (517, 144), (466, 180),
    ];
    // Agujero dentro del polígono 4: no debe pintarse.
    let poligono5: Vec<Point> = vec![(682, 175), (708, 120), (735, 148), (739, 170)];

    let amarillo = Color::new(255, 204, 0);
    let azul = Color::new(64, 128, 255);
    let rojo = Color::new(230, 57, 70);
    let verde = Color::new(80, 200, 120);
    let blanco = Color::new(255, 255, 255);

    fill_polygon(&mut fb, &[&poligono1], amarillo);
    draw_outline(&mut fb, &poligono1, blanco);

    fill_polygon(&mut fb, &[&poligono2], azul);
    draw_outline(&mut fb, &poligono2, blanco);

    fill_polygon(&mut fb, &[&poligono3], rojo);
    draw_outline(&mut fb, &poligono3, blanco);

    // El polígono 5 se pasa como segundo contorno: con la regla par-impar
    // queda como agujero sin relleno.
    fill_polygon(&mut fb, &[&poligono4, &poligono5], verde);
    draw_outline(&mut fb, &poligono4, blanco);
    draw_outline(&mut fb, &poligono5, blanco);

    fb.write_png("out.png").expect("no se pudo escribir out.png");
    println!("Imagen generada: out.png");
}

// Relleno por scan-line con regla par-impar. Acepta varios contornos:
// las intersecciones de todos se mezclan, así los contornos interiores
// se convierten en agujeros.
fn fill_polygon(fb: &mut Framebuffer, contours: &[&[Point]], color: Color) {
    let ys = contours.iter().flat_map(|c| c.iter().map(|p| p.1));
    let y_min = ys.clone().min().unwrap();
    let y_max = ys.max().unwrap();

    for y in y_min..=y_max {
        // Se evalúa la línea en el centro del pixel para no contar
        // dos veces las intersecciones que caen justo en un vértice.
        let scan_y = y as f64 + 0.5;
        let mut intersections: Vec<f64> = Vec::new();

        for contour in contours {
            let n = contour.len();
            for i in 0..n {
                let (x0, y0) = contour[i];
                let (x1, y1) = contour[(i + 1) % n];
                let (x0, y0, x1, y1) = (x0 as f64, y0 as f64, x1 as f64, y1 as f64);
                if (y0 <= scan_y && y1 > scan_y) || (y1 <= scan_y && y0 > scan_y) {
                    let t = (scan_y - y0) / (y1 - y0);
                    intersections.push(x0 + t * (x1 - x0));
                }
            }
        }

        intersections.sort_by(|a, b| a.partial_cmp(b).unwrap());

        for pair in intersections.chunks(2) {
            if let [xa, xb] = pair {
                let start = (xa - 0.5).ceil() as i32;
                let end = (xb - 0.5).floor() as i32;
                for x in start..=end {
                    fb.point(x, y, color);
                }
            }
        }
    }
}

fn draw_outline(fb: &mut Framebuffer, points: &[Point], color: Color) {
    let n = points.len();
    for i in 0..n {
        let (x0, y0) = points[i];
        let (x1, y1) = points[(i + 1) % n];
        line(fb, x0, y0, x1, y1, color);
    }
}

// Algoritmo de Bresenham.
fn line(fb: &mut Framebuffer, mut x0: i32, mut y0: i32, x1: i32, y1: i32, color: Color) {
    let dx = (x1 - x0).abs();
    let dy = -(y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;

    loop {
        fb.point(x0, y0, color);
        if x0 == x1 && y0 == y1 {
            break;
        }
        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            x0 += sx;
        }
        if e2 <= dx {
            err += dx;
            y0 += sy;
        }
    }
}
