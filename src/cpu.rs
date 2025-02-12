use crate::cpu::instruction::{ArithmeticTarget, Instruction};
use crate::cpu::registers::flag_register::FlagsRegister;
use crate::cpu::registers::Registers;

mod registers;
mod instruction;

struct CPU {
    registers: Registers
}

impl CPU {
    fn execute(&mut self, inst: Instruction) {
        match inst {
            Instruction::ADD(target) => {
                match target {
                    ArithmeticTarget::A => {
                        let val = self.registers.get_a();
                        self.add(val);
                    },
                    ArithmeticTarget::B => {
                        let val = self.registers.get_b();
                        self.add(val);
                    },
                    ArithmeticTarget::C => {
                        let val = self.registers.get_c();
                        self.add(val);
                    },
                    ArithmeticTarget::D => {
                        let val = self.registers.get_d();
                        self.add(val);
                    },
                    ArithmeticTarget::E => {
                        let val = self.registers.get_e();
                        self.add(val);
                    },
                    ArithmeticTarget::H => {
                        let val = self.registers.get_h();
                        self.add(val);
                    },
                    ArithmeticTarget::L => {
                        let val = self.registers.get_l();
                        self.add(val);
                    },
                }
            }
            _ => {}
        }
    }

    // adds a value to the A register
    fn add(&mut self, value: u8) {
        let (new_value, did_overflow) = self.registers.get_a().overflowing_add(value);
        let zero = new_value == 0;
        let subtract = false;
        let carry = did_overflow;
        // Half Carry is set if adding the lower nibbles of the value and register A
        // together result in a value bigger than 0xF. If the result is larger than 0xF
        // than the addition caused a carry from the lower nibble to the upper nibble.
        let half_carry = (self.registers.get_a() & 0xF) + (value & 0xF) > 0xF;

        let new_f = FlagsRegister::new(zero, subtract, half_carry, carry);
        self.registers.set_f(new_f);
        self.registers.set_a(new_value);
    }
}

#[cfg(test)]
mod cpu_test {
    use super::*;

    #[test]
    fn test_execute_add () {

        let f = FlagsRegister::new(false, true, true, true);

        let register = Registers::new(1, 2, 3, 15, 5, f, 6, 7);

        let mut cpu = CPU { registers: register };

        cpu.execute(Instruction::ADD(ArithmeticTarget::D));

        assert_eq!(cpu.registers.get_a(), 16);

        let expected_f = FlagsRegister::new(false, false, true, false);

        assert_eq!(cpu.registers.get_f(), expected_f);



    }
}
