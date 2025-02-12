use crate::cpu::instruction::{ArithmeticTarget, Instruction};
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
                        let new_a = self.add(val);
                        self.registers.set_a(new_a);
                    },
                    ArithmeticTarget::B => {
                        let val = self.registers.get_b();
                        let new_a = self.add(val);
                        self.registers.set_a(new_a);
                    },
                    ArithmeticTarget::C => {
                        let val = self.registers.get_c();
                        let new_a = self.add(val);
                        self.registers.set_a(new_a);
                    },
                    ArithmeticTarget::D => {
                        let val = self.registers.get_d();
                        let new_a = self.add(val);
                        self.registers.set_a(new_a);
                    },
                    ArithmeticTarget::E => {
                        let val = self.registers.get_e();
                        let new_a = self.add(val);
                        self.registers.set_a(new_a);
                    },
                    ArithmeticTarget::H => {
                        let val = self.registers.get_h();
                        let new_a = self.add(val);
                        self.registers.set_a(new_a);
                    },
                    ArithmeticTarget::L => {
                        let val = self.registers.get_l();
                        let new_a = self.add(val);
                        self.registers.set_a(new_a);
                    },
                }
            }
            _ => {}
        }
    }

    // adds a value to the A register
    fn add(&mut self, value: u8) -> u8 {
        let (new_value, did_overflow) = self.registers.get_a().overflowing_add(value);
        // TODO: set flags
        new_value
    }
}