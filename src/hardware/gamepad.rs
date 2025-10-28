use bitflags::bitflags;

use crate::hardware::CPU;

const GAMEPAD_ADDRESS: u8 = 0xFF;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Gamepad: u8 {
        const A         = 0b10000000;
        const B         = 0b01000000;
        const SELECT    = 0b00100000;
        const START     = 0b00010000;
        // const UP        = 0b00001000;
        // const DOWN      = 0b00000100;
        // const LEFT      = 0b00000010;
        // const RIGHT     = 0b00000001;
        const UP     = 0x77;
        const DOWN   = 0x73;
        const LEFT   = 0x61;
        const RIGHT  = 0x64;

    }
}

impl CPU {
    pub fn set_gamepad_button(&mut self, gamepad: Gamepad) {
        self.mem_write(GAMEPAD_ADDRESS as u16, gamepad.bits());
    }
}
