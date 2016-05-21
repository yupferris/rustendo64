use super::super::interconnect;
use super::cp0;
use super::opcode::Opcode::*;
use super::opcode::SpecialOpcode::*;
use super::opcode::RegImmOpcode::*;
use super::instruction::Instruction;

use std::fmt;

const NUM_GPR: usize = 32;

pub struct Cpu {
    reg_gpr: [u64; NUM_GPR],
    reg_fpr: [f64; NUM_GPR],

    reg_pc: u64,

    reg_hi: u64,
    reg_lo: u64,

    reg_llbit: bool, // TODO: Enum type

    reg_fcr0: u32,
    reg_fcr31: u32,

    cp0: cp0::Cp0,

    interconnect: interconnect::Interconnect
}

impl Cpu {
    pub fn new(interconnect: interconnect::Interconnect) -> Cpu {
        Cpu {
            reg_gpr: [0; NUM_GPR],
            reg_fpr: [0.0; NUM_GPR],

            reg_pc: 0xffff_ffff_bfc0_0000, // TODO: Move to const

            reg_hi: 0,
            reg_lo: 0,

            reg_llbit: false,

            reg_fcr0: 0,
            reg_fcr31: 0,

            cp0: cp0::Cp0::default(),

            interconnect: interconnect
        }
    }

    // TODO: Different interface
    pub fn run(&mut self) {
        loop {
            self.run_instruction();
        }
    }

    pub fn run_instruction(&mut self) {
        let instr = self.read_instruction(self.reg_pc);

        self.print_instr(instr, self.reg_pc, false);

        self.reg_pc += 4;
        self.execute_instruction(instr);
    }

    fn read_instruction(&self, addr: u64) -> Instruction {
        Instruction(self.read_word(addr))
    }

    fn execute_instruction(&mut self, instr: Instruction) {
        match instr.opcode() {
            Special => {
                match instr.special_op() {
                    Sll => {
                        let value = self.read_reg_gpr(instr.rt()) << instr.sa();
                        let sign_extended_value = (value as i32) as u64;
                        self.write_reg_gpr(instr.rd() as usize, sign_extended_value);
                    }

                    Srl => {
                        let value = self.read_reg_gpr(instr.rt()) >> instr.sa();
                        let sign_extended_value = (value as i32) as u64;
                        self.write_reg_gpr(instr.rd() as usize, sign_extended_value);
                    }

                    Sllv => {
                        let shift = self.read_reg_gpr(instr.rs()) & 0b11111;
                        let value = self.read_reg_gpr(instr.rt()) << shift;
                        let sign_extended_value = (value as i32) as u64;
                        self.write_reg_gpr(instr.rd() as usize, sign_extended_value);
                    }

                    Srlv => {
                        let rs = self.read_reg_gpr(instr.rs()) as u32;
                        let rt = self.read_reg_gpr(instr.rt()) as u32;
                        let shift = rs & 0b11111;
                        let value = rt >> shift;
                        let sign_extended_value = (value as i32) as u64;
                        self.write_reg_gpr(instr.rd() as usize, sign_extended_value);
                    }

                    Jr => {
                        let delay_slot_pc = self.reg_pc;

                        // Update PC before executing delay slot instruction
                        self.reg_pc = self.read_reg_gpr(instr.rs());

                        self.execute_delay_slot(delay_slot_pc);
                    }

                    Multu => {
                        let rs = self.read_reg_gpr(instr.rs()) as u32;
                        let rt = self.read_reg_gpr(instr.rt()) as u32;

                        let res = (rs as u64) * (rt as u64);

                        // TODO: Undefined if last 2 instructions were
                        //  MFHI or MFLO
                        self.reg_lo = (res as i32) as u64;
                        self.reg_hi = ((res >> 32) as i32) as u64;
                    }

                    Mfhi => {
                        let value = self.reg_hi;
                        self.write_reg_gpr(instr.rd() as usize, value);
                    }

                    Mflo => {
                        let value = self.reg_lo;
                        self.write_reg_gpr(instr.rd() as usize, value);
                    }

                    Addu => {
                        let rs = self.read_reg_gpr(instr.rs());
                        let rt = self.read_reg_gpr(instr.rt());
                        let value = (rs.wrapping_add(rt) as i32) as u64;
                        self.write_reg_gpr(instr.rd() as usize, value);
                    }

                    Subu => {
                        let rs = self.read_reg_gpr(instr.rs());
                        let rt = self.read_reg_gpr(instr.rt());
                        let value = (rs.wrapping_sub(rt) as i32) as u64;
                        self.write_reg_gpr(instr.rd() as usize, value);
                    }

                    And => {
                        let rs = self.read_reg_gpr(instr.rs());
                        let rt = self.read_reg_gpr(instr.rt());
                        let value = rs & rt;
                        self.write_reg_gpr(instr.rd() as usize, value);
                    }

                    Or => {
                        let value = self.read_reg_gpr(instr.rs()) | self.read_reg_gpr(instr.rt());
                        self.write_reg_gpr(instr.rd() as usize, value);
                    }

                    Xor => {
                        let value = self.read_reg_gpr(instr.rs()) ^ self.read_reg_gpr(instr.rt());
                        self.write_reg_gpr(instr.rd() as usize, value);
                    }

                    Sltu => {
                        let rs = self.read_reg_gpr(instr.rs());
                        let rt = self.read_reg_gpr(instr.rt());
                        let value = if rs < rt { 1 } else { 0 };
                        self.write_reg_gpr(instr.rd() as usize, value);
                    }
                }
            }

            RegImm => {
                match instr.reg_imm_op() {
                    Bgezal => { self.branch(instr, true, |rs, _| (rs as i64) >= 0); }
                }
            }

            Addi => {
                // TODO: Handle exception overflow
                let res =
                    self.read_reg_gpr(instr.rs()) + instr.imm_sign_extended();
                self.write_reg_gpr(instr.rt(), res);
            }
            Addiu => {
                let res = self.read_reg_gpr(instr.rs())
                    .wrapping_add(instr.imm_sign_extended());
                self.write_reg_gpr(instr.rt(), res);
            }
            Andi => {
                let res = self.read_reg_gpr(instr.rs()) &
                    (instr.imm() as u64);
                self.write_reg_gpr(instr.rt(), res);
            }
            Ori => {
                let res = self.read_reg_gpr(instr.rs()) |
                    (instr.imm() as u64);
                self.write_reg_gpr(instr.rt(), res);
            }
            Lui => {
                let value = ((instr.imm() << 16) as i32) as u64;
                self.write_reg_gpr(instr.rt(), value);
            }
            Mtc0 => {
                let data = self.read_reg_gpr(instr.rt());
                self.cp0.write_reg(instr.rd(), data);
            }

            Beq => { self.branch(instr, false, |rs, rt| rs == rt); },
            Bne => { self.branch(instr, false, |rs, rt| rs != rt); },

            Beql => self.branch_likely(instr, |rs, rt| rs == rt),
            Bnel => self.branch_likely(instr, |rs, rt| rs != rt),

            Lw => {
                let base = instr.rs();

                let sign_extended_offset = instr.offset_sign_extended();
                let virt_addr =
                    self.read_reg_gpr(base).wrapping_add(sign_extended_offset);
                let mem = (self.read_word(virt_addr) as i32) as u64;
                self.write_reg_gpr(instr.rt(), mem);
            },

            Sw => {
                let base = instr.rs();

                let sign_extended_offset = instr.offset_sign_extended();
                let virt_addr =
                    self.read_reg_gpr(base).wrapping_add(sign_extended_offset);
                let mem = self.read_reg_gpr(instr.rt()) as u32;
                self.write_word(virt_addr, mem);
            }
        }
    }

    fn print_instr(&self, instr: Instruction, pc: u64, delay: bool) {
        print!("reg_pc {:018X}: ", pc);
        match instr.opcode() {
            Special => print!("Special: {:?}", instr.special_op()),
            RegImm => print!("RegImm: {:?}", instr.reg_imm_op()),
            _ => print!("{:?}", instr)
        }
        if delay {
            println!(" (DELAY)");
        } else {
            println!("");
        }
    }

    fn execute_delay_slot(&mut self, delay_slot_pc: u64) {
        let delay_slot_instr = self.read_instruction(delay_slot_pc);
        self.print_instr(delay_slot_instr, delay_slot_pc, true);
        self.execute_instruction(delay_slot_instr);
    }

    fn branch<F>(&mut self, instr: Instruction, write_link: bool, f: F) -> bool where F: FnOnce(u64, u64) -> bool {
        let rs = self.read_reg_gpr(instr.rs());
        let rt = self.read_reg_gpr(instr.rt());
        let is_taken = f(rs, rt);

        let delay_slot_pc = self.reg_pc;

        if write_link {
            let link_address = delay_slot_pc + 4;
            self.write_reg_gpr(31, link_address);
        }

        if is_taken {
            let sign_extended_offset =
                instr.offset_sign_extended() << 2;
            // Update PC before executing delay slot instruction
            self.reg_pc =
                self.reg_pc.wrapping_add(sign_extended_offset);

            self.execute_delay_slot(delay_slot_pc);
        }

        is_taken
    }

    fn branch_likely<F>(&mut self, instr: Instruction, f: F) where F: FnOnce(u64, u64) -> bool {
        if !self.branch(instr, false, f) {
            // Skip over delay slot instruction when not branching
            self.reg_pc = self.reg_pc.wrapping_add(4);
        }
    }

    fn read_word(&self, virt_addr: u64) -> u32 {
        let phys_addr = self.virt_addr_to_phys_addr(virt_addr);
        self.interconnect.read_word(phys_addr as u32)
    }

    fn write_word(&mut self, virt_addr: u64, value: u32) {
        let phys_addr = self.virt_addr_to_phys_addr(virt_addr);
        self.interconnect.write_word(phys_addr as u32, value);
    }

    fn virt_addr_to_phys_addr(&self, virt_addr: u64) -> u64 {
        // See Table 5-3 in the VR4300 User's Manual
        let addr_bit_values = (virt_addr >> 29) & 0b111;

        if addr_bit_values == 0b101 {
            // kseg1
            virt_addr - 0xffff_ffff_a000_0000
        } else {
            // TODO
            panic!("Unrecognized virtual address: {:#x}", virt_addr);
        }
    }

    fn write_reg_gpr(&mut self, index: usize, value: u64) {
        if index != 0 {
            self.reg_gpr[index] = value;
        }
    }

    fn read_reg_gpr(&self, index: usize) -> u64 {
        match index {
            0 => 0,
            _ => self.reg_gpr[index]
        }
    }
}

impl fmt::Debug for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        const REGS_PER_LINE: usize = 2;
        const REG_NAMES: [&'static str; NUM_GPR] = [
        "r0", "at", "v0", "v1", "a0", "a1", "a2", "a3",
        "t0", "t1", "t2", "t3", "t4", "t5", "t6", "t7",
        "s0", "s1", "s2", "s3", "s4", "s5", "s6", "s7",
        "t8", "t9", "k0", "k1", "gp", "sp", "s8", "ra",
        ];

        try!(write!(f,"\nCPU General Purpose Registers:"));
        for reg_num in 0..NUM_GPR {
            if (reg_num % REGS_PER_LINE) == 0 {
                try!(writeln!(f,""));
            }
            try!(write!(f,
                "{reg_name}/gpr{num:02}: {value:#018X} ",
                num = reg_num,
                reg_name = REG_NAMES[reg_num],
                value = self.reg_gpr[reg_num],
            ));
        }

        try!(write!(f,"\n\nCPU Floating Point Registers:"));
        for reg_num in 0..NUM_GPR {
            if (reg_num % REGS_PER_LINE) == 0 {
                try!(writeln!(f,""));
            }
            try!(write!(f,
                "fpr{num:02}: {value:21} ",
                num = reg_num,
                value = self.reg_fpr[reg_num],)
            );
        }

        try!(writeln!(f,"\n\nCPU Special Registers:"));
        try!(writeln!(f,
            "\
            reg_pc: {:#018X}\n\
            reg_hi: {:#018X}\n\
            reg_lo: {:#018X}\n\
            reg_llbit: {}\n\
            reg_fcr0:  {:#010X}\n\
            reg_fcr31: {:#010X}\n\
            ",
            self.reg_pc,
            self.reg_hi,
            self.reg_lo,
            self.reg_llbit,
            self.reg_fcr0,
            self.reg_fcr31
        ));

        try!(writeln!(f, "{:#?}", self.cp0));
        writeln!(f, "{:#?}", self.interconnect)
    }
}
