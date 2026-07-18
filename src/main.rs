use image::{Rgb, RgbImage};

const ANCHO: u32 = 800;
const ALTO: u32 = 450;

fn main() {
    let mut img = RgbImage::new(ANCHO, ALTO);

    // pinto todo el fondo primero
    for x in 0..ANCHO {
        for y in 0..ALTO {
            img.put_pixel(x, y, Rgb([20, 20, 30]));
        }
    }

    // los puntos de cada poligono
    let poligono1 = vec![
        (165, 380), (185, 360), (180, 330), (207, 345), (233, 330),
        (230, 360), (250, 380), (220, 385), (205, 410), (193, 383),
    ];
    let poligono2 = vec![(321, 335), (288, 286), (339, 251), (374, 302)];
    let poligono3 = vec![(377, 249), (411, 197), (436, 249)];
    let poligono4 = vec![
        (413, 177), (448, 159), (502, 88), (553, 53), (535, 36), (676, 37), (660, 52),
        (750, 145), (761, 179), (672, 192), (659, 214), (615, 214), (632, 230), (580, 230),
        (597, 215), (552, 214), (517, 144), (466, 180),
    ];
    let poligono5 = vec![(682, 175), (708, 120), (735, 148), (739, 170)];

    let blanco = Rgb([255, 255, 255]);

    // relleno cada poligono con su color y le dibujo la orilla blanca
    rellenar_poligono(&mut img, &poligono1, Rgb([255, 204, 0]));
    dibujar_poligono(&mut img, &poligono1, blanco);

    rellenar_poligono(&mut img, &poligono2, Rgb([255, 120, 180]));
    dibujar_poligono(&mut img, &poligono2, blanco);

    rellenar_poligono(&mut img, &poligono3, Rgb([64, 128, 255]));
    dibujar_poligono(&mut img, &poligono3, blanco);

    rellenar_poligono(&mut img, &poligono4, Rgb([64, 224, 208]));
    dibujar_poligono(&mut img, &poligono4, blanco);

    // el poligono 5 es un agujero adentro del 4, entonces lo relleno
    // con el color del fondo para que se vea vacio
    rellenar_poligono(&mut img, &poligono5, Rgb([20, 20, 30]));
    dibujar_poligono(&mut img, &poligono5, blanco);

    img.save("out.png").unwrap();
    println!("listo, se guardo out.png");
}

// pone un pixel pero con el eje y para arriba (como en la clase),
// por eso se voltea la coordenada y
fn pixel(img: &mut RgbImage, x: i32, y: i32, color: Rgb<u8>) {
    if x >= 0 && y >= 0 && x < ANCHO as i32 && y < ALTO as i32 {
        img.put_pixel(x as u32, ALTO - 1 - y as u32, color);
    }
}

// relleno con scanline: por cada linea horizontal busco donde cruza
// el poligono y pinto entre los pares de cruces
fn rellenar_poligono(img: &mut RgbImage, puntos: &Vec<(i32, i32)>, color: Rgb<u8>) {
    // primero busco el y mas chico y el mas grande del poligono
    let mut y_min = puntos[0].1;
    let mut y_max = puntos[0].1;
    for i in 0..puntos.len() {
        if puntos[i].1 < y_min {
            y_min = puntos[i].1;
        }
        if puntos[i].1 > y_max {
            y_max = puntos[i].1;
        }
    }

    for y in y_min..y_max + 1 {
        // uso la mitad del pixel porque si la linea pasa justo por un
        // vertice se contaba doble y salia mal el relleno
        let ya = y as f32 + 0.5;

        let mut cruces: Vec<f32> = Vec::new();
        for i in 0..puntos.len() {
            let (x1, y1) = puntos[i];
            // el ultimo punto se conecta con el primero
            let (x2, y2) = if i == puntos.len() - 1 {
                puntos[0]
            } else {
                puntos[i + 1]
            };

            // reviso si el lado cruza esta linea horizontal
            if (y1 as f32 <= ya && ya < y2 as f32) || (y2 as f32 <= ya && ya < y1 as f32) {
                // saco en que x cruza con la ecuacion de la recta
                let x = x1 as f32 + (ya - y1 as f32) / (y2 as f32 - y1 as f32) * (x2 as f32 - x1 as f32);
                cruces.push(x);
            }
        }

        // ordeno los cruces de menor a mayor
        cruces.sort_by(|a, b| a.partial_cmp(b).unwrap());

        // pinto de dos en dos: adentro, afuera, adentro, afuera...
        let mut i = 0;
        while i + 1 < cruces.len() {
            let inicio = cruces[i].round() as i32;
            let fin = cruces[i + 1].round() as i32;
            for x in inicio..fin {
                pixel(img, x, y, color);
            }
            i = i + 2;
        }
    }
}

// dibuja la orilla del poligono uniendo cada punto con el siguiente
fn dibujar_poligono(img: &mut RgbImage, puntos: &Vec<(i32, i32)>, color: Rgb<u8>) {
    for i in 0..puntos.len() {
        let (x1, y1) = puntos[i];
        let (x2, y2) = if i == puntos.len() - 1 {
            puntos[0]
        } else {
            puntos[i + 1]
        };
        linea(img, x1, y1, x2, y2, color);
    }
}

// algoritmo DDA para dibujar lineas
fn linea(img: &mut RgbImage, x1: i32, y1: i32, x2: i32, y2: i32, color: Rgb<u8>) {
    let dx = x2 - x1;
    let dy = y2 - y1;

    let mut pasos = dx.abs();
    if dy.abs() > pasos {
        pasos = dy.abs();
    }
    if pasos == 0 {
        pixel(img, x1, y1, color);
        return;
    }

    let mut x = x1 as f32;
    let mut y = y1 as f32;
    for _ in 0..pasos + 1 {
        pixel(img, x.round() as i32, y.round() as i32, color);
        x = x + dx as f32 / pasos as f32;
        y = y + dy as f32 / pasos as f32;
    }
}
