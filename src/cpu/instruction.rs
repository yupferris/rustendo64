use std::fmt;

use super::opcode::Opcode;

use num::FromPrimitive;

#[derive(Clone, Copy)]
pub struct Instruction(pub u32);

impl Instruction {
    #[inline(always)]
    pub fn opcode(&self) -> Opcode {
        Opcode::from_u32((self.0  >> 26) & 0b111111).unwrap_or_else(
            || panic!("Unrecognized instruction: {:#x}", self.0))
    }

    #[inline(always)]
    pub fn rs(&self) -> usize {
        ((self.0 >> 21) & 0b11111) as usize
    }

    #[inline(always)]
    pub fn rt(&self) -> usize {
        ((self.0 >> 16) & 0b11111) as usize
    }

    #[inline(always)]
    pub fn rd(&self) -> u32 {
        (self.0 >> 11) & 0b11111
    }

    #[inline(always)]
    pub fn imm(&self) -> u32 {
        self.0 & 0xffff
    }

    #[inline(always)]
    pub fn imm_sign_extended(&self) -> u64 {
        (self.imm() as i16) as u64
    }

    #[inline(always)]
    pub fn offset(&self) -> u32 {
        self.imm()
    }

    #[inline(always)]
    pub fn offset_sign_extended(&self) -> u64 {
        (self.offset() as i16) as u64
    }
}

impl fmt::Debug for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.opcode())
    }
}
