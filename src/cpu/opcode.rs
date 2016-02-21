enum_from_primitive! {
    #[derive(Debug)]
    pub enum Opcode {
        Addi =  0b001000,
        Addiu = 0b001001,

        Andi =  0b001100,
        Ori =   0b001101,

        Lui =   0b001111,
        Mtc0 =  0b010000,

        Bne =   0b000101,

        Beql =  0b010100,
        Bnel =  0b010101,

        Lw =    0b100011,

        Sw =    0b101011
    }
}
