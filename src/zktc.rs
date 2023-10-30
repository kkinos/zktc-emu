mod cpu;
mod memory;
use cpu::Cpu;
use memory::Memory;

#[derive(Debug)]
pub struct Zktc {
    cpu: Cpu,
    memory: Memory,
    break_point: Option<u16>,
}

pub enum InstInfo {
    R {
        mnemonic: String,
        rd: u8,
        rs: u8,
    },
    I5 {
        mnemonic: String,
        rd: u8,
        rs: u8,
        imm: Option<u16>,
        imm_sext: Option<i16>,
    },
    I8 {
        mnemonic: String,
        rd: u8,
        rs: u8,
        imm: Option<u16>,
        imm_sext: Option<i16>,
    },
    C1 {
        mnemonic: String,
        rd: u8,
    },
    C2 {
        mnemonic: String,
    },
    Trap {
        mnemonic: String,
    },
}

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum Error {
    #[error("memory error")]
    MemoryError(#[from] memory::MemoryError),

    #[error("unknown instruction 0x{0:04x}")]
    UnknownInstruction(u16),

    #[error("debug interrupt")]
    DebugInterrupt(),
}

impl Zktc {
    pub fn new(rom_file: Vec<u8>, ram_file: Vec<u8>) -> Result<Self, Error> {
        Ok(Zktc {
            cpu: Cpu::new(),
            memory: Memory::new(rom_file, ram_file)?,
            break_point: None,
        })
    }

    pub fn do_cmd(&mut self, cmd: Vec<&str>) -> Result<(), Error> {
        match cmd[0] {
            "run" | "r" => self.run()?,
            "step" | "s" => {
                if let Err(e) = self.step() {
                    if e == Error::DebugInterrupt() {
                        println!("{}", e);
                    } else {
                        return Err(e);
                    }
                }
            }
            "break" | "b" => {
                if cmd.len() != 2 {
                    eprintln!("invalid command\ne.g. : b 0x8000");
                    return Ok(());
                }

                let addr = cmd[1];
                if !addr.starts_with("0x") {
                    eprintln!("address is only hexadecimal\ne.g. : b 0x8000");
                    return Ok(());
                }
                match u16::from_str_radix(addr.trim_start_matches("0x"), 16) {
                    Ok(addr) => self.set_break(addr),
                    Err(_) => {
                        eprint!("invalid address\ne.g. : b 0x8000");
                    }
                }
            }
            "regsters" | "regs" => self.print_regs(),

            _ => {
                eprintln!("command not found : {}", cmd[0]);
            }
        }
        Ok(())
    }

    pub fn run(&mut self) -> Result<(), Error> {
        loop {
            if let Err(e) = self.step() {
                if e == Error::DebugInterrupt() {
                    println!("{}", Error::DebugInterrupt());
                    break;
                } else {
                    return Err(e);
                }
            }
            if let Some(b) = self.break_point {
                if self.cpu.pc == b {
                    break;
                }
            }
        }
        Ok(())
    }

    pub fn step(&mut self) -> Result<(), Error> {
        let current_pc = self.cpu.pc;

        let word = self.memory.read_from_memory(&current_pc)?;
        if word == 0x0 {
            return Err(Error::DebugInterrupt());
        }
        self.cpu.pc += 2;

        let opcode = word & 0x001F;
        let rd = ((word & 0x00E0) >> 5) as u8;
        let rs = ((word & 0x0700) >> 8) as u8;
        let func = (word & 0xF800) >> 11;
        let imm_i5 = (word & 0xF800) >> 11;
        let imm_i5_sext = ((word & 0xF800) as i16) >> 11;
        let imm_i8 = (word & 0xFF00) >> 8;
        let imm_i8_sext = ((word & 0xFF00) as i16) >> 8;

        match opcode {
            0b00000 => match func {
                0b0001 => {
                    let inst_info = InstInfo::R {
                        mnemonic: "mov".to_string(),
                        rd,
                        rs,
                    };
                    Self::print_inst_info(current_pc, word, inst_info);

                    self.cpu.mov(rd, rs);
                }
                0b0010 => {
                    let inst_info = InstInfo::R {
                        mnemonic: "add".to_string(),
                        rd,
                        rs,
                    };
                    Self::print_inst_info(current_pc, word, inst_info);

                    self.cpu.add(rd, rs);
                }
                0b0011 => {
                    let inst_info = InstInfo::R {
                        mnemonic: "sub".to_string(),
                        rd,
                        rs,
                    };
                    Self::print_inst_info(current_pc, word, inst_info);

                    self.cpu.sub(rd, rs);
                }
                0b0100 => {
                    let inst_info = InstInfo::R {
                        mnemonic: "and".to_string(),
                        rd,
                        rs,
                    };
                    Self::print_inst_info(current_pc, word, inst_info);

                    self.cpu.and(rd, rs);
                }
                0b0101 => {
                    let inst_info = InstInfo::R {
                        mnemonic: "or".to_string(),
                        rd,
                        rs,
                    };
                    Self::print_inst_info(current_pc, word, inst_info);

                    self.cpu.or(rd, rs);
                }
                0b0110 => {
                    let inst_info = InstInfo::R {
                        mnemonic: "xor".to_string(),
                        rd,
                        rs,
                    };
                    Self::print_inst_info(current_pc, word, inst_info);

                    self.cpu.xor(rd, rs);
                }
                0b0111 => {
                    let inst_info = InstInfo::R {
                        mnemonic: "sll".to_string(),
                        rd,
                        rs,
                    };
                    Self::print_inst_info(current_pc, word, inst_info);

                    self.cpu.sll(rd, rs);
                }
                0b1000 => {
                    let inst_info = InstInfo::R {
                        mnemonic: "srl".to_string(),
                        rd,
                        rs,
                    };
                    Self::print_inst_info(current_pc, word, inst_info);

                    self.cpu.srl(rd, rs);
                }
                0b1001 => {
                    let inst_info = InstInfo::R {
                        mnemonic: "sra".to_string(),
                        rd,
                        rs,
                    };
                    Self::print_inst_info(current_pc, word, inst_info);

                    self.cpu.sra(rd, rs);
                }
                _ => Err(Error::UnknownInstruction(word))?,
            },
            0b00001 => {
                let inst_info = InstInfo::I5 {
                    mnemonic: "addi".to_string(),
                    rd,
                    rs,
                    imm: Some(imm_i5),
                    imm_sext: None,
                };
                Self::print_inst_info(current_pc, word, inst_info);

                self.cpu.addi(rd, rs, imm_i5);
            }
            0b00010 => {
                let inst_info = InstInfo::I5 {
                    mnemonic: "subi".to_string(),
                    rd,
                    rs,
                    imm: Some(imm_i5),
                    imm_sext: None,
                };
                Self::print_inst_info(current_pc, word, inst_info);

                self.cpu.subi(rd, rs, imm_i5);
            }
            0b00011 => {
                let inst_info = InstInfo::I5 {
                    mnemonic: "beq".to_string(),
                    rd,
                    rs,
                    imm: None,
                    imm_sext: Some(imm_i5_sext),
                };
                Self::print_inst_info(current_pc, word, inst_info);

                self.cpu.beq(rd, rs, imm_i5_sext);
            }
            0b00100 => {
                let inst_info = InstInfo::I5 {
                    mnemonic: "bnq".to_string(),
                    rd,
                    rs,
                    imm: None,
                    imm_sext: Some(imm_i5_sext),
                };
                Self::print_inst_info(current_pc, word, inst_info);

                self.cpu.bnq(rd, rs, imm_i5_sext);
            }
            0b00101 => {
                let inst_info = InstInfo::I5 {
                    mnemonic: "blt".to_string(),
                    rd,
                    rs,
                    imm: None,
                    imm_sext: Some(imm_i5_sext),
                };
                Self::print_inst_info(current_pc, word, inst_info);

                self.cpu.blt(rd, rs, imm_i5_sext);
            }
            0b00110 => {
                let inst_info = InstInfo::I5 {
                    mnemonic: "bge".to_string(),
                    rd,
                    rs,
                    imm: None,
                    imm_sext: Some(imm_i5_sext),
                };
                Self::print_inst_info(current_pc, word, inst_info);

                self.cpu.bge(rd, rs, imm_i5_sext);
            }
            0b00111 => {
                let inst_info = InstInfo::I5 {
                    mnemonic: "bltu".to_string(),
                    rd,
                    rs,
                    imm: None,
                    imm_sext: Some(imm_i5_sext),
                };
                Self::print_inst_info(current_pc, word, inst_info);

                self.cpu.bltu(rd, rs, imm_i5_sext);
            }
            0b01000 => {
                let inst_info = InstInfo::I5 {
                    mnemonic: "bgeu".to_string(),
                    rd,
                    rs,
                    imm: None,
                    imm_sext: Some(imm_i5_sext),
                };
                Self::print_inst_info(current_pc, word, inst_info);

                self.cpu.bgeu(rd, rs, imm_i5_sext);
            }
            0b01001 => {
                let inst_info = InstInfo::I5 {
                    mnemonic: "jalr".to_string(),
                    rd,
                    rs,
                    imm: None,
                    imm_sext: Some(imm_i5_sext),
                };
                Self::print_inst_info(current_pc, word, inst_info);

                self.cpu.jalr(rd, rs, imm_i5_sext);
            }
            0b01010 => {
                let inst_info = InstInfo::I5 {
                    mnemonic: "lw".to_string(),
                    rd,
                    rs,
                    imm: None,
                    imm_sext: Some(imm_i5_sext),
                };
                Self::print_inst_info(current_pc, word, inst_info);

                let address = self.cpu.get_gr(rs).wrapping_add(imm_i5_sext as u16);
                let data = self.memory.read_from_memory(&address)?;
                self.cpu.set_gr(rd, data);
            }
            0b01011 => {
                let inst_info = InstInfo::I5 {
                    mnemonic: "sw".to_string(),
                    rd,
                    rs,
                    imm: None,
                    imm_sext: Some(imm_i5_sext),
                };
                Self::print_inst_info(current_pc, word, inst_info);

                let address = self.cpu.get_gr(rs).wrapping_add(imm_i5_sext as u16);
                let data = self.cpu.get_gr(rd);
                self.memory.write_to_memory(&address, data)?;
            }
            0b01100 => {
                let inst_info = InstInfo::I8 {
                    mnemonic: "jal".to_string(),
                    rd,
                    rs,
                    imm: None,
                    imm_sext: Some(imm_i8_sext),
                };
                Self::print_inst_info(current_pc, word, inst_info);

                self.cpu.jal(rd, imm_i8_sext);
            }
            0b01101 => {
                let inst_info = InstInfo::I8 {
                    mnemonic: "lil".to_string(),
                    rd,
                    rs,
                    imm: Some(imm_i8),
                    imm_sext: None,
                };
                Self::print_inst_info(current_pc, word, inst_info);

                self.cpu.lil(rd, imm_i8);
            }
            0b01110 => {
                let inst_info = InstInfo::I8 {
                    mnemonic: "lih".to_string(),
                    rd,
                    rs,
                    imm: Some(imm_i8),
                    imm_sext: None,
                };
                Self::print_inst_info(current_pc, word, inst_info);

                self.cpu.lih(rd, imm_i8);
            }
            0b11110 => match func {
                0b00001 => {
                    let inst_info = InstInfo::C1 {
                        mnemonic: "push".to_string(),
                        rd,
                    };
                    Self::print_inst_info(current_pc, word, inst_info);

                    let data = self.cpu.get_gr(rd);
                    self.cpu.sp -= 2;
                    self.memory.write_to_memory(&self.cpu.sp, data)?;
                }
                0b00010 => {
                    let inst_info = InstInfo::C1 {
                        mnemonic: "pop".to_string(),
                        rd,
                    };
                    Self::print_inst_info(current_pc, word, inst_info);

                    let data = self.memory.read_from_memory(&self.cpu.sp)?;
                    self.cpu.set_gr(rd, data);
                    self.cpu.sp += 2;
                }
                0b00011 => {
                    let inst_info = InstInfo::C1 {
                        mnemonic: "rpc".to_string(),
                        rd,
                    };
                    Self::print_inst_info(current_pc, word, inst_info);

                    self.cpu.rpc(rd);
                }
                0b00100 => {
                    let inst_info = InstInfo::C1 {
                        mnemonic: "rsp".to_string(),
                        rd,
                    };
                    Self::print_inst_info(current_pc, word, inst_info);

                    self.cpu.rsp(rd);
                }
                0b00101 => {
                    let inst_info = InstInfo::C1 {
                        mnemonic: "rpsr".to_string(),
                        rd,
                    };
                    Self::print_inst_info(current_pc, word, inst_info);

                    self.cpu.rpsr(rd);
                }
                0b00110 => {
                    let inst_info = InstInfo::C1 {
                        mnemonic: "rtlr".to_string(),
                        rd,
                    };
                    Self::print_inst_info(current_pc, word, inst_info);

                    self.cpu.rtlr(rd);
                }
                0b00111 => {
                    let inst_info = InstInfo::C1 {
                        mnemonic: "rthr".to_string(),
                        rd,
                    };
                    Self::print_inst_info(current_pc, word, inst_info);

                    self.cpu.rthr(rd);
                }
                0b01000 => {
                    let inst_info = InstInfo::C1 {
                        mnemonic: "rppc".to_string(),
                        rd,
                    };
                    Self::print_inst_info(current_pc, word, inst_info);

                    self.cpu.rppc(rd);
                }
                0b01001 => {
                    let inst_info = InstInfo::C1 {
                        mnemonic: "rppsr".to_string(),
                        rd,
                    };
                    Self::print_inst_info(current_pc, word, inst_info);

                    self.cpu.rppsr(rd);
                }
                0b01010 => {
                    let inst_info = InstInfo::C1 {
                        mnemonic: "wsp".to_string(),
                        rd,
                    };
                    Self::print_inst_info(current_pc, word, inst_info);

                    self.cpu.wsp(rd);
                }
                0b01011 => {
                    let inst_info = InstInfo::C1 {
                        mnemonic: "wpsr".to_string(),
                        rd,
                    };
                    Self::print_inst_info(current_pc, word, inst_info);

                    self.cpu.wpsr(rd);
                }
                0b01100 => {
                    let inst_info = InstInfo::C1 {
                        mnemonic: "wtlr".to_string(),
                        rd,
                    };
                    Self::print_inst_info(current_pc, word, inst_info);

                    self.cpu.wtlr(rd);
                }
                0b01101 => {
                    let inst_info = InstInfo::C1 {
                        mnemonic: "wthr".to_string(),
                        rd,
                    };
                    Self::print_inst_info(current_pc, word, inst_info);

                    self.cpu.wthr(rd);
                }
                0b01110 => {
                    let inst_info = InstInfo::C1 {
                        mnemonic: "wppc".to_string(),
                        rd,
                    };
                    Self::print_inst_info(current_pc, word, inst_info);

                    self.cpu.wppc(rd);
                }
                0b01111 => {
                    let inst_info = InstInfo::C1 {
                        mnemonic: "wppsr".to_string(),
                        rd,
                    };
                    Self::print_inst_info(current_pc, word, inst_info);

                    self.cpu.wppsr(rd);
                }
                _ => Err(Error::UnknownInstruction(word))?,
            },
            0b11111 => match func {
                0b00001 => {
                    let inst_info = InstInfo::C2 {
                        mnemonic: "rfi".to_string(),
                    };
                    Self::print_inst_info(current_pc, word, inst_info);

                    self.cpu.rfi();
                }
                0b00010 => {
                    let inst_info = InstInfo::C2 {
                        mnemonic: "rtr".to_string(),
                    };
                    Self::print_inst_info(current_pc, word, inst_info);

                    self.cpu.rtr();
                }
                0b00011 => {
                    let inst_info = InstInfo::C2 {
                        mnemonic: "wtr".to_string(),
                    };
                    Self::print_inst_info(current_pc, word, inst_info);

                    self.cpu.wtr();
                }
                _ => {
                    if word == 0xFFFF {
                        let inst_info = InstInfo::Trap {
                            mnemonic: "trap".to_string(),
                        };
                        Self::print_inst_info(current_pc, word, inst_info);

                        self.cpu.trap();
                    } else {
                        Err(Error::UnknownInstruction(word))?
                    }
                }
            },
            _ => Err(Error::UnknownInstruction(word))?,
        };

        Ok(())
    }

    fn print_regs(&self) {
        println!(
            " x0 : 0x{:04x} x1 : 0x{:04x} x2 : 0x{:04x} x3 : 0x{:04x}",
            self.cpu.get_gr(0),
            self.cpu.get_gr(1),
            self.cpu.get_gr(2),
            self.cpu.get_gr(3),
        );
        println!(
            " x4 : 0x{:04x} x5 : 0x{:04x} x6 : 0x{:04x} x7 : 0x{:04x}",
            self.cpu.get_gr(4),
            self.cpu.get_gr(5),
            self.cpu.get_gr(6),
            self.cpu.get_gr(7),
        );
        println!(
            " pc : 0x{:04x} sp : 0x{:04x} psr : 0x{:04x} tr : 0x{:08x}",
            self.cpu.pc, self.cpu.sp, self.cpu.psr, self.cpu.tr,
        );
        println!(
            " tlr : 0x{:04x} thr : 0x{:04x} ppc : 0x{:04x} ppsr : 0x{:08x}",
            self.cpu.tlr, self.cpu.thr, self.cpu.ppc, self.cpu.ppsr,
        );
    }

    fn set_break(&mut self, address: u16) {
        self.break_point = Some(address);
    }

    fn print_inst_info(current_pc: u16, word: u16, inst_info: InstInfo) {
        match inst_info {
            InstInfo::R { mnemonic, rd, rs } => {
                println!(
                    "pc:0x{:04x} {:016b} {} x{} x{}",
                    current_pc, word, mnemonic, rd, rs
                )
            }
            InstInfo::I5 {
                mnemonic,
                rd,
                rs,
                imm,
                imm_sext,
            } => {
                if let Some(imm) = imm {
                    println!(
                        "pc:0x{:04x} {:016b} {} x{} x{} {}",
                        current_pc, word, mnemonic, rd, rs, imm
                    )
                } else {
                    println!(
                        "pc:0x{:04x} {:016b} {} x{} x{} {}",
                        current_pc,
                        word,
                        mnemonic,
                        rd,
                        rs,
                        imm_sext.unwrap()
                    )
                }
            }
            InstInfo::I8 {
                mnemonic,
                rd,
                imm,
                imm_sext,
                ..
            } => {
                if let Some(imm) = imm {
                    println!(
                        "pc:0x{:04x} {:016b} {} x{} {}",
                        current_pc, word, mnemonic, rd, imm
                    )
                } else {
                    println!(
                        "pc:0x{:04x} {:016b} {} x{} {}",
                        current_pc,
                        word,
                        mnemonic,
                        rd,
                        imm_sext.unwrap()
                    )
                }
            }
            InstInfo::C1 { mnemonic, rd } => {
                println!(
                    "pc:0x{:04x} {:016b} {} x{} ",
                    current_pc, word, mnemonic, rd
                )
            }
            InstInfo::C2 { mnemonic } => {
                println!("pc:0x{:04x} {:016b} {} ", current_pc, word, mnemonic)
            }
            InstInfo::Trap { mnemonic } => {
                println!("pc:0x{:04x} {:016b} {} ", current_pc, word, mnemonic)
            }
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn mov_test() {
        run_test("mem/mov_test.mem");
    }

    #[test]
    fn add_test() {
        run_test("mem/add_test.mem");
    }

    #[test]
    fn sub_test() {
        run_test("mem/sub_test.mem");
    }

    #[test]
    fn and_test() {
        run_test("mem/sub_test.mem");
    }

    #[test]
    fn or_test() {
        run_test("mem/or_test.mem");
    }

    #[test]
    fn xor_test() {
        run_test("mem/xor_test.mem");
    }

    #[test]
    fn sll_test() {
        run_test("mem/sll_test.mem");
    }

    #[test]
    fn srl_test() {
        run_test("mem/srl_test.mem");
    }

    #[test]
    fn sra_test() {
        run_test("mem/sra_test.mem");
    }

    #[test]
    fn addi_test() {
        run_test("mem/addi_test.mem");
    }

    #[test]
    fn subi_test() {
        run_test("mem/subi_test.mem");
    }

    #[test]
    fn beq_test() {
        run_test("mem/beq_test.mem");
    }

    #[test]
    fn bnq_test() {
        run_test("mem/bnq_test.mem");
    }

    #[test]
    fn blt_test() {
        run_test("mem/blt_test.mem");
    }

    #[test]
    fn bge_test() {
        run_test("mem/bge_test.mem");
    }

    #[test]
    fn bltu_test() {
        run_test("mem/bltu_test.mem");
    }

    #[test]
    fn bgeu_test() {
        run_test("mem/bgeu_test.mem");
    }

    #[test]
    fn jalr_test() {
        run_test("mem/jalr_test.mem");
    }

    #[test]
    fn lw_test() {
        run_test("mem/lw_test.mem");
    }

    #[test]
    fn sw_test() {
        run_test("mem/sw_test.mem");
    }

    #[test]
    fn jal_test() {
        run_test("mem/jal_test.mem");
    }

    #[test]
    fn lil_test() {
        run_test("mem/lil_test.mem");
    }

    #[test]
    fn lih_test() {
        run_test("mem/lih_test.mem");
    }

    #[test]
    fn push_test() {
        run_test("mem/push_test.mem");
    }

    #[test]
    fn pop_test() {
        run_test("mem/pop_test.mem");
    }

    #[test]
    fn rpc_test() {
        run_test("mem/rpc_test.mem");
    }

    #[test]
    fn rsp_test() {
        run_test("mem/rsp_test.mem");
    }

    #[test]
    fn rpsr_test() {
        run_test("mem/rpsr_test.mem");
    }
    #[test]
    fn rtlr_test() {
        run_test("mem/rtlr_test.mem");
    }

    #[test]
    fn rppc_test() {
        run_test("mem/rppc_test.mem");
    }

    #[test]
    fn rppsr_test() {
        run_test("mem/rppsr_test.mem");
    }

    #[test]
    fn wsp_test() {
        run_test("mem/wsp_test.mem");
    }

    #[test]
    fn wpsr_test() {
        run_test("mem/wpsr_test.mem");
    }

    #[test]
    fn wtlr_test() {
        run_test("mem/wtlr_test.mem");
    }

    #[test]
    fn wthr_test() {
        run_test("mem/wthr_test.mem");
    }

    #[test]
    fn wppc_test() {
        run_test("mem/wppc_test.mem");
    }
    #[test]
    fn wppsr_test() {
        run_test("mem/wppsr_test.mem");
    }

    // cannot test for C2 instructions

    fn run_test(path: &str) {
        let mut zktc = test_setup(path);
        zktc.run().unwrap();
        assert_eq!(zktc.memory.read_from_memory(&0xfffe).unwrap(), 1);
    }

    fn test_setup(path: &str) -> Zktc {
        let f = std::fs::read_to_string(path).unwrap();
        let f = f.split_whitespace().collect::<Vec<_>>();

        let mut test_mem: Vec<u8> = vec![];
        for line in f {
            let mut hex = hex::decode(line).unwrap();
            test_mem.append(&mut hex);
        }

        Zktc::new(test_mem, vec![]).unwrap()
    }
}
