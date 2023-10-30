use super::memory::ROM_LOW_ADDRESS;

#[derive(Debug)]
pub struct Cpu {
    pub pc: u16,
    pub gr: [u16; 8],
    pub sp: u16,
    pub psr: u16,
    pub tr: u32,
    pub tlr: u16,
    pub thr: u16,
    pub ppc: u16,
    pub ppsr: u16,
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            pc: ROM_LOW_ADDRESS,
            gr: [0; 8],
            sp: 0,
            psr: 0,
            tr: 0,
            tlr: 0,
            thr: 0,
            ppc: 0,
            ppsr: 0,
        }
    }

    // There is no need to check if num is less than 8 because invalid registers cannot be specified by the instruction.
    pub fn get_gr(&self, num: u8) -> u16 {
        if num == 0 {
            0
        } else {
            self.gr[num as usize]
        }
    }

    pub fn set_gr(&mut self, num: u8, data: u16) {
        if num != 0 {
            self.gr[num as usize] = data;
        }
    }

    // R instruction
    pub fn mov(&mut self, rd: u8, rs: u8) {
        let data = self.get_gr(rs);
        self.set_gr(rd, data);
    }

    pub fn add(&mut self, rd: u8, rs: u8) {
        let data = self.get_gr(rd).wrapping_add(self.get_gr(rs));
        self.set_gr(rd, data);
    }

    pub fn sub(&mut self, rd: u8, rs: u8) {
        let data = self.get_gr(rd).wrapping_sub(self.get_gr(rs));
        self.set_gr(rd, data);
    }

    pub fn and(&mut self, rd: u8, rs: u8) {
        let data = self.get_gr(rd) & self.get_gr(rs);
        self.set_gr(rd, data);
    }

    pub fn or(&mut self, rd: u8, rs: u8) {
        let data = self.get_gr(rd) | self.get_gr(rs);
        self.set_gr(rd, data);
    }

    pub fn xor(&mut self, rd: u8, rs: u8) {
        let data = self.get_gr(rd) ^ self.get_gr(rs);
        self.set_gr(rd, data);
    }

    pub fn sll(&mut self, rd: u8, rs: u8) {
        let data = self.get_gr(rd) << self.get_gr(rs);
        self.set_gr(rd, data)
    }
    pub fn srl(&mut self, rd: u8, rs: u8) {
        let data = self.get_gr(rd) >> self.get_gr(rs);
        self.set_gr(rd, data);
    }
    pub fn sra(&mut self, rd: u8, rs: u8) {
        let data = (self.get_gr(rd) as i16) >> self.get_gr(rs);
        self.set_gr(rd, data as u16);
    }

    // I5 instruction
    pub fn addi(&mut self, rd: u8, rs: u8, imm: u16) {
        let data = self.get_gr(rs).wrapping_add(imm);
        self.set_gr(rd, data);
    }

    pub fn subi(&mut self, rd: u8, rs: u8, imm: u16) {
        let data = self.get_gr(rs).wrapping_sub(imm);
        self.set_gr(rd, data);
    }

    pub fn beq(&mut self, rd: u8, rs: u8, imm: i16) {
        if self.get_gr(rd) == self.get_gr(rs) {
            self.pc -= 2;
            self.pc = self.pc.wrapping_add(imm as u16);
        }
    }

    pub fn bnq(&mut self, rd: u8, rs: u8, imm: i16) {
        if self.get_gr(rd) != self.get_gr(rs) {
            self.pc -= 2;
            self.pc = self.pc.wrapping_add(imm as u16);
        }
    }

    pub fn blt(&mut self, rd: u8, rs: u8, imm: i16) {
        if (self.get_gr(rd) as i16) < (self.get_gr(rs) as i16) {
            self.pc -= 2;
            self.pc = self.pc.wrapping_add(imm as u16);
        }
    }

    pub fn bge(&mut self, rd: u8, rs: u8, imm: i16) {
        if (self.get_gr(rd) as i16) >= (self.get_gr(rs) as i16) {
            self.pc -= 2;
            self.pc = self.pc.wrapping_add(imm as u16);
        }
    }

    pub fn bltu(&mut self, rd: u8, rs: u8, imm: i16) {
        if self.get_gr(rd) < self.get_gr(rs) {
            self.pc -= 2;
            self.pc = self.pc.wrapping_add(imm as u16);
        }
    }

    pub fn bgeu(&mut self, rd: u8, rs: u8, imm: i16) {
        if self.get_gr(rd) >= self.get_gr(rs) {
            self.pc -= 2;
            self.pc = self.pc.wrapping_add(imm as u16);
        }
    }

    pub fn jalr(&mut self, rd: u8, rs: u8, imm: i16) {
        self.set_gr(rd, self.pc);
        self.pc = self.get_gr(rs).wrapping_add(imm as u16);
    }

    pub fn jal(&mut self, rd: u8, imm: i16) {
        self.set_gr(rd, self.pc);
        self.pc -= 2;
        self.pc = self.pc.wrapping_add(imm as u16);
    }

    pub fn lil(&mut self, rd: u8, imm: u16) {
        self.set_gr(rd, imm);
    }

    pub fn lih(&mut self, rd: u8, imm: u16) {
        self.set_gr(rd, imm << 8);
    }

    pub fn rpc(&mut self, rd: u8) {
        self.set_gr(rd, self.pc);
    }

    pub fn rsp(&mut self, rd: u8) {
        self.set_gr(rd, self.sp);
    }

    pub fn rpsr(&mut self, rd: u8) {
        self.set_gr(rd, self.psr);
    }

    pub fn rtlr(&mut self, rd: u8) {
        self.set_gr(rd, self.tlr);
    }

    pub fn rthr(&mut self, rd: u8) {
        self.set_gr(rd, self.thr);
    }

    pub fn rppc(&mut self, rd: u8) {
        self.set_gr(rd, self.ppc);
    }

    pub fn rppsr(&mut self, rd: u8) {
        self.set_gr(rd, self.ppsr);
    }

    pub fn wsp(&mut self, rd: u8) {
        self.sp = self.get_gr(rd);
    }

    pub fn wpsr(&mut self, rd: u8) {
        self.psr = self.get_gr(rd);
    }

    pub fn wtlr(&mut self, rd: u8) {
        self.tlr = self.get_gr(rd);
    }

    pub fn wthr(&mut self, rd: u8) {
        self.thr = self.get_gr(rd);
    }

    pub fn wppc(&mut self, rd: u8) {
        self.ppc = self.get_gr(rd);
    }

    pub fn wppsr(&mut self, rd: u8) {
        self.ppsr = self.get_gr(rd);
    }

    pub fn rfi(&mut self) {
        self.pc = self.ppc;
        self.psr = self.ppsr;
    }

    pub fn rtr(&mut self) {
        self.thr = ((self.tr & 0xFFFF0000) >> 16) as u16;
        self.tlr = (self.tr & 0x0000FFFF) as u16;
    }

    pub fn wtr(&mut self) {
        let tr = ((self.thr as u32) << 16) | self.tlr as u32;
        self.tr = tr;
    }

    pub fn trap(&mut self) {
        self.ppc = self.pc;
        self.ppsr = self.psr;
        self.psr = 0x2;
        self.pc = 0;
    }
}
