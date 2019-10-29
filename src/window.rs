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
        }
    }

    fn run(&mut self) {
        use minifb::Key::Escape;

        while self.win.is_open() && !self.win.is_key_down(Escape) {
            if Instant::now() - self.instruction_timestamp > Duration::from_millis(2) {
                self.cpu.tick(&mut self.bus);
                self.instruction_timestamp = Instant::now();
            }

            if Instant::now() - self.display_timestamp > Duration::from_millis(10) {
                self.draw_screen();
                self.display_timestamp = Instant::now();
            }
        }
    }
}
