use super::super::interconnect;
use super::cp0;
use super::instruction as ins;

const NUM_GPR: usize = 32;

#[derive(Debug)]
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

            reg_pc: 0,

            reg_hi: 0,
            reg_lo: 0,

            reg_llbit: false,

            reg_fcr0: 0,
            reg_fcr31: 0,

            cp0: cp0::Cp0::default(),

            interconnect: interconnect
        }
    }

    pub fn power_on_reset(&mut self) {
        self.cp0.power_on_reset();

        self.reg_pc = 0xffff_ffff_bfc0_0000; // TODO: Move to const
    }

    // TODO: Different interface
    pub fn run(&mut self) {
        loop {
            self.run_instruction();
        }
    }

    pub fn run_instruction(&mut self) {
        let instr = ins::Instruction(self.read_word(self.reg_pc));

        match instr.opcode() {
            ins::ORI => {
                let res = self.read_reg_gpr(instr.rs()) | instr.imm();
                self.write_reg_gpr(instr.rt(), res);
            },
            ins::LUI => {
                let value = instr.imm_sign_extended() << 16;
                self.write_reg_gpr(instr.rt(), value);
            },
            ins::COP0 => match instr.cop_op() {
                ins::MT => {
                    let data = self.read_reg_gpr(instr.rt());
                    self.cp0.write_reg(instr.rd(), data);
                },
                _ => panic!("Unrecognized COP0 instruction: {:?}", instr)
            },
            ins::LW => {
                let virt_addr = self.read_reg_gpr(instr.base())
                                    .wrapping_add(instr.imm_sign_extended());
                let mem = (self.read_word(virt_addr) as i32) as u64;
                self.write_reg_gpr(instr.rt(), mem);
            }
            _ => panic!("Unrecognized instruction: {:?}", instr)
        }

        self.reg_pc += 4;
    }

    fn read_word(&self, virt_addr: u64) -> u32 {
        let phys_addr = self.virt_addr_to_phys_addr(virt_addr);
        // TODO: Check endianness
        self.interconnect.read_word(phys_addr as u32)
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
