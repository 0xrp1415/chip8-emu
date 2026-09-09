use std::{fs, thread, time::Instant};


mod display;
mod keybinds;
mod instruction;
mod cpu;
mod sprites;
mod io;

fn main() {
    let mut cpu = cpu::cpu::new();

    // Gets the program path from command line arguments
    let args = std::env::args().collect::<Vec<String>>();
    let program_path = &args[1];
    println!("Loading program from {}", program_path);
    let program_data = fs::read(program_path).expect("Failed to read program file");

    cpu.load_sprites();
    cpu.load_program(&program_data);
    let sdl_context = sdl2::init().unwrap();
    let mut display = display::Display::new(640, 320,10,&sdl_context);
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut is_running = true;
    let mut last_timer = Instant::now();
    
    while is_running {
        cpu.decode();

        let now = Instant::now();
        if now.duration_since(last_timer).as_millis() >= 16 {
            cpu.decrement_timers();
            last_timer = now;
        }

        display.draw(&cpu.io);
        for event in event_pump.poll_iter() {
            cpu.io.handle_io(&event, &mut is_running);
        }
        thread::sleep(std::time::Duration::from_millis(2));        
    }
}
