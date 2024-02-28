extern crate sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

use std::collections::HashMap;
use std::time::Duration;
use std::thread;
use std::fs::File;
use std::io::Read;

mod chip8;
use chip8::Chip8;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video();

    let window = video_subsystem.expect("Failed to get video subsystem").window("Chip8 Emulator", 640, 320)
        .build()
        .expect("Failed to create window");

    let mut canvas = window.into_canvas().build().expect("Failed to create a canvas");
    let mut event_pump = sdl_context.event_pump().expect("Failed to get event pump");

    let file_path = "games/games/Soccer.ch8";
    let mut file = File::open(file_path).unwrap();
    let mut rom_data = Vec::new();
    file.read_to_end(&mut rom_data).unwrap();
    
    let mut chip8 = Chip8::new();
    
    chip8.load_rom(&rom_data);

    let key_map: HashMap<Keycode, usize> = [
        (Keycode::Num1, 0x1), 
        (Keycode::Num2, 0x2), 
        (Keycode::Num3, 0x3), 
        (Keycode::Num4, 0xC),
        (Keycode::Q, 0x4), 
        (Keycode::W, 0x5), 
        (Keycode::E, 0x6), 
        (Keycode::R, 0xD),
        (Keycode::A, 0x7), 
        (Keycode::S, 0x8), 
        (Keycode::D, 0x9), 
        (Keycode::F, 0xE),
        (Keycode::Z, 0xA), 
        (Keycode::X, 0x0), 
        (Keycode::C, 0xB), 
        (Keycode::V, 0xF),
    ].iter().cloned().collect();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'running,
                Event::KeyDown { keycode: Some(keycode), .. } => {
                    if let Some(&key) = key_map.get(&keycode) {
                        chip8.set_key(key, true);
                    }
                },
                Event::KeyUp { keycode: Some(keycode), .. } => {
                    if let Some(&key) = key_map.get(&keycode) {
                        chip8.set_key(key, false);
                    }
                },
                _ => {}
            }
        }

        chip8.emulate_cycle();

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.set_draw_color(Color::RGB(255, 255, 255));

        let graphics = chip8.get_graphics();

        for (i, &pixel) in graphics.iter().enumerate() {
            if pixel == 1 {
                let x = (i % 64) as i32;
                let y = (i / 64) as i32;
                canvas.fill_rect(Rect::new(x * 10, y * 10, 10, 10)).unwrap();
            }
        }

        canvas.present();

        chip8.tick();

        thread::sleep(Duration::from_millis(16));
    }
}
