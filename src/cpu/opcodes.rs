// Instructions see VR4300 manuel Figure 16-1
pub const SPECIAL: u32 = 0o00;
pub const REGIMM: u32 = 0o01;
pub const J: u32 = 0o02;
pub const JAL: u32 = 0o03;
pub const BEQ: u32 = 0o04;
pub const BNE: u32 = 0o05;
pub const BLEZ: u32 = 0o06;
pub const BGTZ: u32 = 0o07;

pub const ADDI: u32 = 0o10;
pub const ADDIU: u32 = 0o11;
pub const SLTI: u32 = 0o12;
pub const SLTIU: u32 = 0o13;
pub const ANDI: u32 = 0o14;
pub const ORI: u32 = 0o15;
pub const XORI: u32 = 0o16;
pub const LUI: u32 = 0o17;

pub const COP0: u32 = 0o20;
pub const COP1: u32 = 0o21;
pub const COP2: u32 = 0o23;
pub const BEQL: u32 = 0o24;
pub const BNEL: u32 = 0o25;
pub const BLEZL: u32 = 0o26;
pub const BGTZL: u32 = 0o27;

pub const DADDI: u32 = 0o30;
pub const DADDIU: u32 = 0o31;
pub const LDL: u32 = 0o32;
pub const LDR: u32 = 0o33;

pub const LB: u32 = 0o40;
pub const LH: u32 = 0o41;
pub const LWL: u32 = 0o42;
pub const LW: u32 = 0o43;
pub const LBU: u32 = 0o44;
pub const LHU: u32 = 0o45;
pub const LWR: u32 = 0o46;
pub const LWU: u32 = 0o47;

pub const SB: u32 = 0o50;
pub const SH: u32 = 0o51;
pub const SWL: u32 = 0o52;
pub const SW: u32 = 0o53;
pub const SDL: u32 = 0o45;
pub const SDR: u32 = 0o55;
pub const SWR: u32 = 0o56;
pub const CACHE: u32 = 0o57;

pub const LL: u32 = 0o60;
pub const LWC1: u32 = 0o61;
pub const LWC2: u32 = 0o62;
pub const LLD: u32 = 0o64;
pub const LDC1: u32 = 0o65;
pub const LDC2: u32 = 0o66;
pub const LD: u32 = 0o67;

pub const SC: u32 = 0o70;
pub const SWC1: u32 = 0o71;
pub const SWC2: u32 = 0o72;
pub const SCD: u32 = 0o74;
pub const SDC1: u32 = 0o75;
pub const SDC2: u32 = 0o76;
pub const SD: u32 = 0o77;

pub mod special_func {
    pub const SLL: u32 = 0o00;
    pub const SRL: u32 = 0o02;
    pub const SRA: u32 = 0o03;
    pub const SLLV: u32 = 0o04;
    pub const SRLV: u32 = 0o06;
    pub const SRAV: u32 = 0o07;

    pub const JR: u32 = 0o10;
    pub const JALR: u32 = 0o11;
    pub const SYSCALL: u32 = 0o14;
    pub const BREAK: u32 = 0o15;
    pub const SYNC: u32 = 0o17;

    pub const MFHI: u32 = 0o20;
    pub const MTHI: u32 = 0o21;
    pub const MFLO: u32 = 0o22;
    pub const MTLO: u32 = 0o23;
    pub const DSLLV: u32 = 0o24;
    pub const DSRLV: u32 = 0o26;
    pub const DSRAV: u32 = 0o27;

    pub const MULT: u32 = 0o30;
    pub const MULTU: u32 = 0o31;
    pub const DIV: u32 = 0o32;
    pub const DIVU: u32 = 0o34;
    pub const DMULT: u32 = 0o34;
    pub const DMULTU: u32 = 0o35;
    pub const DDIV: u32 = 0o36;
    pub const DDIVU: u32 = 0o37;

    pub const ADD: u32 = 0o40;
    pub const ADDU: u32 = 0o41;
    pub const SUB: u32 = 0o42;
    pub const SUBU: u32 = 0o43;
    pub const AND: u32 = 0o44;
    pub const OR: u32 = 0o45;
    pub const XOR: u32 = 0o46;
    pub const NOR: u32 = 0o47;

    pub const SLT: u32 = 0o52;
    pub const SLTU: u32 = 0o53;
    pub const DADD: u32 = 0o54;
    pub const DADDU: u32 = 0o55;
    pub const DSUB: u32 = 0o56;
    pub const DSUBU: u32 = 0o57;

    pub const TGE: u32 = 0o60;
    pub const TGEU: u32 = 0o61;
    pub const TLT: u32 = 0o62;
    pub const TLTU: u32 = 0o63;
    pub const TEQ: u32 = 0o64;
    pub const TNE: u32 = 0o66;

    pub const DSLL: u32 = 0o70;
    pub const DSRL: u32 = 0o72;
    pub const DSRA: u32 = 0o73;
    pub const DSLL32: u32 = 0o74;
    pub const DSRL32: u32 = 0o76;
    pub const DSRA32: u32 = 0o77;
}
