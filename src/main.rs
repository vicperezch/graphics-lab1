mod framebuffer;
mod line;

use framebuffer::Framebuffer;
use raylib::prelude::*;

use crate::line::{draw_polygon, fill_polygon};

fn main() {
    let width = 800;
    let height = 600;
    let mut framebuffer = Framebuffer::new(width, height, Color::BLACK);

    framebuffer.clear();

    framebuffer.set_current_color(Color::GREEN);
    let p1: Vec<Vector2> = vec![
        Vector2 { x: 165.0, y: 380.0 },
        Vector2 { x: 185.0, y: 360.0 },
        Vector2 { x: 180.0, y: 330.0 },
        Vector2 { x: 207.0, y: 345.0 },
        Vector2 { x: 233.0, y: 330.0 },
        Vector2 { x: 230.0, y: 360.0 },
        Vector2 { x: 250.0, y: 380.0 },
        Vector2 { x: 220.0, y: 385.0 },
        Vector2 { x: 205.0, y: 410.0 },
        Vector2 { x: 193.0, y: 383.0 },
    ];

    let p2: Vec<Vector2> = vec![
        Vector2 { x: 321.0, y: 335.0 },
        Vector2 { x: 288.0, y: 286.0 },
        Vector2 { x: 339.0, y: 251.0 },
        Vector2 { x: 374.0, y: 302.0 },
    ];

    let p3: Vec<Vector2> = vec![
        Vector2 { x: 377.0, y: 249.0 },
        Vector2 { x: 411.0, y: 197.0 },
        Vector2 { x: 436.0, y: 249.0 },
    ];

    let p4: Vec<Vector2> = vec![
        Vector2 { x: 413.0, y: 177.0 },
        Vector2 { x: 448.0, y: 159.0 },
        Vector2 { x: 502.0, y: 88.0 },
        Vector2 { x: 553.0, y: 53.0 },
        Vector2 { x: 535.0, y: 36.0 },
        Vector2 { x: 676.0, y: 37.0 },
        Vector2 { x: 660.0, y: 52.0 },
        Vector2 { x: 750.0, y: 145.0 },
        Vector2 { x: 761.0, y: 179.0 },
        Vector2 { x: 672.0, y: 192.0 },
        Vector2 { x: 659.0, y: 214.0 },
        Vector2 { x: 615.0, y: 214.0 },
        Vector2 { x: 632.0, y: 230.0 },
        Vector2 { x: 580.0, y: 230.0 },
        Vector2 { x: 597.0, y: 215.0 },
        Vector2 { x: 552.0, y: 214.0 },
        Vector2 { x: 517.0, y: 144.0 },
        Vector2 { x: 466.0, y: 180.0 },
    ];

    let p5: Vec<Vector2> = vec![
        Vector2 { x: 682.0, y: 175.0 },
        Vector2 { x: 708.0, y: 120.0 },
        Vector2 { x: 735.0, y: 148.0 },
        Vector2 { x: 739.0, y: 170.0 },
    ];

    let polygons = vec![&p1, &p2, &p3, &p4];

    for pol in polygons {
        draw_polygon(&mut framebuffer, pol);
        fill_polygon(&mut framebuffer, pol);
    }

    draw_polygon(&mut framebuffer, &p5);

    framebuffer.set_current_color(Color::BLACK);
    fill_polygon(&mut framebuffer, &p5);

    let output = "polygons.bmp";

    framebuffer.render_to_file(output);
}
