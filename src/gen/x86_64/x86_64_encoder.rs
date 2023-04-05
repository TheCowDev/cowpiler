use crate::gen::x86_64::x86_64_allocator::X86Register;
use crate::misc::byte_writer::ByteWriter;

pub(crate) struct X86_64Encoder {
    writer: ByteWriter,
}

impl X86_64Encoder {
    pub(crate) fn new() -> Self {
        Self { writer: ByteWriter::new() }
    }

    pub(crate) fn move_reg_i64(&mut self, dest: X86Register, value: i64) -> usize {
        self.writer.write_u8(0x48); // REX prefix
        self.writer.write_u8(0xB8 | (dest.encode() & 0x7)); //src register
        self.writer.write_i64(value)
    }

    pub(crate) fn mov_reg_to_reg(&mut self, src: X86Register, dst: X86Register) {
        if src == dst {
            return;
        }

        let src_reg: u8 = src.encode();
        let dest_reg: u8 = dst.encode();

        let mut rex: u8 = 0x40;
        rex |= 1 << 3; // REX.W bit for 64-bit operation
        rex |= ((src_reg & 8) >> 1) | ((dest_reg & 8) >> 3); // REX.R and REX.B bits for register extension
        self.writer.write_u8(rex); // write ref prefix

        self.writer.write_u8(0x89); // opcode for mov

        let mut modrm: u8 = 0;
        modrm |= 3 << 6; // register-to-register encoding
        modrm |= (src_reg & 7) << 3; // 3 bits for destination register
        modrm |= dest_reg & 7; // 3 bits for source register
        self.writer.write_u8(modrm);
    }

    pub(crate) fn move_reg_to_xmm(&mut self, src: X86Register, dst: X86Register) {
        self.writer.write_u8(0x66);
        self.writer.write_u8(0x48);
        self.writer.write_u8(0x0F);
        self.writer.write_u8(0x6E);
        self.writer.write_u8(0xC0 | (dst.encode() & 0x7) << 3 | (src.encode() & 0x7));
    }

    pub(crate) fn mov_xmm_to_xmm(&mut self, src: X86Register, dst: X86Register) {
        if src == dst {
            return;
        }

        let src_reg: u8 = src.encode();
        let dest_reg: u8 = dst.encode();

        if src_reg >= X86Register::XMM8.encode() || dest_reg > X86Register::XMM8.encode() {
            self.writer.write_u8(0x40 | ((dest_reg & 8) >> 1) | ((src_reg & 8) >> 3));
            self.writer.write_u8(0x0F);
            self.writer.write_u8(0x28);
            self.writer.write_u8(0xC0 | ((dest_reg & 7) << 3) | (src_reg & 7));
        } else {
            self.writer.write_u8(0x0F);
            self.writer.write_u8(0x28);
            self.writer.write_u8(0xC0 | (dest_reg << 3) | src_reg);
        }
    }

    //load
    pub(crate) fn mov_mem_to_reg(&mut self, mem_reg: X86Register, dest_reg: X86Register) {
        self.writer.write_u8(0x48);
        self.writer.write_u8(0x8B);
        self.writer.write_u8(0x00 | ((mem_reg.encode() & 0x07) << 3) | (dest_reg.encode() & 0x07));
    }

    //store
    pub(crate) fn mov_reg_to_mem(&mut self, reg: X86Register, mem_reg: X86Register) {
        self.writer.write_u8(0x48);
        self.writer.write_u8(0x89);
        self.writer.write_u8(0x00 | ((reg.encode() & 0x07) << 3) | (mem_reg.encode() & 0x07));
    }

    pub(crate) fn add_reg_reg(&mut self, left: X86Register, right: X86Register) {
        self.writer.write_u8(0x01);
        let mut modrm: u8 = 0;
        modrm |= 3 << 6; // set operation to 11 (register-to-register)
        modrm |= right.encode() << 3; // set destination register
        modrm |= left.encode(); // set source register
        self.writer.write_u8(modrm);
    }

    pub(crate) fn add_xmm_xmm(&mut self, left: X86Register, right: X86Register) {
        let mut rex: u8 = 0x40;
        if left.encode() >= 8 { rex |= 0x04; };
        if right.encode() >= 8 { rex |= 0x01; };

        self.writer.write_u8(rex);
        self.writer.write_u8(0x66);
        self.writer.write_u8(0x0F);
        self.writer.write_u8(0x58);
        self.writer.write_u8(0xC0 | ((left.encode() & 0x07) << 3) | (right.encode() & 0x07));
    }

    pub(crate) fn sub_reg_reg(&mut self, left: X86Register, right: X86Register) {
        self.writer.write_u8(0x29);
        let mut modrm: u8 = 0;
        modrm |= 3 << 6; // set operation to 11 (register-to-register)
        modrm |= right.encode() << 3; // set destination register
        modrm |= left.encode(); // set source register
        self.writer.write_u8(modrm);
    }

    pub(crate) fn mul_reg_reg(&mut self, left: X86Register, right: X86Register) {
        self.writer.write_u8(0x0F);
        self.writer.write_u8(0xAF);
        let mut modrm: u8 = 0;
        modrm |= 3 << 6; // set operation to 11 (register-to-register)
        modrm |= right.encode() << 3; // set destination register
        modrm |= left.encode(); // set source register
        self.writer.write_u8(modrm);
    }

    pub(crate) fn div_reg_reg(&mut self, divisor: X86Register) {
        // idiv only works on rax and rdx
        self.writer.write_u8(0xF7);
        let mut modrm: u8 = 0;
        modrm |= 3 << 6; // set operation to 11 (register-to-register)
        modrm |= 7 << 3; // set the idiv operation within the 0xF7 opcode
        modrm |= divisor.encode(); // set divisor register
        self.writer.write_u8(modrm);
    }

    pub(crate) fn eq_reg_reg(&mut self, left: X86Register, right: X86Register) {
        self.writer.write_u8(0x39);
        let mut modrm: u8 = 0;
        modrm |= 3 << 6; // set operation to 11 (register-to-register)
        modrm |= right.encode() << 3; // set destination register
        modrm |= left.encode(); // set source register
        self.writer.write_u8(modrm);
    }

    pub(crate) fn push_reg(&mut self, reg: X86Register) {
        if reg.is_xmm() {
            self.writer.write_u8(0x48); // rex.w prefix
            self.writer.write_u8(0x83); // opcode for 'sub'
            self.writer.write_u8(0xEC); // modR/M byte for 'sub' with RSP
            self.writer.write_u8(0x10); //Immediate 16
        } else {

            // below r8 can have a for condense way to be pushed
            if reg.encode() < X86Register::R8.encode() {
                self.writer.write_u8(0x50 + reg.encode());
            } else {
                self.writer.write_u8(0x41); //rex prefix for extended registers
                self.writer.write_u8(0x50 + (reg.encode() - X86Register::R8.encode()));
            }
        }
    }

    pub(crate) fn pop_reg(&mut self, reg: X86Register) {
        if reg.is_xmm() {
            // movaps xmmN, [rsp]
            self.writer.write_u8(0x48);
            self.writer.write_u8(0x0F);
            self.writer.write_u8(0x28);
            self.writer.write_u8(0x04 | (reg.encode() << 3));

            // movaps [rsp], xmm
            self.writer.write_u8(0x48);
            self.writer.write_u8(0x83);
            self.writer.write_u8(0xC4);
            self.writer.write_u8(0x10);
        } else {
            // below r8 have a condense way to be pushed
            if reg.encode() < X86Register::R8.encode() {
                self.writer.write_u8(0x58 + reg.encode());
            } else {
                self.writer.write_u8(0x41);
                self.writer.write_u8(0x58 + (reg.encode() - X86Register::R8.encode()));
            }
        }
    }

    pub(crate) fn jmp(&mut self) -> usize {
        self.writer.write_u8(0xE9);
        self.writer.write_i32(0)
    }

    pub(crate) fn cond_jmp(&mut self, reg: X86Register) -> usize {
        // cmp reg,0
        self.writer.write_u8(0x48);
        self.writer.write_u8(0x83);
        self.writer.write_u8(0xF8 | (reg.encode() & 7));
        self.writer.write_u8(0x00);

        // jz, jump if zero
        self.writer.write_u8(0x0F);
        self.writer.write_u8(0x084);
        self.writer.write_i32(0)
    }

    pub(crate) fn ret(&mut self) {
        self.writer.write_u8(0xC3);
    }

    pub(crate) fn push_shadow(&mut self) {
        self.writer.write_u8(0x48);
        self.writer.write_u8(0x83);
        self.writer.write_u8(0xec);
        self.writer.write_u8(0x20);
    }

    pub(crate) fn pop_shadow(&mut self) {
        self.writer.write_u8(0x48);
        self.writer.write_u8(0x83);
        self.writer.write_u8(0xc4);
        self.writer.write_u8(0x20);
    }

    pub(crate) fn call(&mut self, reg: X86Register) {
        self.writer.write_u8(0x48);
        self.writer.write_u8(0xFF);
        self.writer.write_u8(0xD0 | (reg.encode() & 0x07));
    }

    pub(crate) fn bytes(&self) -> &Vec<u8> {
        self.writer.bytes()
    }
}