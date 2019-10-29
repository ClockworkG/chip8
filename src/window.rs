use crate::context::Context;
use crate::cpu::CPU;
use crate::bus::Bus;
use crate::memory::{MainMemory, ROM, Memory};
use std::time::{Duration, Instant};

use minifb;

pub struct Window {
    cpu: CPU,
    bus: Bus,
    win: minifb::Window,
    screen_buffer: Vec<u32>,

    instruction_timestamp: Instant,
    display_timestamp: Instant,
    timer_timestamp: Instant,
    input_timestamp: Instant,
}

impl Window {
    fn draw_screen(&mut self) {
        let frame_buffer = self.bus.get_frame_buffer();

        for y in 0..320 {
            let offset = y * 640;

            for x in 0..640 {
                let pixel = frame_buffer.read((x / 10, y / 10));
                self.screen_buffer[offset + x] = if pixel {
                    0xFFFFFF
                } else {
                    0x0
                };
            }
        }

        self.win.update_with_buffer(&self.screen_buffer).unwrap();
    }

    fn key_mapping(key: minifb::Key) -> Option<u8> {
        use minifb::Key;

        match key {
            Key::NumPad0 => Some(0x0),
            Key::NumPad1 => Some(0x1),
            Key::NumPad2 => Some(0x2),
            Key::NumPad3 => Some(0x3),
            Key::NumPad4 => Some(0x4),
            Key::NumPad5 => Some(0x5),
            Key::NumPad6 => Some(0x6),
            Key::NumPad7 => Some(0x7),
            Key::NumPad8 => Some(0x8),
            Key::NumPad9 => Some(0x9),
            Key::A => Some(0xA),
            Key::B => Some(0xB),
            Key::C => Some(0xC),
            Key::D => Some(0xD),
            Key::E => Some(0xE),
            Key::F => Some(0xF),
            _ => None
        }
    }

    fn get_key(&mut self) -> Option<u8> {
        use minifb::KeyRepeat;

        match self.win.get_keys_pressed(KeyRepeat::Yes) {
            Some(keys) => if !keys.is_empty() {
                Window::key_mapping(keys[0])
            } else {
                None
            },
            None => None,
        }
    }
}

impl Context for Window {
    fn new(rom: ROM) -> Self {
        let mem = MainMemory::with_rom(rom);

        Window {
            cpu: CPU::new(),
            bus: Bus::new(mem),
            win: minifb::Window::new(
                "Chip8",
                640,
                320,
                minifb::WindowOptions::default()
            ).unwrap(),
            screen_buffer: vec![0; 320 * 640],
            instruction_timestamp: Instant::now(),
            display_timestamp: Instant::now(),
            timer_timestamp: Instant::now(),
            input_timestamp: Instant::now(),
        }
    }

    fn run(&mut self) {
        use minifb::Key::Escape;

        while self.win.is_open() && !self.win.is_key_down(Escape) {
            let key = self.get_key();

            if key.is_some() || Instant::now() - self.input_timestamp > Duration::from_millis(200) {
                self.bus.press_key(key);
                self.input_timestamp = Instant::now();
            }

            if Instant::now() - self.instruction_timestamp > Duration::from_millis(2) {
                self.cpu.tick(&mut self.bus);
                self.instruction_timestamp = Instant::now();
            }

            if Instant::now() - self.timer_timestamp > Duration::from_millis(17) {
                self.cpu.timer_decrement();
                self.timer_timestamp = Instant::now();
            }

            if Instant::now() - self.display_timestamp > Duration::from_millis(10) {
                self.draw_screen();
                self.display_timestamp = Instant::now();
            }
        }
    }
}
