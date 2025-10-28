use bitflags::bitflags;

bitflags! {
    /// # Status Register (P) http://wiki.nesdev.com/w/index.php/Status_flags
    ///
    ///  7 6 5 4 3 2 1 0
    ///  N V _ B D I Z C
    ///  | |   | | | | +--- Carry Flag
    ///  | |   | | | +----- Zero Flag
    ///  | |   | | +------- Interrupt Disable
    ///  | |   | +--------- Decimal Mode (not used on NES)
    ///  | |   +----------- Break Command
    ///  | +--------------- Overflow Flag
    ///  +----------------- Negative Flag
    ///
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct CpuStatus: u8 {
        /// Carry is set during unsigned additions when the sum
        /// of the two products
        /// is greater than 255/0xff
        const CARRY        =  0b00000001;
        const ZERO         =  0b00000010;
        const INTERRUPT    =  0b00000100;
        const DECIMAL_MODE =  0b00001000;
        const BREAK        =  0b00010000;
        /// Overflow is set during signed additions and when the sum
        /// of the two numbers could be less than -128 or greater than 127.
        /// This can only occur when both parameters are negative or positive when
        /// represented as a signed number
        const OVERFLOW     =  0b01000000;
        const NEGATIVE     =  0b10000000;
    }
}

impl PartialEq<u8> for CpuStatus {
    fn eq(&self, other: &u8) -> bool {
        self.bits() == *other
    }
}

impl Default for CpuStatus {
    fn default() -> Self {
        Self::empty()
    }
}

impl CpuStatus {
    pub fn update_zero_and_negative_flags(&mut self, value: u8) {
        self.set(CpuStatus::ZERO, value == 0);
        self.set(CpuStatus::NEGATIVE, value & 0b1000_0000 != 0);
    }
}
