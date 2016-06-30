use super::apu::Apu;
use super::ppu::Ppu;
use super::cart::Cartridge;

pub struct Interconnect {
    ram: Box<[u8]>,
    apu: Apu,
    ppu: Ppu,
    cart: Cartridge,
}

impl Interconnect {
    // TODO Implement chr_rom, prg_ram, and prg_rom
    pub fn new(cart_rom: Vec<u8>) -> Interconnect {
        Interconnect {
            ram: vec![0; 0x0800].into_boxed_slice(),
            apu: Apu::default(),
            ppu: Ppu::default(),
            cart: Cartridge::new(cart_rom),
        }
    }

    pub fn read_byte(&self, virt_addr: u16) -> u8 {
        use super::mem_map::*;
        let phys_addr = map_virt_addr(virt_addr);
        println!("phys_addr: {:?}", phys_addr);
        match phys_addr {
            PhysAddr::CpuRam(addr) => {self.ram[addr as usize]},
            PhysAddr::RamMirrorOne(addr) => {self.ram[(addr - 0x0800) as usize]},
            PhysAddr::RamMirrorTwo(addr) => {self.ram[(addr - 2 * 0x0800) as usize]},
            PhysAddr::RamMirrorThree(addr) => {self.ram[(addr - 3 * 0x0800) as usize]},
            PhysAddr::PpuRegs(addr) => {self.ppu.read_reg(addr - 0x2000)},
            PhysAddr::PpuMirrors(addr) => {self.ppu.read_reg((addr - 0x2000) % 8)},
            PhysAddr::ApuRegs(addr) => {self.apu.read_reg(addr - 0x4000)},
            PhysAddr::CartSpace(addr) => {self.cart.read_cart(addr - 0x8000)},
        }
    }

    pub fn read_word(&self, virt_addr: u16) -> u16 {
        use super::mem_map::*;
        let phys_addr = map_virt_addr(virt_addr);
        match phys_addr {
            PhysAddr::CpuRam(addr) =>         {self.ram[addr as usize] as u16 |
                                               (self.ram[(addr + 1) as usize] as u16) << 8},
            PhysAddr::RamMirrorOne(addr) =>   {self.ram[(addr - 0x0800) as usize] as u16 |
                                               (self.ram[(addr + 1 -0x0800) as usize] as u16) << 8},
            PhysAddr::RamMirrorTwo(addr) =>   {self.ram[(addr - 2 * 0x0800) as usize] as u16 |
                                               (self.ram[(addr + 1 - (2 * 0x0800)) as usize] as u16) << 8},
            PhysAddr::RamMirrorThree(addr) => {self.ram[(addr - 3 * 0x0800) as usize] as u16 |
                                               (self.ram[(addr + 1 - (3 * 0x0800)) as usize] as u16) << 8},
            PhysAddr::CartSpace(addr) => {self.cart.read_cart(addr - 0x4020) as u16 | (self.cart.read_cart(addr + 1 -0x4020) as u16) << 8},
            _ => panic!("{:?} does not support reading words", phys_addr),
        }
    }
}
