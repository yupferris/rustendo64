enum_from_primitive! {
    #[derive(Debug)]
    pub enum Opcode {
        Andi = 0b001100,
        Ori =  0b001101,

        Lui =  0b001111,

        Mtc0 = 0b010000,

        Beql = 0b010100,

        Lw =   0b100011
    }
}
