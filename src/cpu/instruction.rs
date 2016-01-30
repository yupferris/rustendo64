use std::fmt;

// Opcodes
pub const LUI:     u32 = 0b001111;
pub const LW:      u32 = 0b100011;
pub const LWU:     u32 = 0b100111;
pub const SW:      u32 = 0b101011;
pub const ADDI:    u32 = 0b001000;
pub const ADDIU:   u32 = 0b001001;
pub const ANDI:    u32 = 0b001100;
pub const ORI:     u32 = 0b001101;
pub const XORI:    u32 = 0b001110;
pub const SLTI:    u32 = 0b001010;
pub const SLTIU:   u32 = 0b001011;
pub const J:       u32 = 0b000010;
pub const JAL:     u32 = 0b000011;
pub const BEQ:     u32 = 0b000100;
pub const BEQL:    u32 = 0b010100;
pub const BNE:     u32 = 0b000101;
pub const BNEL:    u32 = 0b010101;
pub const BGTZ:    u32 = 0b000111;
pub const BGTZL:   u32 = 0b010111;
pub const BLEZ:    u32 = 0b000110;
pub const BLEZL:   u32 = 0b010110;
pub const DADDI:   u32 = 0b011000;
pub const DADDIU:  u32 = 0b011001;
pub const LB:      u32 = 0b100000;
pub const LBU:     u32 = 0b100100;
pub const LH:      u32 = 0b100001;
pub const LHU:     u32 = 0b100101;
pub const LD:      u32 = 0b110111;
pub const LDL:     u32 = 0b011010;
pub const LDR:     u32 = 0b011011;
pub const LWL:     u32 = 0b100010;
pub const LWR:     u32 = 0b100110;
pub const SB:      u32 = 0b101000;
pub const SH:      u32 = 0b101001;
pub const SD:      u32 = 0b111111;
pub const SDL:     u32 = 0b101100;
pub const SDR:     u32 = 0b101101;
pub const SWL:     u32 = 0b101010;
pub const SWR:     u32 = 0b101110;
pub const LL:      u32 = 0b110000;
pub const LLD:     u32 = 0b110100;
pub const SC:      u32 = 0b111000;
pub const SCD:     u32 = 0b111100;
pub const CACHE:   u32 = 0b101111;
pub const SPECIAL: u32 = 0b000000;
pub const REGIMM:  u32 = 0b000001;
pub const COP0:    u32 = 0b010000;
pub const COP1:    u32 = 0b010001;
pub const LDC1:    u32 = 0b110101;
pub const LWC1:    u32 = 0b110001;
pub const SDC1:    u32 = 0b111101;
pub const SWC1:    u32 = 0b111001;

// SPECIAL ops
pub const JR:      u32 = 0b001000;
pub const JALR:    u32 = 0b001001;
pub const ADD:     u32 = 0b100000;
pub const ADDU:    u32 = 0b100001;
pub const SUB:     u32 = 0b100010;
pub const SUBU:    u32 = 0b100011;
pub const AND:     u32 = 0b100100;
pub const OR:      u32 = 0b100101;
pub const XOR:     u32 = 0b100110;
pub const NOR:     u32 = 0b100111;
pub const SRLV:    u32 = 0b000110;
pub const SRAV:    u32 = 0b000111;
pub const SLLV:    u32 = 0b000100;
pub const SLT:     u32 = 0b101010;
pub const SLTU:    u32 = 0b101011;
pub const SRL:     u32 = 0b000010;
pub const SRA:     u32 = 0b000011;
pub const SLL:     u32 = 0b000000;
pub const DADD:    u32 = 0b101100;
pub const DADDU:   u32 = 0b101101;
pub const DSLL:    u32 = 0b111000;
pub const DSLLV:   u32 = 0b010100;
pub const DSLL32:  u32 = 0b111100;
pub const DSRA:    u32 = 0b111011;
pub const DSRAV:   u32 = 0b010111;
pub const DSRA32:  u32 = 0b111111;
pub const DSRL:    u32 = 0b111010;
pub const DSRLV:   u32 = 0b010110;
pub const DSRL32:  u32 = 0b111110;
pub const DSUB:    u32 = 0b101110;
pub const DSUBU:   u32 = 0b101111;
pub const MFHI:    u32 = 0b010000;
pub const MFLO:    u32 = 0b010010;
pub const MTHI:    u32 = 0b010001;
pub const MTLO:    u32 = 0b010011;
pub const MULT:    u32 = 0b011000;
pub const MULTU:   u32 = 0b011001;
pub const DIV:     u32 = 0b011010;
pub const DIVU:    u32 = 0b011011;
pub const DMULT:   u32 = 0b011100;
pub const DMULTU:  u32 = 0b011101;
pub const DDIV:    u32 = 0b011110;
pub const DDIVU:   u32 = 0b011111;
pub const TEQ:     u32 = 0b110100;
pub const TGE:     u32 = 0b110000;
pub const TGEU:    u32 = 0b110001;
pub const TLT:     u32 = 0b110010;
pub const TLTU:    u32 = 0b110011;
pub const TNE:     u32 = 0b110110;
pub const SYNC:    u32 = 0b001111;
pub const SYSCALL: u32 = 0b001100;
pub const BREAK:   u32 = 0b001101;

// REG-IMM ops
pub const BGEZ:    u32 = 0b00001;
pub const BGEZAL:  u32 = 0b10001;
pub const BGEZALL: u32 = 0b10011;
pub const BGEZL:   u32 = 0b00011;
pub const BLTZ:    u32 = 0b00000;
pub const BLTZAL:  u32 = 0b10000;
pub const BLTZALL: u32 = 0b10010;
pub const BLTZL:   u32 = 0b00010;
pub const TEQI:    u32 = 0b01100;
pub const TGEI:    u32 = 0b01000;
pub const TGEIU:   u32 = 0b01001;
pub const TLTI:    u32 = 0b01010;
pub const TLTIU:   u32 = 0b01011;
pub const TNEI:    u32 = 0b01110;

// Coprocessor ops common to all coprocessors
pub const MF:      u32 = 0b00000;
pub const DMF:     u32 = 0b00001;
pub const CF:      u32 = 0b00010;
pub const MT:      u32 = 0b00100;
pub const DMT:     u32 = 0b00101;
pub const CT:      u32 = 0b00110;
pub const BC:      u32 = 0b01000;
pub const CO:      u32 = 0b10000;

// BC sub-ops
pub const BCF:     u32 = 0b00000;
pub const BCFL:    u32 = 0b00010;
pub const BCT:     u32 = 0b00001;
pub const BCTL:    u32 = 0b00011;

// COP0 specific
pub const ERET:    u32 = 0b011000;
pub const TLBP:    u32 = 0b001000;
pub const TLBR:    u32 = 0b000001;
pub const TLBWI:   u32 = 0b000010;
pub const TLBWR:   u32 = 0b000110;

// COP1 FP functions
pub const FABS:    u32 = 0b000101;
pub const FADD:    u32 = 0b000000;
pub const FCEILL:  u32 = 0b001010;
pub const FCEILW:  u32 = 0b001110;
pub const FCVTD:   u32 = 0b100001;
pub const FCVTL:   u32 = 0b100101;
pub const FCVTS:   u32 = 0b100000;
pub const FCVTW:   u32 = 0b100100;
pub const FDIV:    u32 = 0b000011;
pub const FFLOORL: u32 = 0b001011;
pub const FFLOORW: u32 = 0b001111;
pub const FMOV:    u32 = 0b000110;
pub const FMUL:    u32 = 0b000010;
pub const FNEG:    u32 = 0b000111;
pub const FROUNDL: u32 = 0b001000;
pub const FROUNDW: u32 = 0b001100;
pub const FSQRT:   u32 = 0b000100;
pub const FSUB:    u32 = 0b000001;
pub const FTRUNCL: u32 = 0b001001;
pub const FTRUNCW: u32 = 0b001101;
pub const FCF:     u32 = 0b110000;
pub const FCUN:    u32 = 0b110001;
pub const FCEQ:    u32 = 0b110010;
pub const FCUEQ:   u32 = 0b110011;
pub const FCOLT:   u32 = 0b110100;
pub const FCULT:   u32 = 0b110101;
pub const FCOLE:   u32 = 0b110110;
pub const FCULE:   u32 = 0b110111;
pub const FCSF:    u32 = 0b111000;
pub const FCNGLE:  u32 = 0b111001;
pub const FCSEQ:   u32 = 0b111010;
pub const FCNGL:   u32 = 0b111011;
pub const FCLT:    u32 = 0b111100;
pub const FCNGE:   u32 = 0b111101;
pub const FCLE:    u32 = 0b111110;
pub const FCNGT:   u32 = 0b111111;

// FP formats
pub const FMT_S:   u32 = 0b010000;
pub const FMT_D:   u32 = 0b010001;
pub const FMT_W:   u32 = 0b010100;
pub const FMT_L:   u32 = 0b010101;

// Register names for disassembling
pub const REG_NAMES: [&'static str; 32] = [
    "zz", "at", "v0", "v1", "a0", "a1", "a2", "a3",
    "t0", "t1", "t2", "t3", "t4", "t5", "t6", "t7",
    "s0", "s1", "s2", "s3", "s4", "s5", "s6", "s7",
    "t8", "t9", "k0", "k1", "gp", "sp", "fp", "ra"];

pub const FP_REG_NAMES: [&'static str; 32] = [
    "$f0",  "$f1",  "$f2",  "$f3",  "$f4",  "$f5",  "$f6",  "$f7",
    "$f8",  "$f9",  "$f10", "$f11", "$f12", "$f13", "$f14", "$f15",
    "$f16", "$f17", "$f18", "$f19", "$f20", "$f21", "$f22", "$f23",
    "$f24", "$f25", "$f26", "$f27", "$f28", "$f29", "$f30", "$f31"];

pub const FP_FORMATS: [&'static str; 32] = [
    "?", "?", "?", "?", "?", "?", "?", "?",
    "?", "?", "?", "?", "?", "?", "?", "?",
    "s", "d", "?", "?", "w", "l", "?", "?",
    "?", "?", "?", "?", "?", "?", "?", "?"];


#[derive(Clone, Copy)]
pub struct Instruction(pub u32);

// Note: all the accessors are declared inline(always) so that they are also
// inline in debug builds.

impl Instruction {
    /// Opcode field: bits 26-31 -- this is present in ALL instrs.
    #[inline(always)]
    pub fn opcode(self) -> u32 {
        self.0 >> 26
    }

    // Accessors for the bits 21-25

    /// Coprocessor sub-operation -- present in COPx instrs.
    #[inline(always)]
    pub fn cop_op(self) -> u32 {
        (self.0 as u32 >> 21) & 0b11111
    }

    /// Source register -- present in arithmetic and branch instrs.
    #[inline(always)]
    pub fn rs(self) -> usize {
        (self.0 as usize >> 21) & 0b11111
    }

    /// Base address register -- present in memory instrs.
    #[inline(always)]
    pub fn base(self) -> usize {
        (self.0 as usize >> 21) & 0b11111
    }

    /// FP format -- present in most FP instrs.
    #[inline(always)]
    pub fn fp_fmt(self) -> u32 {
        (self.0 as u32 >> 21) & 0b11111
    }

    // Accessors for the bits 16-20

    /// REGIMM operation -- present in REGIMM branch instrs.
    #[inline(always)]
    pub fn regimm_op(self) -> u32 {
        (self.0 >> 16) & 0b11111
    }

    /// Source2 or target register -- present in arithmetic and some branch instrs.
    #[inline(always)]
    pub fn rt(self) -> usize {
        (self.0 as usize >> 16) & 0b11111
    }

    /// FP source2 or target -- present in FP instrs.
    #[inline(always)]
    pub fn ft(self) -> usize {
        (self.0 as usize >> 16) & 0b11111
    }

    // Accessors for the bits 11-15

    /// Destination register -- present in non-immediate arithmetic instrs.
    #[inline(always)]
    pub fn rd(self) -> usize {
        (self.0 as usize >> 11) & 0b11111
    }

    /// Coprocessor register -- present in cop move instrs.
    #[inline(always)]
    pub fn cop_reg(self) -> usize {
        (self.0 as usize >> 11) & 0b11111
    }

    /// FP source -- present in FP instrs.
    #[inline(always)]
    pub fn fs(self) -> usize {
        (self.0 as usize >> 11) & 0b11111
    }

    // Accessors for the bits 6-10

    /// Shift amount -- present in immediate shift instrs.
    #[inline(always)]
    pub fn sa(self) -> u64 {
        (self.0 as u64 >> 6) & 0b11111
    }

    /// FP destination -- present in FP instrs.
    #[inline(always)]
    pub fn fd(self) -> usize {
        (self.0 as usize >> 6) & 0b11111
    }

    // Accessors for the bits 0-5

    /// SPECIAL operation -- present for opcode = SPECIAL.
    #[inline(always)]
    pub fn special_op(self) -> u32 {
        self.0 & 0b111111
    }

    /// FP operation -- present in FP instrs.
    #[inline(always)]
    pub fn fp_op(self) -> u32 {
        self.0 & 0b111111
    }

    // Accessors for the bits 0-15

    /// Unsigned, zero-extended immediate.
    #[inline(always)]
    pub fn imm(self) -> u64 {
        self.0 as u64 & 0xffff
    }

    /// Unsigned, sign-extended immediate.
    #[inline(always)]
    pub fn imm_sign_extended(self) -> u64 {
        (self.0 & 0xffff) as i16 as u64
    }

    /// Signed, sign-extended immediate.
    #[inline(always)]
    pub fn imm_signed(self) -> i64 {
        (self.0 & 0xffff) as i16 as i64
    }

    // Accessors for the bits 0-23

    /// Jump target -- present in full-length jump instruction.
    #[inline(always)]
    pub fn j_target(self) -> u64 {
        (self.0 & 0x3ff_ffff) as u64
    }
}

impl fmt::Debug for Instruction {
    /// Debug-output an instruction by disassembling it.
    ///
    /// The output is made to resemble what `objdump -m mips` prints, with the
    /// following caveats:
    ///
    /// * Branch targets are printed relative, since this routine doesn't
    ///   know the instruction's actual address.
    /// * We don't do checks for unused bits being zero, so some invalid
    ///   instructions will not be flagged as invalid.
    /// * Probably not all short forms are implemented.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        /// Expand an argument shorthand into its output form.
        macro_rules! aexp {
            (rt)    => { REG_NAMES[self.rt()] };
            (rs)    => { REG_NAMES[self.rs()] };
            (rd)    => { REG_NAMES[self.rd()] };
            (base)  => { REG_NAMES[self.base()] };
            (imm)   => { format!("{:#x}", self.imm()) };
            (ims)   => { self.imm_signed() };
            (ioff)  => { (self.imm_signed() + 1) << 2 };
            (sa)    => { self.sa() };
            (jadr)  => { format!("{:#x}", self.j_target() << 2) };
            (cop)   => { format!("{:#x}", self.rt()) };
            (cpreg) => { format!("${}", self.cop_reg()) };
            (ft)    => { FP_REG_NAMES[self.ft()] };
            (fd)    => { FP_REG_NAMES[self.fd()] };
            (fs)    => { FP_REG_NAMES[self.fs()] };
        }
        /// N-argument instruction with special formats.
        macro_rules! ins1 {
            ($name:expr, $a1:tt) => { write!(f, "{:7} {}", $name, aexp!($a1)) }
        }
        macro_rules! ins2 {
            ($name:expr, $a1:tt, $a2:tt) => {
                write!(f, "{:7} {}, {}", $name, aexp!($a1), aexp!($a2)) }
        }
        macro_rules! ins3 {
            ($name:expr, $a1:tt, $a2:tt, $a3:tt) => {
                write!(f, "{:7} {}, {}, {}", $name, aexp!($a1), aexp!($a2), aexp!($a3)) }
        }
        macro_rules! insm {
            ($name:expr, $a1:tt, $a2:tt, $a3:tt) => {
                write!(f, "{:7} {}, {}({})", $name, aexp!($a1), aexp!($a2), aexp!($a3)) }
        }
        /// FP instructions.
        macro_rules! fpins2 {
            ($name:expr, $a1:tt, $a2:tt) => {
                write!(f, "{:7} {}, {}", format!("{}.{}", $name,
                                                 FP_FORMATS[self.fp_fmt() as usize]),
                       aexp!($a1), aexp!($a2)) }
        }
        macro_rules! fpins3 {
            ($name:expr, $a1:tt, $a2:tt, $a3:tt) => {
                write!(f, "{:7} {}, {}, {}", format!("{}.{}", $name,
                                                     FP_FORMATS[self.fp_fmt() as usize]),
                       aexp!($a1), aexp!($a2), aexp!($a3)) }
        }
        /// For unknown opcodes, write the full word in hex.
        macro_rules! unknown {
            () => { write!(f, "{:7} {:#010x}", "????", self.0) }
        }

        match self.opcode() {
            LUI     => ins2!("lui",    rt, imm),
            LW      => insm!("lw",     rt, ims, base),
            LWU     => insm!("lwu",    rt, ims, base),
            SW      => insm!("sw",     rt, ims, base),
            ADDI    => ins3!("addi",   rt, rs, ims),
            ADDIU   => if self.rs() == 0 { ins2!("li", rt, ims) } else { ins3!("addiu", rt, rs, ims) },
            ANDI    => ins3!("andi",   rt, rs, imm),
            ORI     => ins3!("ori",    rt, rs, imm),
            XORI    => ins3!("xori",   rt, rs, imm),
            SLTI    => ins3!("slti",   rt, rs, imm),
            SLTIU   => ins3!("sltiu",  rt, rs, imm),
            J       => ins1!("j",      jadr),
            JAL     => ins1!("jal",    jadr),
            BEQ     => if self.rt() == 0 { ins2!("beqz",  rs, ioff) } else { ins3!("beq",  rs, rt, ioff) },
            BEQL    => if self.rt() == 0 { ins2!("beqzl", rs, ioff) } else { ins3!("beql", rs, rt, ioff) },
            BNE     => if self.rt() == 0 { ins2!("bnez",  rs, ioff) } else { ins3!("bne",  rs, rt, ioff) },
            BNEL    => if self.rt() == 0 { ins2!("bnezl", rs, ioff) } else { ins3!("bnel", rs, rt, ioff) },
            BGTZ    => ins2!("bgtz",   rs, ioff),
            BGTZL   => ins2!("bgtzl",  rs, ioff),
            BLEZ    => ins2!("blez",   rs, ioff),
            BLEZL   => ins2!("blezl",  rs, ioff),
            DADDI   => ins3!("daddi",  rt, rs, imm),
            DADDIU  => ins3!("daddiu", rt, rs, imm),
            LB      => insm!("lb",     rt, ims, base),
            LBU     => insm!("lbu",    rt, ims, base),
            LH      => insm!("lh",     rt, ims, base),
            LHU     => insm!("lhu",    rt, ims, base),
            LD      => insm!("ld",     rt, ims, base),
            LDL     => insm!("ldl",    rt, ims, base),
            LDR     => insm!("ldr",    rt, ims, base),
            LWL     => insm!("lwl",    rt, ims, base),
            LWR     => insm!("lwr",    rt, ims, base),
            SB      => insm!("sb",     rt, ims, base),
            SH      => insm!("sh",     rt, ims, base),
            SD      => insm!("sd",     rt, ims, base),
            SDL     => insm!("sdl",    rt, ims, base),
            SDR     => insm!("sdr",    rt, ims, base),
            SWL     => insm!("swl",    rt, ims, base),
            SWR     => insm!("swr",    rt, ims, base),
            LL      => insm!("ll",     rt, ims, base),
            LLD     => insm!("lld",    rt, ims, base),
            SC      => insm!("sc",     rt, ims, base),
            SCD     => insm!("scd",    rt, ims, base),
            CACHE   => insm!("cache",  cop, ims, base),
            SPECIAL => match self.special_op() {
                JR      => ins1!("jr", rs),
                JALR    => if self.rd() == 31 { ins1!("jalr", rs) } else { ins2!("jalr", rd, rs) },
                ADD     => ins3!("add",    rd, rs, rt),
                ADDU    => ins3!("addu",   rd, rs, rt),
                SUB     => ins3!("sub",    rd, rs, rt),
                SUBU    => ins3!("subu",   rd, rs, rt),
                AND     => ins3!("and",    rd, rs, rt),
                OR      => if self.rt() == 0 { ins2!("move", rd, rs) } else { ins3!("or", rd, rs, rt) },
                XOR     => ins3!("xor",    rd, rs, rt),
                NOR     => ins3!("nor",    rd, rs, rt),
                SLT     => ins3!("slt",    rd, rs, rt),
                SLTU    => ins3!("sltu",   rd, rs, rt),
                SLLV    => ins3!("sllv",   rd, rt, rs),
                SRAV    => ins3!("srav",   rd, rt, rs),
                SRLV    => ins3!("srlv",   rd, rt, rs),
                SLL     => if self.sa() == 0 { write!(f, "nop") } else { ins3!("sll", rd, rt, sa) },
                SRA     => ins3!("sra",    rd, rt, sa),
                SRL     => ins3!("srl",    rd, rt, sa),
                DADD    => ins3!("dadd",   rd, rs, rt),
                DADDU   => ins3!("daddu",  rd, rs, rt),
                DSUB    => ins3!("sub",    rd, rs, rt),
                DSUBU   => ins3!("subu",   rd, rs, rt),
                DSLLV   => ins3!("dsllv",  rd, rt, rs),
                DSRAV   => ins3!("dsrav",  rd, rt, rs),
                DSRLV   => ins3!("dsrlv",  rd, rt, rs),
                DSLL    => ins3!("dsll",   rd, rt, sa),
                DSLL32  => ins3!("dsll32", rd, rt, sa),
                DSRA    => ins3!("dsra",   rd, rt, sa),
                DSRA32  => ins3!("dsra32", rd, rt, sa),
                DSRL    => ins3!("dsrl",   rd, rt, sa),
                DSRL32  => ins3!("dsrl32", rd, rt, sa),
                MFHI    => ins1!("mfhi",   rd),
                MFLO    => ins1!("mflo",   rd),
                MTHI    => ins1!("mthi",   rs),
                MTLO    => ins1!("mtlo",   rs),
                MULT    => ins2!("mult",   rs, rt),
                MULTU   => ins2!("multu",  rs, rt),
                DIV     => ins2!("div",    rs, rt),
                DIVU    => ins2!("divu",   rs, rt),
                DMULT   => ins2!("dmult",  rs, rt),
                DMULTU  => ins2!("dmultu", rs, rt),
                DDIV    => ins2!("ddiv",   rs, rt),
                DDIVU   => ins2!("ddivu",  rs, rt),
                TEQ     => ins2!("teq",    rs, rt),
                TGE     => ins2!("tge",    rs, rt),
                TGEU    => ins2!("tgeu",   rs, rt),
                TLT     => ins2!("tlt",    rs, rt),
                TLTU    => ins2!("tltu",   rs, rt),
                TNE     => ins2!("tne",    rs, rt),
                SYNC    => write!(f, "sync"),
                SYSCALL => write!(f, "syscall {:#x}", self.0 >> 6),
                BREAK   => write!(f, "break   {:#x}", self.0 >> 6),
                _       => unknown!(),
            },
            REGIMM  => match self.regimm_op() {
                BGEZ    => if self.rs() == 0 { ins1!("b",    ioff) } else { ins2!("bgez",    rs, ioff) },
                BGEZAL  => if self.rs() == 0 { ins1!("bal",  ioff) } else { ins2!("bgezal",  rs, ioff) },
                BGEZALL => if self.rs() == 0 { ins1!("ball", ioff) } else { ins2!("bgezall", rs, ioff) },
                BGEZL   => if self.rs() == 0 { ins1!("bl",   ioff) } else { ins2!("bgezl",   rs, ioff) },
                BLTZ    => ins2!("bltz",    rs, ioff),
                BLTZAL  => ins2!("bltzal",  rs, ioff),
                BLTZALL => ins2!("bltzall", rs, ioff),
                BLTZL   => ins2!("bltzl",   rs, ioff),
                TEQI    => ins2!("teqi",    rs, imm),
                TGEI    => ins2!("tgei",    rs, imm),
                TGEIU   => ins2!("tgeiu",   rs, imm),
                TLTI    => ins2!("tlti",    rs, imm),
                TLTIU   => ins2!("tltiu",   rs, imm),
                TNEI    => ins2!("tnei",    rs, imm),
                _       => unknown!(),
            },
            COP0    => match self.cop_op() {
                MF      => ins2!("mfc0",  rt, cpreg),
                MT      => ins2!("mtc0",  rt, cpreg),
                DMF     => ins2!("dmfc0", rt, cpreg),
                DMT     => ins2!("dmtc0", rt, cpreg),
                BC      => match self.regimm_op() {
                    BCF     => ins1!("bc0f",  ioff),
                    BCFL    => ins1!("bc0fl", ioff),
                    BCT     => ins1!("bc0t",  ioff),
                    BCTL    => ins1!("bc0tl", ioff),
                    _       => unknown!(),
                },
                CO      => match self.special_op() {
                    ERET    => write!(f, "eret"),
                    TLBP    => write!(f, "tlbp"),
                    TLBR    => write!(f, "tlbr"),
                    TLBWI   => write!(f, "tlbwi"),
                    TLBWR   => write!(f, "tlbwr"),
                    _       => unknown!(),
                },
                _       => unknown!(),
            },
            COP1    => match self.cop_op() {
                MF      => ins2!("mfc1",  rt, fs),
                MT      => ins2!("mtc1",  rt, fs),
                DMF     => ins2!("dmfc1", rt, fs),
                DMT     => ins2!("dmtc1", rt, fs),
                CF      => ins2!("cfc1",  rt, cpreg),
                CT      => ins2!("ctc1",  rt, cpreg),
                BC      => match self.regimm_op() {
                    BCF     => ins1!("bc1f",  ioff),
                    BCFL    => ins1!("bc1fl", ioff),
                    BCT     => ins1!("bc1t",  ioff),
                    BCTL    => ins1!("bc1tl", ioff),
                    _       => unknown!(),
                },
                _       => match self.fp_op() {
                    FABS    => fpins2!("abs",     fd, fs),
                    FADD    => fpins3!("add",     fd, fs, ft),
                    FCEILL  => fpins2!("ceil.l",  fd, fs),
                    FCEILW  => fpins2!("ceil.w",  fd, fs),
                    FCVTD   => fpins2!("cvt.d",   fd, fs),
                    FCVTL   => fpins2!("cvt.l",   fd, fs),
                    FCVTS   => fpins2!("cvt.s",   fd, fs),
                    FCVTW   => fpins2!("cvt.w",   fd, fs),
                    FDIV    => fpins3!("div",     fd, fs, ft),
                    FFLOORL => fpins2!("floor.l", fd, fs),
                    FFLOORW => fpins2!("floor.w", fd, fs),
                    FMOV    => fpins2!("mov",     fd, fs),
                    FMUL    => fpins3!("mul",     fd, fs, ft),
                    FNEG    => fpins2!("neg",     fd, fs),
                    FROUNDL => fpins2!("round.l", fd, fs),
                    FROUNDW => fpins2!("round.w", fd, fs),
                    FSQRT   => fpins2!("sqrt",    fd, fs),
                    FSUB    => fpins3!("sub",     fd, fs, ft),
                    FTRUNCL => fpins2!("trunc.l", fd, fs),
                    FTRUNCW => fpins2!("trunc.w", fd, fs),
                    FCF     => fpins2!("c.f",     fs, ft),
                    FCUN    => fpins2!("c.un",    fs, ft),
                    FCEQ    => fpins2!("c.eq",    fs, ft),
                    FCUEQ   => fpins2!("c.ueq",   fs, ft),
                    FCOLT   => fpins2!("c.olt",   fs, ft),
                    FCULT   => fpins2!("c.ult",   fs, ft),
                    FCOLE   => fpins2!("c.ole",   fs, ft),
                    FCULE   => fpins2!("c.ule",   fs, ft),
                    FCSF    => fpins2!("c.sf",    fs, ft),
                    FCNGLE  => fpins2!("c.ngle",  fs, ft),
                    FCSEQ   => fpins2!("c.seq",   fs, ft),
                    FCNGL   => fpins2!("c.ngl",   fs, ft),
                    FCLT    => fpins2!("c.lt",    fs, ft),
                    FCNGE   => fpins2!("c.nge",   fs, ft),
                    FCLE    => fpins2!("c.cle",   fs, ft),
                    FCNGT   => fpins2!("c.ngt",   fs, ft),
                    _       => unknown!(),
                }
            },
            LDC1    => insm!("ldc1", ft, ims, base),
            LWC1    => insm!("lwc1", ft, ims, base),
            SDC1    => insm!("sdc1", ft, ims, base),
            SWC1    => insm!("swc1", ft, ims, base),
            _       => unknown!(),
        }
    }
}


#[test]
fn test_instr_decoding() {
    let word = Instruction(0x03224021);  // addu   t0, t9, v0
    assert_eq!(word.opcode(), SPECIAL);
    assert_eq!(word.special_op(), ADDU);
    assert_eq!(word.rd(), 8);   // t0
    assert_eq!(word.rs(), 25);  // t9
    assert_eq!(word.rt(), 2);   // v0
    let word = Instruction(0x8ccffffc);  // lw     t7, -4(a2)
    assert_eq!(word.opcode(), LW);
    assert_eq!(word.base(), 6); // a2
    assert_eq!(word.imm(), 0xfffc);
    assert_eq!(word.imm_sign_extended(), 0xffff_ffff_ffff_fffc);
    assert_eq!(word.imm_signed(), -0x4);
    let word = Instruction(0x00032a03);  // sra    a1, v1, 8
    assert_eq!(word.opcode(), SPECIAL);
    assert_eq!(word.special_op(), SRA);
    assert_eq!(word.sa(), 8);
    let word = Instruction(0x06310004);  // bgezal s1, 20
    assert_eq!(word.opcode(), REGIMM);
    assert_eq!(word.regimm_op(), BGEZAL);
    let word = Instruction(0x408a7000);  // mtc0   t2, $14
    assert_eq!(word.opcode(), COP0);
    assert_eq!(word.cop_op(), MT);
    assert_eq!(word.cop_reg(), 14);
    let word = Instruction(0x460a4200);  // add.s  $f8, $f8, $f10
    assert_eq!(word.opcode(), COP1);
    assert_eq!(word.fp_op(), FADD);
    assert_eq!(word.fp_fmt(), FMT_S);
    assert_eq!(word.fd(), 8);
    assert_eq!(word.fs(), 8);
    assert_eq!(word.ft(), 10);
    let word = Instruction(0x08000010);  // j      0x40
    assert_eq!(word.opcode(), J);
    assert_eq!(word.j_target(), 0x10);
}

#[test]
fn test_disassembly() {
    // Testing one of each type of instruction.
    for &(word, repr) in &[(0x3c093456, "lui     t1, 0x3456"),
                           (0x8fbf001e, "lw      ra, 30(sp)"),
                           (0x27bdffd0, "addiu   sp, sp, -48"),
                           (0x08000010, "j       0x40"),
                           (0x5512fffd, "bnel    t0, s2, -8"),
                           (0x5500fffd, "bnezl   t0, -8"),
                           (0x1de00010, "bgtz    t7, 68"),
                           (0xbd190000, "cache   0x19, 0(t0)"),
                           (0x03e00008, "jr      ra"),
                           (0x0320f809, "jalr    t9"),
                           (0x0320e809, "jalr    sp, t9"),
                           (0x01485021, "addu    t2, t2, t0"),
                           (0x00408025, "move    s0, v0"),
                           (0x00000000, "nop"),
                           (0x00032a03, "sra     a1, v1, 8"),
                           (0x0003183f, "dsra32  v1, v1, 0"),
                           (0x00001010, "mfhi    v0"),
                           (0x01cf001d, "dmultu  t6, t7"),
                           (0x0000000f, "sync"),
                           (0x0000000c, "syscall 0x0"),
                           (0x0000010d, "break   0x4"),
                           (0x07010003, "bgez    t8, 16"),
                           (0x04110003, "bal     16"),
                           (0x408a7000, "mtc0    t2, $14"),
                           (0x42000018, "eret"),
                           (0x44029000, "mfc1    v0, $f18"),
                           (0x444af800, "cfc1    t2, $31"),
                           (0x46205a45, "abs.d   $f9, $f11"),
                           (0x460a4200, "add.s   $f8, $f8, $f10"),
                           (0xd7280030, "ldc1    $f8, 48(t9)")]
    {
        assert_eq!(repr, &format!("{:?}", Instruction(word)));
    }
}
