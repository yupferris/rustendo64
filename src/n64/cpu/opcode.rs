enum_from_primitive! {
    #[derive(Debug)]
    pub enum Opcode {
        Special = 0b000000,
        RegImm =  0b000001,

        Addi =    0b001000,
        Addiu =   0b001001,

        Andi =    0b001100,
        Ori =     0b001101,

        Lui =     0b001111,
        Mtc0 =    0b010000,

        Beq =     0b000100,
        Bne =     0b000101,

        Beql =    0b010100,
        Bnel =    0b010101,

        Lw =      0b100011,

        Sw =      0b101011,
    }
}

enum_from_primitive! {
    #[derive(Debug)]
    pub enum SpecialOpcode {
        Sll =   0b000000,

        Srl =   0b000010,

        Sllv =  0b000100,

        Srlv =  0b000110,

        Jr =    0b001000,

        Multu = 0b011001,

        Mfhi =  0b010000,
        Mflo =  0b010010,

        Addu =  0b100001,

        Subu =  0b100011,
        And =   0b100100,
        Or =    0b100101,
        Xor =   0b100110,

        Sltu =  0b101011,
    }
}

enum_from_primitive! {
    #[derive(Debug)]
    pub enum RegImmOpcode {
        Bgezal = 0b10001,
    }
}
