use crate::cpu::registers::flag_register::FlagsRegister;

pub mod flag_register;

pub struct Registers {
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

    pub fn new(a: u8,
               b: u8,
               c: u8,
               d: u8,
               e: u8,
               f: FlagsRegister,
               h: u8,
               l: u8,) -> Registers {

        Registers {
            a,
            b,
            c,
            d,
            e,
            f,
            h,
            l
        }
    }

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


impl Registers {
    pub(crate) fn set_a(&mut self, value: u8) {
        self.a = value;
    }

    pub fn get_a(&self) -> u8 {self.a}

    fn set_b(&mut self, value: u8) {
        self.b = value;
    }

    pub fn get_b(&self) -> u8 {self.b}

    fn set_c(&mut self, value: u8) {
        self.c = value;
    }

    pub fn get_c(&self) -> u8 {self.c}

    fn set_d(&mut self, value: u8) {
        self.d = value;
    }

    pub fn get_d(&self) -> u8 {self.d}

    fn set_e(&mut self, value: u8) {
        self.e = value;
    }

    pub fn get_e(&self) -> u8 {self.e}

    pub fn set_f(&mut self, value: FlagsRegister ) {
        self.f = value;
    }

    pub fn get_f(&self) -> FlagsRegister {self.f.clone()}

    fn set_h(&mut self, value: u8) {
        self.h = value;
    }

    pub fn get_h(&self) -> u8 {self.h}

    fn set_l(&mut self, value: u8) {
        self.l = value;
    }

    pub fn get_l(&self) -> u8 {self.l}
}
