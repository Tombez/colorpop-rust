use rand::prelude::ThreadRng;
use rand::Rng;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use std::time::Duration;

#[derive(Copy, Clone)]
struct Tile {
    x: i32,
    y: i32,
    color: Color,
}

pub fn main() {
    let width = 1000;
    let height = 1000;
    let col_s = 1;
    let row_s = 1;

    let cols = width / col_s;
    let rows = height / row_s;

    let mut tiles = Vec::<Tile>::new();
    let mut filled = vec![0u8; (cols * rows) as usize];
    let mut rng = rand::thread_rng();

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Colorpop Rust", width as u32, height as u32)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let alter = |c, rng: &mut ThreadRng| {
        let off = rng.gen_range(-5i16, 6);
        std::cmp::max(std::cmp::min(c as i16 + off, 255), 0i16) as u8
    };
    let mut queue_tile =
        |x: i32, y: i32, mut color: Color, tiles: &mut Vec<Tile>, rng: &mut ThreadRng| {
            if x < 0 || x >= cols || y < 0 || y >= rows {
                return;
            }
            let i = x + y * cols;
            let byte = (i / 8) as usize;
            let bit = i % 8;
            if filled[byte] >> bit & 1 == 1 {
                return;
            }
            filled[byte] |= 1 << bit;

            color.r = alter(color.r, rng);
            color.g = alter(color.g, rng);
            color.b = alter(color.b, rng);
            tiles.push(Tile { x, y, color });
        };
    let mut add_tile = |index: usize,
                        canvas: &mut Canvas<sdl2::video::Window>,
                        tiles: &mut Vec<Tile>,
                        rng: &mut ThreadRng| {
        let tile = tiles[index];
        if let Some(last) = tiles.pop() {
            if index != tiles.len() {
                tiles[index] = last;
            }
        }

        canvas.set_draw_color(tile.color);
        canvas
            .fill_rect(sdl2::rect::Rect::new(
                tile.x * col_s,
                tile.y * row_s,
                col_s as u32,
                row_s as u32,
            ))
            .unwrap();

        queue_tile(tile.x as i32 + 1, tile.y as i32, tile.color, tiles, rng);
        queue_tile(tile.x as i32 - 1, tile.y as i32, tile.color, tiles, rng);
        queue_tile(tile.x as i32, tile.y + 1 as i32, tile.color, tiles, rng);
        queue_tile(tile.x as i32, tile.y - 1 as i32, tile.color, tiles, rng);
    };

    let bits = rng.gen::<u32>();
    let r = (bits & 0xff) as u8;
    let g = (bits >> 8 & 0xff) as u8;
    let b = (bits >> 16 & 0xff) as u8;
    let color = Color::RGB(r, g, b);
    tiles.push(Tile {
        x: cols / 2,
        y: rows / 2,
        color,
    });

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        let mut count = (tiles.len() as f32 * 0.3).ceil();
        loop {
            if count <= 0.0 { break; }
            count -= 1.0;
            let index = rng.gen_range(0, tiles.len());
            add_tile(index, &mut canvas, &mut tiles, &mut rng);
        }

        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
