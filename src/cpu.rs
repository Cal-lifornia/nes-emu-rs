use bitflags::{Flags, bitflags};

use crate::opcode::{AddressingMode, CPU_OPS_CODES, Instruction};

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
    fn update_zero_and_negative_flags(&mut self, value: u8) {
        self.set(CpuStatus::ZERO, value == 0);
        self.set(CpuStatus::NEGATIVE, value & 0b1000_0000 != 0);
    }
}

const STACK_RESET: u8 = 0xFF;
const STACK: u16 = 0x0100;

#[allow(clippy::upper_case_acronyms)]
pub struct CPU {
    pub register_a: u8,
    pub register_x: u8,
    pub register_y: u8,
    pub status: CpuStatus,
    pub program_counter: u16,
    pub stack_pointer: u8,
    memory: [u8; 0xFFFF],
}

impl Default for CPU {
    fn default() -> Self {
        Self {
            register_a: 0,
            register_x: 0,
            register_y: 0,
            status: CpuStatus::default(),
            program_counter: 0,
            stack_pointer: STACK_RESET,
            memory: [0; 0xFFFF],
        }
    }
}

impl CPU {
    fn reset(&mut self) {
        self.register_a = 0;
        self.register_x = 0;
        self.register_y = 0;
        self.stack_pointer = STACK_RESET;
        self.status.clear();

        self.program_counter = self.mem_read_u16(0xFFFC)
    }

    fn mem_read(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    fn mem_write(&mut self, addr: u16, data: u8) {
        self.memory[addr as usize] = data;
    }

    // Returns the memory at position as little endian
    fn mem_read_u16(&self, pos: u16) -> u16 {
        let lo = self.mem_read(pos);
        let hi = self.mem_read(pos + 1);
        u16::from_be_bytes([hi, lo])
    }

    // Writes the data as correct little endian into memory
    fn mem_write_u16(&mut self, pos: u16, data: u16) {
        let le_bits = data.to_le_bytes();
        self.mem_write(pos, le_bits[0]);
        self.mem_write(pos + 1, le_bits[1]);
    }

    fn stack_push(&mut self, value: u8) {
        self.mem_write(STACK + self.stack_pointer as u16, value);
        self.stack_pointer = self.stack_pointer.wrapping_sub(1);
    }

    fn stack_push_u16(&mut self, value: u16) {
        let le_bits = value.to_le_bytes();
        self.stack_push(le_bits[0]);
        self.stack_push(le_bits[1]);
    }

    fn stack_pop(&mut self) -> u8 {
        self.stack_pointer = self.stack_pointer.wrapping_add(1);
        self.mem_read(STACK + self.stack_pointer as u16)
    }

    fn stack_pop_u16(&mut self) -> u16 {
        let lo = self.stack_pop();
        let hi = self.stack_pop();

        u16::from_le_bytes([hi, lo])
    }

    pub fn load_and_run(&mut self, program: &[u8]) {
        self.load(program);
        self.reset();
        self.run();
    }

    pub fn load(&mut self, program: &[u8]) {
        self.memory[0x8000..(0x8000 + program.len())].copy_from_slice(program);
        self.mem_write_u16(0xFFFC, 0x8000);
    }

    fn add_to_register_a(&mut self, data: u8) {
        let sum = self.register_a as u16
            + data as u16
            + (if self.status.contains(CpuStatus::CARRY) {
                1
            } else {
                0
            }) as u16;

        // If the sum is greater than 255 set carry flag
        self.status.set(CpuStatus::CARRY, sum > 0xff);

        let result = sum as u8;

        //
        let overflow = (self.register_a ^ result) & (data ^ result) & 0x80 != 0;

        self.status.set(CpuStatus::OVERFLOW, overflow);

        self.set_register_a(result);
    }

    fn set_register_a(&mut self, value: u8) {
        self.register_a = value;
        self.status.update_zero_and_negative_flags(self.register_a);
    }
    fn set_register_x(&mut self, value: u8) {
        self.register_x = value;
        self.status.update_zero_and_negative_flags(self.register_x);
    }
    fn set_register_y(&mut self, value: u8) {
        self.register_y = value;
        self.status.update_zero_and_negative_flags(self.register_y);
    }

    fn get_operand_address(&mut self, mode: &AddressingMode) -> u16 {
        match mode {
            AddressingMode::Immediate => self.program_counter,
            AddressingMode::ZeroPage => self.mem_read(self.program_counter) as u16,
            AddressingMode::ZeroPageX => {
                let pos = self.mem_read(self.program_counter);
                pos.wrapping_add(self.register_x) as u16
            }
            AddressingMode::ZeroPageY => {
                let pos = self.mem_read(self.program_counter);
                pos.wrapping_add(self.register_y) as u16
            }
            AddressingMode::Absolute => self.mem_read_u16(self.program_counter),
            AddressingMode::AbsoluteX => {
                let base = self.mem_read_u16(self.program_counter);
                base.wrapping_add(self.register_x as u16)
            }
            AddressingMode::AbsoluteY => {
                let base = self.mem_read_u16(self.program_counter);
                base.wrapping_add(self.register_y as u16)
            }
            AddressingMode::IndirectX => {
                let base = self.mem_read(self.program_counter);

                let ptr: u8 = base.wrapping_add(self.register_x);
                let lo = self.mem_read(ptr as u16);
                let hi = self.mem_read(ptr.wrapping_add(1) as u16);

                u16::from_be_bytes([hi, lo])
            }
            AddressingMode::IndirectY => {
                let base = self.mem_read(self.program_counter);

                let lo = self.mem_read(base as u16);
                let hi = self.mem_read(base.wrapping_add(1) as u16);

                let deref_base = u16::from_be_bytes([hi, lo]);
                deref_base.wrapping_add(self.register_y as u16)
            }
            AddressingMode::Other => {
                panic!("mode {:?} not supported", mode)
            }
        }
    }

    pub fn run(&mut self) {
        use Instruction::*;
        loop {
            let opscode = self.mem_read(self.program_counter);
            self.program_counter += 1;

            let Some(command) = CPU_OPS_CODES.get(&opscode) else {
                panic!("no command found for opcode")
            };

            match &command.instruction {
                ADC => {
                    let addr = self.get_operand_address(&command.addressing_mode);
                    let value = self.mem_read(addr);
                    self.add_to_register_a(value);
                }
                ASL => {
                    let accumulator = matches!(command.addressing_mode, AddressingMode::Other);
                    let (addr, mut value) = if accumulator {
                        (0, self.register_a)
                    } else {
                        let addr = self.get_operand_address(&command.addressing_mode);
                        (addr, self.mem_read(addr))
                    };

                    self.status.set(CpuStatus::CARRY, value >> 7 == 1);

                    value <<= 1;

                    if accumulator {
                        self.set_register_a(value);
                    } else {
                        self.mem_write(addr, value);
                        self.status.update_zero_and_negative_flags(value);
                    }
                }
                AND => {
                    let addr = self.get_operand_address(&command.addressing_mode);
                    let value = self.mem_read(addr);
                    self.set_register_a(self.register_a & value);
                }
                BIT => {
                    let addr = self.get_operand_address(&command.addressing_mode);
                    let value = self.mem_read(addr);

                    self.status.update_zero_and_negative_flags(value);

                    self.status
                        .set(CpuStatus::OVERFLOW, value & 0b01000000 != 0);
                }

                BRK => {
                    self.status.insert(CpuStatus::BREAK);
                    return;
                }
                CLC => {
                    self.status.remove(CpuStatus::CARRY);
                }
                CLI => {
                    self.status.remove(CpuStatus::INTERRUPT);
                }
                CLV => {
                    self.status.remove(CpuStatus::OVERFLOW);
                }
                CMP => {
                    let addr = self.get_operand_address(&command.addressing_mode);
                    let value = self.mem_read(addr);

                    self.status.set(CpuStatus::CARRY, self.register_a >= value);
                    self.status.set(CpuStatus::ZERO, self.register_a == value);
                    self.status.set(CpuStatus::NEGATIVE, value & 0x80 != 0);
                }
                CPX => {
                    let addr = self.get_operand_address(&command.addressing_mode);
                    let value = self.mem_read(addr);

                    self.status.set(CpuStatus::CARRY, self.register_x >= value);
                    self.status.set(CpuStatus::ZERO, self.register_x == value);
                    self.status.set(CpuStatus::NEGATIVE, value & 0x80 != 0);
                }
                CPY => {
                    let addr = self.get_operand_address(&command.addressing_mode);
                    let value = self.mem_read(addr);

                    self.status.set(CpuStatus::CARRY, self.register_y >= value);
                    self.status.set(CpuStatus::ZERO, self.register_y == value);
                    self.status.set(CpuStatus::NEGATIVE, value & 0x80 != 0);
                }
                DEC => {
                    let addr = self.get_operand_address(&command.addressing_mode);
                    let mut value = self.mem_read(addr);

                    value = value.wrapping_sub(1);
                    self.mem_write(addr, value);
                    self.status.update_zero_and_negative_flags(value);
                }
                DEX => {
                    let value = self.register_x.wrapping_sub(1);
                    self.set_register_x(value);
                }
                DEY => {
                    let value = self.register_y.wrapping_sub(1);
                    self.set_register_y(value);
                }
                EOR => {
                    let addr = self.get_operand_address(&command.addressing_mode);
                    let value = self.mem_read(addr);

                    self.set_register_a(self.register_a ^ value);
                }
                INC => {
                    let addr = self.get_operand_address(&command.addressing_mode);
                    let mut value = self.mem_read(addr);

                    value = value.wrapping_add(1);
                    self.mem_write(addr, value);
                    self.status.update_zero_and_negative_flags(value);
                }
                INX => {
                    self.set_register_x(self.register_x.wrapping_add(1));
                }
                INY => {
                    self.set_register_y(self.register_y.wrapping_sub(1));
                }

                JMP => {
                    let addr = match &command.addressing_mode {
                        AddressingMode::Absolute => {
                            self.get_operand_address(&command.addressing_mode)
                        }
                        AddressingMode::Other => {
                            let addr = self.mem_read_u16(self.program_counter);
                            if addr & 0x00FF == 0x00FF {
                                let lo = self.mem_read(addr);
                                let hi = self.mem_read(addr & 0xFF00);
                                u16::from_be_bytes([hi, lo])
                            } else {
                                self.mem_read_u16(addr)
                            }
                        }

                        _ => unreachable!(),
                    };
                    self.program_counter = addr;
                }
                JSR => {
                    self.stack_push_u16(self.program_counter + 2 - 1);
                    let target_address = self.mem_read_u16(self.program_counter);
                    self.program_counter = target_address;
                }
                LDA => {
                    let addr = self.get_operand_address(&command.addressing_mode);
                    let value = self.mem_read(addr);
                    self.set_register_a(value);
                }
                LDX => {
                    let addr = self.get_operand_address(&command.addressing_mode);
                    let value = self.mem_read(addr);
                    self.set_register_x(value);
                }
                LDY => {
                    let addr = self.get_operand_address(&command.addressing_mode);
                    let value = self.mem_read(addr);
                    self.set_register_y(value);
                }
                LSR => {
                    let accumulator = matches!(command.addressing_mode, AddressingMode::Other);
                    let (addr, mut value) = if accumulator {
                        (0, self.register_a)
                    } else {
                        let addr = self.get_operand_address(&command.addressing_mode);
                        (addr, self.mem_read(addr))
                    };

                    self.status.set(CpuStatus::CARRY, value & 1 == 1);

                    value >>= 1;

                    if accumulator {
                        self.set_register_a(value);
                    } else {
                        self.mem_write(addr, value);
                        self.status.update_zero_and_negative_flags(value);
                    }
                }
                NOP => {}
                ORA => {
                    let addr = self.get_operand_address(&command.addressing_mode);
                    let value = self.mem_read(addr);
                    self.set_register_a(self.register_a | value);
                }
                PHA => {
                    self.stack_push(self.register_a);
                }
                PHP => {
                    self.stack_push(self.status.bits());
                }
                PLA => {
                    let value = self.stack_pop();
                    self.set_register_a(value);
                }
                PLP => {
                    let value = self.stack_pop();
                    self.status = CpuStatus::from_bits_truncate(value);
                }
                ROL => {
                    let accumulator = matches!(command.addressing_mode, AddressingMode::Other);
                    let (addr, mut value) = if accumulator {
                        (0, self.register_a)
                    } else {
                        let addr = self.get_operand_address(&command.addressing_mode);
                        (addr, self.mem_read(addr))
                    };

                    let carry: u8 = if self.status.contains(CpuStatus::CARRY) {
                        1
                    } else {
                        0
                    };

                    self.status.set(CpuStatus::CARRY, value & 0x80 == 0x80);

                    value <<= 1;
                    value |= carry;

                    if accumulator {
                        self.set_register_a(value);
                    } else {
                        self.mem_write(addr, value);
                        self.status.update_zero_and_negative_flags(value);
                    }
                }

                ROR => {
                    let accumulator = matches!(command.addressing_mode, AddressingMode::Other);
                    let (addr, mut value) = if accumulator {
                        (0, self.register_a)
                    } else {
                        let addr = self.get_operand_address(&command.addressing_mode);
                        (addr, self.mem_read(addr))
                    };

                    let carry: u8 = if self.status.contains(CpuStatus::CARRY) {
                        0x80
                    } else {
                        0
                    };

                    self.status.set(CpuStatus::CARRY, value & 1 == 1);

                    value >>= 1;
                    value |= carry;

                    if accumulator {
                        self.set_register_a(value);
                    } else {
                        self.mem_write(addr, value);
                        self.status.update_zero_and_negative_flags(value);
                    }
                }

                RTI => {
                    let value = self.stack_pop();
                    self.status = CpuStatus::from_bits_truncate(value);

                    self.program_counter = self.stack_pop_u16();
                }
                RTS => {
                    self.program_counter = self.stack_pop_u16() + 1;
                }
                // A - B = A + (-B)
                // -B = !B + 1
                SBC => {
                    let addr = self.get_operand_address(&command.addressing_mode);
                    let data = self.mem_read(addr);
                    self.add_to_register_a((data as i8).wrapping_neg().wrapping_sub(1) as u8);
                }
                SEC => {
                    self.status.insert(CpuStatus::CARRY);
                }
                SEI => {
                    self.status.insert(CpuStatus::INTERRUPT);
                }
                STA => {
                    let addr = self.get_operand_address(&command.addressing_mode);
                    self.mem_write(addr, self.register_a);
                }
                STX => {
                    let addr = self.get_operand_address(&command.addressing_mode);
                    self.mem_write(addr, self.register_x);
                }
                STY => {
                    let addr = self.get_operand_address(&command.addressing_mode);
                    self.mem_write(addr, self.register_y);
                }
                TAX => {
                    self.set_register_x(self.register_a);
                }
                TAY => {
                    self.set_register_y(self.register_a);
                }
                TSX => {
                    let value = self.stack_pop();
                    self.set_register_x(value);
                }
                TXA => {
                    self.set_register_a(self.register_x);
                }
                _ => todo!(),
            }

            self.program_counter += (command.len - 1) as u16;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0xa9_lda_immediate_load_data() {
        let mut cpu = CPU::default();
        cpu.load_and_run(&[0xa9, 0x05, 0x00]);
        assert_eq!(cpu.register_a, 0x05);
        assert!(cpu.status & CpuStatus::ZERO == 0b00);
        assert!(cpu.status & CpuStatus::NEGATIVE == 0);
    }

    #[test]
    fn test_0xa9_lda_zero_flag() {
        let mut cpu = CPU::default();
        cpu.load_and_run(&[0xa9, 0x00, 0x00]);
        assert!(cpu.status & CpuStatus::ZERO == 0b10);
    }

    #[test]
    fn test_lda_negative_flag() {
        let mut cpu = CPU::default();
        cpu.load_and_run(&[0xa9, 0xA5, 0x00]);
        assert!(cpu.status.contains(CpuStatus::NEGATIVE))
    }
    #[test]
    fn test_5_ops_working_together() {
        let mut cpu = CPU::default();
        cpu.load_and_run(&[0xa9, 0xc0, 0xaa, 0xe8, 0x00]);

        assert_eq!(cpu.register_x, 0xc1)
    }

    #[test]
    fn test_inx_overflow() {
        let mut cpu = CPU::default();
        // #[TODO] Use load() then reset() then modify for tests, then run()
        cpu.load_and_run(&[0xa9, 255, 0xaa, 0xe8, 0xe8, 0x00]);

        assert_eq!(cpu.register_x, 1)
    }

    #[test]
    fn test_lda_from_memory() {
        let mut cpu = CPU::default();
        cpu.mem_write(0x10, 0x55);
        cpu.load_and_run(&[0xa5, 0x10, 0x00]);
        assert_eq!(cpu.register_a, 0x55)
    }

    #[test]
    fn test_asl() {
        let mut cpu = CPU::default();
        cpu.load_and_run(&[0xa9, 0b11111110, 0x0A, 0x00]);

        // Confirms that the bits were shifted correctly
        assert_eq!(cpu.register_a, 0b11111100);

        // Confirms that the carry flag was correctly set
        assert!(cpu.status.contains(CpuStatus::CARRY))
    }

    #[test]
    fn test_rol() {
        let mut cpu = CPU::default();
        // Adds value to accumulator, sets the carry flag then runs the ROL Op
        cpu.load_and_run(&[0xa9, 0b01111110, 0x38, 0x2A, 0x00]);

        // Confirms that bits were shifted correctly and that
        // the carry flag set bit 0 correctly
        assert_eq!(cpu.register_a, 0b11111101);

        // Confirms that the carry flag copied the value from bit 7
        assert!(!cpu.status.contains(CpuStatus::CARRY))
    }
}
