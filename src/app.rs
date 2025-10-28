use anyhow::Result;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::EventLoop,
    keyboard::{KeyCode, PhysicalKey},
    window::Window,
};

use crate::hardware::{CPU, Gamepad};

#[derive(Default)]
pub struct App {
    window: Option<Window>,
    cpu: CPU,
    focused: bool,
}

impl App {
    pub fn focused(&self) -> bool {
        self.focused
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        self.window = Some(
            event_loop
                .create_window(Window::default_attributes())
                .unwrap(),
        )
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                println!("Close requested; stoppping");
                event_loop.exit();
            }
            WindowEvent::Focused(focused) => self.focused = focused,
            // WindowEvent::KeyboardInput { event, .. } => {
            //     if let PhysicalKey::Code(key_code) = event.physical_key {
            //         match key_code {
            //             KeyCode::Escape => event_loop.exit(),
            //             KeyCode::KeyW => self
            //                 .cpu
            //                 .set_gamepad_button(Gamepad::UP, event.state.is_pressed()),
            //             KeyCode::KeyA => self
            //                 .cpu
            //                 .set_gamepad_button(Gamepad::LEFT, event.state.is_pressed()),
            //             KeyCode::KeyS => self
            //                 .cpu
            //                 .set_gamepad_button(Gamepad::DOWN, event.state.is_pressed()),
            //             KeyCode::KeyD => self
            //                 .cpu
            //                 .set_gamepad_button(Gamepad::RIGHT, event.state.is_pressed()),
            //             _ => (),
            //         }
            //     }
            // }
            _ => (),
        }
    }
}
