use std::fs;
use std::time::Duration;

use emulator_8080::invaders::display::Display;
use emulator_8080::invaders::inputs::{Inputs, Player};
use emulator_8080::invaders::shift_register::ShiftRegister;
use emulator_8080::system::System;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

const FRAME_DURATION: u64 = 1_000_000 / 60;
const CLOCK_CYCLES_PER_FRAME: u64 = 20_000_000 / FRAME_DURATION;

fn main() {
    let test_rom = fs::read("./roms/INVADERS.COM").expect("Cannot load test ROM TST8080.COM!");

    let mut system = System::new();
    let mut inputs = Inputs::new();
    let mut shift_register = ShiftRegister::new();

    system.load_program(test_rom);

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("space invaders", 256 * 2, 224 * 2)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();
    let mut display = Display::new(&texture_creator, 256, 224, Color::RGB(192, 168, 1)).unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        inputs.reset();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => {
                    break 'running;
                },
                Event::KeyDown { keycode: Some(keycode), .. } => match keycode {
                    Keycode::Escape => break 'running,

                    Keycode::KpEnter => inputs.set_credit(),

                    // P1 controls
                    Keycode::A => inputs.set_player_left(Player::Player1),
                    Keycode::D => inputs.set_player_right(Player::Player1),
                    Keycode::W => inputs.set_player_shoot(Player::Player1),
                    Keycode::Z => inputs.set_player_start(Player::Player1),
                    
                    // P2 controls
                    Keycode::J => inputs.set_player_left(Player::Player2),
                    Keycode::L => inputs.set_player_right(Player::Player2),
                    Keycode::I => inputs.set_player_shoot(Player::Player2),
                    Keycode::N => inputs.set_player_start(Player::Player2),

                    _ => {}
                },
                _ => {}
            }
        }

        let shift_offset = system.get_output(2) & 0b111;

        shift_register.set_offset(shift_offset);
        // TODO: output flushing
        shift_register.push_value(value)

        system.set_input(1, 0b00001110);
        system.set_input(1, inputs.get_port_1());
        system.set_input(2, inputs.get_port_2());

        system.run(CLOCK_CYCLES_PER_FRAME as usize / 2);

        let display_bytes_first_half = system.read_memory_region(0x2400, 0x3fff);
        display.update_texture(&display_bytes_first_half);
        let _ = canvas.copy(display.get_texture(), None, None);

        canvas.present();
        ::std::thread::sleep(Duration::from_micros(FRAME_DURATION / 2));

        inputs.reset();
        system.interrupt(8);
        system.run(CLOCK_CYCLES_PER_FRAME as usize / 2);

        let display_bytes_second_half = system.read_memory_region(0x2400, 0x3fff);
        display.update_texture(&display_bytes_second_half);
        let _ = canvas.copy(display.get_texture(), None, None);

        canvas.present();
        ::std::thread::sleep(Duration::from_micros(FRAME_DURATION / 2));

        system.interrupt(10);
    }
}
