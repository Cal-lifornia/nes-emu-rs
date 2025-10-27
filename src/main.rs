use nes_emu_rs::{cpu::CPU, snake::SNAKE_CODE};
use rand::Rng;
use sdl2::{
    EventPump,
    event::Event,
    keyboard::Keycode,
    pixels::{Color, PixelFormatEnum},
};

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("Snake Game", (32.0 * 10.0) as u32, (32.0 * 10.0) as u32)
        .position_centered()
        .build()
        .expect("window");

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    canvas.set_scale(10.0, 10.0).expect("set scale");

    let creator = canvas.texture_creator();
    let mut texture = creator
        .create_texture_target(PixelFormatEnum::RGB24, 32, 32)
        .expect("set to valid texture target");

    let mut cpu = CPU::default();
    cpu.load(&SNAKE_CODE);
    cpu.reset();

    let mut screen_state = [0_u8; 32 * 3 * 32];
    let mut rng = rand::thread_rng();

    cpu.run_with_callback(move |cpu| {
        handle_user_input(cpu, &mut event_pump);
        cpu.mem_write(0xfe, rng.gen_range(1, 16));

        if read_screen_state(cpu, &mut screen_state) {
            texture
                .update(None, &screen_state, 32 * 3)
                .expect("updated texture");
            canvas.copy(&texture, None, None).unwrap();
            canvas.present();
        }
        ::std::thread::sleep(std::time::Duration::new(0, 70_000));
    });
}

fn handle_user_input(cpu: &mut CPU, event_pump: &mut EventPump) {
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => std::process::exit(0),

            Event::KeyDown {
                keycode: Some(Keycode::W),
                ..
            } => {
                cpu.mem_write(0xff, 0x77);
            }

            Event::KeyDown {
                keycode: Some(Keycode::A),
                ..
            } => {
                cpu.mem_write(0xff, 0x73);
            }

            Event::KeyDown {
                keycode: Some(Keycode::S),
                ..
            } => {
                cpu.mem_write(0xff, 0x61);
            }

            Event::KeyDown {
                keycode: Some(Keycode::D),
                ..
            } => {
                cpu.mem_write(0xff, 0x64);
            }
            _ => {}
        }
    }
}

fn colour(byte: u8) -> Color {
    match byte {
        0 => Color::BLACK,
        1 => Color::WHITE,
        2 | 9 => Color::GREY,
        3 | 10 => Color::RED,
        4 | 11 => Color::GREEN,
        5 | 12 => Color::BLUE,
        6 | 13 => Color::MAGENTA,
        7 | 14 => Color::YELLOW,
        _ => Color::CYAN,
    }
}

fn read_screen_state(cpu: &CPU, frame: &mut [u8; 32 * 3 * 32]) -> bool {
    let mut frame_idx = 0;
    let mut update = false;
    for i in 0x0200..0x0600 {
        let colour_idx = cpu.mem_read(i as u16);
        let (b1, b2, b3) = colour(colour_idx).rgb();
        if frame[frame_idx] != b1 || frame[frame_idx + 1] != b2 || frame[frame_idx + 2] != b3 {
            frame[frame_idx] = b1;
            frame[frame_idx + 1] = b2;
            frame[frame_idx + 2] = b3;
            update = true;
        }
        frame_idx += 3;
    }
    update
}
