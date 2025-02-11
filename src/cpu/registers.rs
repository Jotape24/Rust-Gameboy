use crate::cpu::registers::flag_register::FlagsRegister;

mod flag_register;

struct Registers {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: FlagsRegister,
    h: u8,
    l: u8,
}


/// there is chances of read and write two registers at a time
/// the possible combinations are "af", "bc", "de" and "hl"
/// since "f" register is special, it has his own way to make things
impl Registers {

    // a -> 0a -> a0 -> af
    fn get_af(&self) -> u16 {
        let f_bits = u8::from(self.f.clone());

        (self.a as u16) << 8
            | (f_bits as u16)
    }


    fn set_af(&mut self, value: u16) {
        self.a = ((value & 0xFF00) >> 8) as u8; // af -> a0 -> 0a -> a
        self.f = FlagsRegister::from((value & 0xFF) as u8);
    }


    fn get_bc(&self) -> u16 {
        (self.b as u16) << 8
            | self.c as u16
    }

    fn set_bc(&mut self, value: u16) {
        self.b = ((value & 0xFF00) >> 8) as u8;
        self.c = (value & 0xFF) as u8;
    }

    fn get_de(&self) -> u16 {
        (self.d as u16) << 8
            | (self.e as u16)
    }

    fn get_hl(&self) -> u16 {
        (self.h as u16) << 8
            | (self.l as u16)
    }



}
