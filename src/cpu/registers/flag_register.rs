#[derive(Clone, Eq, PartialEq, Debug)]
pub struct FlagsRegister {
    zero: bool,
    subtract: bool,
    half_carry: bool,
    carry: bool
}
const ZERO_FLAG_BYTE_POSITION: u8 = 7;
const SUBTRACT_FLAG_BYTE_POSITION: u8 = 6;
const HALF_CARRY_FLAG_BYTE_POSITION: u8 = 5;
const CARRY_FLAG_BYTE_POSITION: u8 = 4;

impl From<FlagsRegister> for u8  {
    fn from(flag: FlagsRegister) -> u8 {
        (if flag.zero       { 1 } else { 0 }) << ZERO_FLAG_BYTE_POSITION |
            (if flag.subtract   { 1 } else { 0 }) << SUBTRACT_FLAG_BYTE_POSITION |
            (if flag.half_carry { 1 } else { 0 }) << HALF_CARRY_FLAG_BYTE_POSITION |
            (if flag.carry      { 1 } else { 0 }) << CARRY_FLAG_BYTE_POSITION
    }
}

impl From<u8> for FlagsRegister {
    fn from(byte: u8) -> Self {
        let zero = ((byte >> ZERO_FLAG_BYTE_POSITION) & 0b1) != 0;
        let subtract = ((byte >> SUBTRACT_FLAG_BYTE_POSITION) & 0b1) != 0;
        let half_carry = ((byte >> HALF_CARRY_FLAG_BYTE_POSITION) & 0b1) != 0;
        let carry = ((byte >> CARRY_FLAG_BYTE_POSITION) & 0b1) != 0;

        FlagsRegister {
            zero,
            subtract,
            half_carry,
            carry
        }
    }
}

impl FlagsRegister {
    pub fn new(zero: bool, subtract: bool, half_carry: bool, carry: bool) -> Self {
        FlagsRegister {
            zero,
            subtract,
            half_carry,
            carry
        }
    }
}

// getters and setters
impl FlagsRegister {
    pub fn get_zero(&self) -> bool {self.zero}

    pub fn set_zero(&mut self, value: bool) {self.zero = value;}

    pub fn get_subtract(&self) -> bool {self.subtract}

    pub fn set_subtract(&mut self, value: bool) {self.subtract = value;}

    pub fn get_half_carry(&self) -> bool {self.half_carry}

    pub fn set_half_carry(&mut self, value: bool) {self.half_carry = value;}

    pub fn get_carry(&self) -> bool {self.carry}

    pub fn set_carry(&mut self, value: bool) {self.carry = value;}
}
