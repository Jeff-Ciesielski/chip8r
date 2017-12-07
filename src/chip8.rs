extern crate rand;

use chip8::rand::Rng;

pub const SCREEN_X: usize = 64;
pub const SCREEN_Y: usize = 32;
const CHAR_SPRITES: [u8; 80] = [0xf0, 0x90, 0x90, 0x90, 0xf0,
                                0x20, 0x60, 0x20, 0x20, 0x70,
                                0xf0, 0x10, 0xf0, 0x80, 0xf0,
                                0xf0, 0x10, 0xf0, 0x10, 0xf0,
                                0x90, 0x90, 0xf0, 0x10, 0x10,
                                0xf0, 0x80, 0xf0, 0x10, 0xf0,
                                0xf0, 0x80, 0xf0, 0x90, 0xf0,
                                0xf0, 0x10, 0x20, 0x40, 0x40,
                                0xf0, 0x90, 0xf0, 0x90, 0xf0,
                                0xf0, 0x90, 0xf0, 0x10, 0xf0,
                                0xf0, 0x90, 0xf0, 0x90, 0x90,
                                0xe0, 0x90, 0xe0, 0x90, 0xe0,
                                0xf0, 0x80, 0x80, 0x80, 0xf0,
                                0xe0, 0x90, 0x90, 0x90, 0xe0,
                                0xf0, 0x80, 0xf0, 0x80, 0xf0,
                                0xf0, 0x80, 0xf0, 0x80, 0x80];


pub struct Core {
    pub frame_buffer: [u8; (SCREEN_X / 8) * SCREEN_Y],
    memory: [u8; 0x1000],
    registers: [u8; 0x10],
    stack: [u16; 0x10],
    sp: u8,
    i: u16,
    pc: u16,
    dt: u8,
    st: u8
}

impl Core {

    pub fn load_rom(&mut self, rom: &Vec<u8>) {
        println!("Loading Rom");
        for (i, elem) in rom.iter().enumerate() {
            self.memory[0x200 + i] = *elem;
        }
    }

    fn load_sprites(&mut self) {
        for (i, val) in CHAR_SPRITES.iter().enumerate() {
            self.memory[i] = *val;
        }
    }

    pub fn new() -> Core {
        let mut result = Core{
            frame_buffer: [0u8; (SCREEN_X / 8) * SCREEN_Y],
            memory: [0u8; 0x1000],
            registers: [0u8; 0x10],
            stack: [0u16; 0x10],
            sp: 0,
            i: 0,
            pc: 0x200,
            dt: 0,
            st: 0
        };
        result.soft_reset();
        result.load_sprites();
        result
    }

    fn op_cls(&mut self, _inst: u16) {
        println!("Clearing Screen");
        for (_, elem) in self.frame_buffer.iter_mut().enumerate() {
            *elem = 0;
        }
    }

    fn op_ret(&mut self, _inst: u16) {
        println!("ret");
        self.pc = self.stack[self.sp as usize];
        self.sp -= 1;
    }

    fn op_jp(&mut self, inst: u16) {
        println!("jp");
        let tgt_addr = inst & 0xfff;
        self.pc = tgt_addr;
    }

    fn op_call(&mut self, inst: u16) {
        println!("call");
        self.sp += 1;
        self.stack[self.sp as usize] = self.pc;
        self.pc = inst & 0xfff;
    }

    fn op_se(&mut self, inst: u16) {
        println!("se");
        let rx = ((inst & 0xf00) >> 8) as usize;
        let comp = (inst & 0xff) as u8;

        if self.registers[rx] == comp {
            self.pc += 2;
        }
    }

    fn op_sne(&mut self, inst: u16) {
        println!("sne");
        let rx = ((inst & 0xf00) >> 8) as usize;
        let comp = (inst & 0xff) as u8;

        if self.registers[rx] != comp {
            self.pc += 2;
        }
    }

    fn op_sereg(&mut self, inst: u16) {
        println!("sereg");
        let rx = ((inst & 0xf00) >> 8) as usize;
        let ry = ((inst & 0xf0) >> 4) as usize;

        if self.registers[rx] == self.registers[ry] {
            self.pc += 2;
        }
    }

    fn op_ld(&mut self, inst: u16) {
        println!("ld");
        let rx = ((inst & 0xf00) >> 8) as usize;
        let val = (inst & 0xff) as u8;

        self.registers[rx] = val;
    }

    fn op_add(&mut self, inst: u16) {
        println!("add");
        let rx = ((inst & 0xf00) >> 8) as usize;
        let val = inst & 0xff;

        // Wrapping_add to allow integer overflow
        self.registers[rx] = self.registers[rx]
            .wrapping_add(val as u8);
    }

    fn op_ldreg(&mut self, inst: u16) {
        println!("ldreg");
        let rx = ((inst & 0xf00) >> 8) as usize;
        let ry = ((inst & 0xf0) >> 4) as usize;

        self.registers[rx] = self.registers[ry];
    }

    fn op_or(&mut self, inst: u16) {
        println!("or");
        let rx = ((inst & 0xf00) >> 8) as usize;
        let ry = ((inst & 0xf0) >> 4) as usize;

        self.registers[rx] = self.registers[rx] | self.registers[ry];
    }

    fn op_and(&mut self, inst: u16) {
        println!("and");
        let rx = ((inst & 0xf00) >> 8) as usize;
        let ry = ((inst & 0xf0) >> 4) as usize;
        self.registers[rx] = self.registers[rx] & self.registers[ry];
    }

    fn op_xor(&mut self, inst: u16) {
        println!("xor");
        let rx = ((inst & 0xf00) >> 8) as usize;
        let ry = ((inst & 0xf0) >> 4) as usize;

        let val = self.registers[rx] ^ self.registers[ry];
        self.registers[rx] = val;
    }

    fn op_addcarry(&mut self, inst: u16) {
        println!("addcarry");
        let rx = ((inst & 0xf00) >> 8) as usize;
        let ry = ((inst & 0xf0) >> 4) as usize;

        let result = (self.registers[rx] as u16)
            .wrapping_add(self.registers[ry] as u16);

        self.registers[rx] = (result & 0xff) as u8;
        self.registers[0xf] = ((result >> 8) & 0x1) as u8;
    }

    fn op_sub(&mut self, inst: u16) {
        println!("sub");
        let rx = ((inst & 0xf00) >> 8) as usize;
        let ry = ((inst & 0xf0) >> 4) as usize;

        if self.registers[rx] < self.registers[ry] {
            self.registers[0xf] = 1;
        } else {
            self.registers[0xf] = 0;
        }

        // Use wrapping_sub here to allow for integer overflow
        self.registers[rx] = self.registers[rx]
            .wrapping_sub(self.registers[ry]);
    }

    fn op_shr(&mut self, inst: u16) {
        println!("shr");
        let rx = ((inst & 0xf00) >> 8) as usize;

        self.registers[0xf] = self.registers[rx] & 0x1;
        self.registers[rx] = self.registers[rx] >> 1;
    }

    fn op_subn(&mut self, inst: u16) {
        println!("subn");
        let rx = ((inst & 0xf00) >> 8) as usize;
        let ry = ((inst & 0xf0) >> 4) as usize;

        if self.registers[rx] > self.registers[ry] {
            self.registers[0xf] = 1;
        } else {
            self.registers[0xf] = 0;
        }

        self.registers[rx] = self.registers[rx]
            .wrapping_sub(self.registers[ry]);
    }

    fn op_shl(&mut self, inst: u16) {
        println!("shl");
        let rx = ((inst & 0xf00) >> 8) as usize;

        self.registers[0xf] = (self.registers[rx] & 0x80) >> 7;
        self.registers[rx] = self.registers[rx] << 1;
    }

    fn op_snereg(&mut self, inst: u16) {
        println!("snereg");
        let rx = ((inst & 0xf00) >> 8) as usize;
        let ry = ((inst & 0xf0) >> 4) as usize;

        if self.registers[rx] != self.registers[ry] {
            self.pc += 2;
        }
    }

    fn op_ldi(&mut self, inst: u16) {
        println!("ldi");
        self.i = inst & 0xfff;
    }

    fn op_jp_offset(&mut self, inst: u16) {
        println!("jp_offset");
        self.pc = (inst & 0xfff) + self.registers[0] as u16;
    }

    fn op_rnd(&mut self, inst: u16) {
        println!("rnd");
        let rx = ((inst & 0xf00) >> 8) as usize;
        let mask = (inst & 0xff) as u8;

        let rand_val = rand::thread_rng().gen_range(0x00, 0x100) as u8;
        self.registers[rx] = rand_val & (inst as u8 & mask);
    }

    fn op_drw(&mut self, inst: u16) {
        println!("drw");
        let rx = ((inst & 0xf00) >> 8) as usize;
        let ry = ((inst & 0xf0) >> 4) as usize;
        let n = inst & 0xf;

        let x = self.registers[rx];
        let y = self.registers[ry];

        let remainder_bits = x % 8;

        // Clear Vf
        self.registers[0xf] = 0;
        let cur_idx = (x as usize / (SCREEN_X / 8)) as usize;
        for offset in 0..n {
            let y_idx = (y as u16 + offset) as usize;;
            let sprite_byte = self.memory[(self.i + offset) as usize];
            let mask = (0xff >> remainder_bits) as u8;
            let idx = (SCREEN_Y) * (cur_idx) + (y_idx);

            println!("{}, {}", cur_idx, y_idx);

            if self.frame_buffer[idx] & mask != 0 {
                println!("Collision!");
                self.registers[0xf] |= 0x1;
            }

            self.frame_buffer[idx] ^= sprite_byte >> remainder_bits;

            // Handle being on the edge of a sprite
            if remainder_bits > 0 {
                let ovf_idx = (cur_idx + 1) % (SCREEN_X / 8);
                let ovf_mask = 0xff << remainder_bits;
                let idx = (SCREEN_Y) * (ovf_idx) + (y_idx);

                println!("OVERFLOW: {}, {}", ovf_idx, y_idx);

                if (self.frame_buffer[idx] & ovf_mask) != 0 {
                    self.registers[0xf] = self.registers[0xf] | 0x1;
                }

                self.frame_buffer[idx] ^= sprite_byte << (8 - remainder_bits);
            }
        }
    }

    fn op_skp(&mut self, inst: u16) {
        println!("skp");
        let rx = ((inst & 0xf00) >> 8) as usize;

        // TODO: Check key pressed
        if false {
            self.pc += 2;
        }
    }

    fn op_sknp(&mut self, inst: u16) {
        println!("sknp");
        let rx = ((inst & 0xf00) >> 8) as usize;

        // TODO: Check key pressed
        if true {
            self.pc += 2;
        }
    }

    fn op_ldreg_dt(&mut self, inst: u16) {
        println!("ldreg_dt");
        let rx = ((inst & 0xf00) >> 8) as usize;

        self.registers[rx] = self.dt
    }

    fn op_ldreg_key(&mut self, inst: u16) {
        println!("ldreg_key");
        let rx = ((inst & 0xf00) >> 8) as usize;

        // TODO: Get key
        self.registers[rx] = 0;
    }

    fn op_lddt_reg(&mut self, inst: u16) {
        println!("lddt_reg");
        let rx = ((inst & 0xf00) >> 8) as usize;

        self.dt = self.registers[rx];
    }

    fn op_ldst_reg(&mut self, inst: u16) {
        println!("ldst_reg");
        let rx = ((inst & 0xf00) >> 8) as usize;

        self.st = self.registers[rx];
    }

    fn op_addi_reg(&mut self, inst: u16) {
        println!("addi_reg");
        let rx = ((inst & 0xf00) >> 8) as usize;

        self.i += self.registers[rx] as u16;
    }

    fn op_ldf(&mut self, inst: u16) {
        println!("ldf");
        let rx = ((inst & 0xf00) >> 8) as usize;

        self.i = self.registers[rx] as u16 * 5;
    }

    fn op_ldb(&mut self, inst: u16) {
        println!("ldb");
        let rx = ((inst & 0xf00) >> 8) as usize;
        let mut val = self.registers[rx];

        for i in 0..3 {
            let digit = val % 10;
            val = val / 10;
            self.memory[(self.i + (2 - i)) as usize] = digit
        }
    }

    fn op_ldreg_mem(&mut self, inst: u16) {
        println!("ldreg_mem");
        let rx = ((inst & 0xf00) >> 8) as usize;

        for i in 0..rx + 1 {
            self.memory[(self.i + i as u16) as usize] = self.registers[i];
        }
    }

    fn op_ldmem_reg(&mut self, inst: u16) {
        println!("ldmem_reg");
        let rx = ((inst & 0xf00) >> 8) as usize;

        for i in 0..rx + 1 {
            self.registers[i] = self.memory[(self.i + i as u16) as usize];
        }
    }

    pub fn soft_reset(&mut self) {
        println!("Performing soft reset");
        self.pc = 0x200;
        self.i = 0x00;
        self.sp = 0x00;

        // Clear the framebuffer
        self.op_cls(0x00);
    }

    fn trap(inst: u16) {
        println!("ERROR: Unable to process opcode: {:x}", inst);
        loop {}
    }

    fn execute(&mut self, inst: u16) {
        let n0 = (inst & 0xf) as u8;
        let n1 = (inst >> 4) as u8 & 0xf;
        let n2 = (inst >> 8) as u8 & 0xf;
        let n3 = (inst >> 12) as u8 & 0xf;

        match (n3, n2, n1, n0) {
            (0x0,   _, 0xE, 0x0) => self.op_cls(inst),
            (0x0,   _, 0xE, 0xE) => self.op_ret(inst),
            (0x1,   _,   _,   _) => self.op_jp(inst),
            (0x2,   _,   _,   _) => self.op_call(inst),
            (0x3,   _,   _,   _) => self.op_se(inst),
            (0x4,   _,   _,   _) => self.op_sne(inst),
            (0x5,   _,   _,   _) => self.op_sereg(inst),
            (0x6,   _,   _,   _) => self.op_ld(inst),
            (0x7,   _,   _,   _) => self.op_add(inst),
            (0x8,   _,   _, 0x0) => self.op_ldreg(inst),
            (0x8,   _,   _, 0x1) => self.op_or(inst),
            (0x8,   _,   _, 0x2) => self.op_and(inst),
            (0x8,   _,   _, 0x3) => self.op_xor(inst),
            (0x8,   _,   _, 0x4) => self.op_addcarry(inst),
            (0x8,   _,   _, 0x5) => self.op_sub(inst),
            (0x8,   _,   _, 0x6) => self.op_shr(inst),
            (0x8,   _,   _, 0x7) => self.op_subn(inst),
            (0x8,   _,   _, 0xE) => self.op_shl(inst),
            (0x9,   _,   _, 0x0) => self.op_snereg(inst),
            (0xA,   _,   _,   _) => self.op_ldi(inst),
            (0xB,   _,   _,   _) => self.op_jp_offset(inst),
            (0xC,   _,   _,   _) => self.op_rnd(inst),
            (0xD,   _,   _,   _) => self.op_drw(inst),
            (0xE,   _, 0x9, 0xE) => self.op_skp(inst),
            (0xE,   _, 0xA, 0x1) => self.op_sknp(inst),
            (0xF,   _, 0x0, 0x7) => self.op_ldreg_dt(inst),
            (0xF,   _, 0x0, 0xA) => self.op_ldreg_key(inst),
            (0xF,   _, 0x1, 0x5) => self.op_lddt_reg(inst),
            (0xF,   _, 0x1, 0x8) => self.op_ldst_reg(inst),
            (0xF,   _, 0x1, 0xE) => self.op_addi_reg(inst),
            (0xF,   _, 0x2, 0x9) => self.op_ldf(inst),
            (0xF,   _, 0x3, 0x3) => self.op_ldb(inst),
            (0xF,   _, 0x5, 0x5) => self.op_ldreg_mem(inst),
            (0xF,   _, 0x6, 0x5) => self.op_ldmem_reg(inst),
            (_,_,_,_) => Core::trap(inst),
        }
    }

    fn fetch(&mut self) -> u16 {
        let result: u16 = (self.memory[self.pc as usize] as u16) << 8 | self.memory[self.pc as usize + 1] as u16;
        print!("Fetch[0x{:04x}]: 0x{:04x} | ",  self.pc, result);

        self.pc += 2;
        result
    }

    pub fn tick(&mut self) {
        let inst = self.fetch();
        self.execute(inst);
        if self.dt > 0 {
            self.dt -= 1;
        }
    }
}
