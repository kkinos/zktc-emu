#[derive(Debug)]
pub struct Memory {
    rom: Vec<u8>,
    ram: Vec<u8>,
}

pub const ROM_HIGH_ADDRESS: u16 = 0xFFFF;
pub const ROM_LOW_ADDRESS: u16 = 0x8000;
pub const ROM_SIZE: u16 = (ROM_HIGH_ADDRESS - ROM_LOW_ADDRESS) + 1;

pub const RAM_HIGH_ADDRESS: u16 = 0x7FFF;
pub const RAM_LOW_ADDRESS: u16 = 0x0;
pub const RAM_SIZE: u16 = (RAM_HIGH_ADDRESS - RAM_LOW_ADDRESS) + 1;

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum MemoryError {
    #[error("rom file is too large")]
    TooLargeRomFile(),

    #[error("ram file is too large")]
    TooLargeRamFile(),

    #[error("address 0x{0:04x} is out of range")]
    InvalidAddress(u16),
}

impl Memory {
    pub fn new(rom_file: Vec<u8>, ram_file: Vec<u8>) -> Result<Self, MemoryError> {
        let mut memory = Memory {
            rom: rom_file,
            ram: ram_file,
        };
        if (ROM_SIZE as usize) < memory.rom.len() {
            Err(MemoryError::TooLargeRomFile())?
        }
        if (RAM_SIZE as usize) < memory.ram.len() {
            Err(MemoryError::TooLargeRamFile())?
        }

        for _i in 0..((ROM_SIZE as usize) - memory.rom.len()) {
            memory.rom.push(0);
        }
        for _i in 0..((RAM_SIZE as usize) - memory.ram.len()) {
            memory.ram.push(0);
        }
        Ok(memory)
    }

    pub fn read_from_memory(&self, address: &u16, half: bool) -> Result<u16, MemoryError> {
        let mut data = if (ROM_LOW_ADDRESS..=ROM_HIGH_ADDRESS - 1).contains(address) {
            self.read_from_rom(&(address - ROM_LOW_ADDRESS))
        } else if (RAM_LOW_ADDRESS..=RAM_HIGH_ADDRESS - 1).contains(address) {
            self.read_from_ram(&(address - RAM_LOW_ADDRESS))
        } else {
            Err(MemoryError::InvalidAddress(*address))?
        };
        if half {
            data = ((data & 0x00ff) as i8) as u16; // sign extention
        }
        Ok(data)
    }
    pub fn write_to_memory(
        &mut self,
        address: &u16,
        mut data: u16,
        half: bool,
    ) -> Result<(), MemoryError> {
        if half {
            data &= 0x00ff;
        }
        if (ROM_LOW_ADDRESS..=ROM_HIGH_ADDRESS - 1).contains(address) {
            if half {
                data |= self.read_from_rom(&(address - ROM_LOW_ADDRESS)) & 0xff00;
            }
            self.write_to_rom(&(address - ROM_LOW_ADDRESS), data);
        } else if (RAM_LOW_ADDRESS..=RAM_HIGH_ADDRESS - 1).contains(address) {
            if half {
                data |= self.read_from_ram(&(address - RAM_LOW_ADDRESS)) & 0xff00;
            }
            self.write_to_ram(&(address - RAM_LOW_ADDRESS), data);
        } else {
            Err(MemoryError::InvalidAddress(*address))?
        };
        Ok(())
    }

    fn read_from_rom(&self, address: &u16) -> u16 {
        ((self.rom[(address + 1) as usize] as u16) << 8) | (self.rom[*address as usize] as u16)
    }

    fn read_from_ram(&self, address: &u16) -> u16 {
        ((self.ram[(address + 1) as usize] as u16) << 8) | (self.ram[*address as usize] as u16)
    }

    pub fn write_to_rom(&mut self, address: &u16, data: u16) {
        self.rom[*(address) as usize] = (data & 0x00FF) as u8;
        self.rom[(address + 1) as usize] = ((data & 0xFF00) >> 8) as u8;
    }

    pub fn write_to_ram(&mut self, address: &u16, data: u16) {
        self.ram[*(address) as usize] = (data & 0x00FF) as u8;
        self.ram[(address + 1) as usize] = ((data & 0xFF00) >> 8) as u8;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[should_panic]
    fn rom_file_is_too_large() {
        let rom_file: Vec<u8> = vec![0; (ROM_SIZE + 1) as usize];
        let ram_file: Vec<u8> = vec![0];
        Memory::new(rom_file, ram_file).unwrap();
    }

    #[test]
    #[should_panic]
    fn ram_file_is_too_large() {
        let rom_file: Vec<u8> = vec![0];
        let ram_file: Vec<u8> = vec![0; (RAM_SIZE + 1) as usize];
        Memory::new(rom_file, ram_file).unwrap();
    }
}
