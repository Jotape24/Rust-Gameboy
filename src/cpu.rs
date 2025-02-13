use crate::cpu::instruction::{ArithmeticTarget, ArithmeticTarget16, Instruction};
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
            Instruction::ADDHL(target) => {
                match target {
                    ArithmeticTarget16::BC => {
                        let val = self.registers.get_bc();
                        self.add_hl(val)
                    }
                    ArithmeticTarget16::DE => {
                        let val = self.registers.get_de();
                        self.add_hl(val)
                    }
                    ArithmeticTarget16::HL => {
                        let val = self.registers.get_hl();
                        self.add_hl(val)
                    }
                }
            }
            Instruction::ADC(target) => {
                match target {
                    ArithmeticTarget::A => {
                        let val = self.registers.get_a();
                        self.add_carry(val);
                    },
                    ArithmeticTarget::B => {
                        let val = self.registers.get_b();
                        self.add_carry(val);
                    },
                    ArithmeticTarget::C => {
                        let val = self.registers.get_c();
                        self.add_carry(val);
                    },
                    ArithmeticTarget::D => {
                        let val = self.registers.get_d();
                        self.add_carry(val);
                    },
                    ArithmeticTarget::E => {
                        let val = self.registers.get_e();
                        self.add_carry(val);
                    },
                    ArithmeticTarget::H => {
                        let val = self.registers.get_h();
                        self.add_carry(val);
                    },
                    ArithmeticTarget::L => {
                        let val = self.registers.get_l();
                        self.add_carry(val);
                    },
                }
            }
            Instruction::SUB(target) => {
                match target {
                    ArithmeticTarget::A => {
                        let val = self.registers.get_a();
                        self.sub(val);
                    },
                    ArithmeticTarget::B => {
                        let val = self.registers.get_b();
                        self.sub(val);
                    },
                    ArithmeticTarget::C => {
                        let val = self.registers.get_c();
                        self.sub(val);
                    },
                    ArithmeticTarget::D => {
                        let val = self.registers.get_d();
                        self.sub(val);
                    },
                    ArithmeticTarget::E => {
                        let val = self.registers.get_e();
                        self.sub(val);
                    },
                    ArithmeticTarget::H => {
                        let val = self.registers.get_h();
                        self.sub(val);
                    },
                    ArithmeticTarget::L => {
                        let val = self.registers.get_l();
                        self.sub(val);
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

    // adds a value to the HL register
    fn add_hl(&mut self,value: u16) {
        let (new_value, did_overflow) = self.registers.get_hl().overflowing_add(value);
        let zero = self.registers.get_f().get_zero();
        let subtract = false;
        let carry = did_overflow;

        // Half Carry is set if adding the lower nibbles of the value and register HL
        // together result in a value bigger than 0xFFF. is the pass from the lower nibble from the
        // higher byte to the upper nibble
        let half_carry = (self.registers.get_hl() & 0xFFF) + (value & 0xFFF) > 0xFFF;

        let new_f = FlagsRegister::new(zero, subtract, half_carry, carry);
        self.registers.set_f(new_f);
        self.registers.set_hl(new_value);
    }

    // adds a value + carry to the A register
    fn add_carry(&mut self, value: u8) {
        let carry_val = u8::from(self.registers.get_f().get_carry()); // Obtener el carry actual

        let (partial_sum, carry1) = self.registers.get_a().overflowing_add(value);
        let (new_value, carry2) = partial_sum.overflowing_add(carry_val);

        let zero = new_value == 0;
        let subtract = false;
        let carry = carry1 || carry2;

        let half_carry = ((self.registers.get_a() & 0xF) + (value & 0xF) + carry_val) > 0xF;

        let new_f = FlagsRegister::new(zero, subtract, half_carry, carry);
        self.registers.set_f(new_f);
        self.registers.set_a(new_value);
    }


    fn sub(&mut self, value: u8) {
        let (new_value, did_overflow) = self.registers.get_a().overflowing_sub(value);
        let zero = new_value == 0;
        let subtract = true;
        let carry = did_overflow;
        // if the lower nibble of value is bigger than the lower nibble of A
        let half_carry = (self.registers.get_a() & 0xF) < (value & 0xF);

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

    #[test]
    fn test_execute_add_hl () {

        let f = FlagsRegister::new(false, true, true, true);

        let register = Registers::new(1, 2, 3, 255, 4, f, 1, 2);

        assert_eq!(register.get_hl(), 0b0000000100000010);
        assert_eq!(register.get_de(), 0b1111111100000100);

        let mut cpu = CPU { registers: register };

        cpu.execute(Instruction::ADDHL(ArithmeticTarget16::DE));

        assert_eq!(cpu.registers.get_hl(), 0b0000000000000110);

        let expected_f = FlagsRegister::new(false, false, true, true);

        assert_eq!(cpu.registers.get_f(), expected_f);
    }


    #[test]
    fn test_execute_add_carry () {
        let f = FlagsRegister::new(false, true, true, true);

        let register = Registers::new(0b1111_1111, 2, 3, 0b0000_0010, 5, f, 6, 7);

        let mut cpu = CPU { registers: register };

        cpu.execute(Instruction::ADC(ArithmeticTarget::D));

        let expected_f = FlagsRegister::new(false, false, true, true);

        assert_eq!(cpu.registers.get_f(), expected_f);

        assert_eq!(cpu.registers.get_a(), 0b0000_0010);
    }

    #[test]
    fn test_execute_sub () {
        let f = FlagsRegister::new(false, true, true, true);

        let register = Registers::new(0b1111_0001, 2, 3, 0b0000_0010, 5, f, 6, 7);

        let mut cpu = CPU { registers: register };

        cpu.execute(Instruction::SUB(ArithmeticTarget::D));

        let expected_f = FlagsRegister::new(false, true, true, false);

        assert_eq!(cpu.registers.get_f(), expected_f);

        assert_eq!(cpu.registers.get_a(), 0b01110_1111);
    }
}
