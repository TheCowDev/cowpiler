use crate::gen::x86_64::x86_64_allocator::X86Register;
use crate::misc::byte_writer::ByteWriter;

pub(crate) struct X86_64Encoder {
    writer: ByteWriter,
}

impl X86_64Encoder {
    pub(crate) fn new() -> Self {
        X86_64Encoder { writer: ByteWriter::new() }
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

    pub(crate) fn add_reg_reg(&mut self, left: X86Register, right: X86Register) {
        self.writer.write_u8(0x01);
        let mut modrm: u8 = 0;
        modrm |= 3 << 6; // set operation to 11 (register-to-register)
        modrm |= right.encode() << 3; // set destination register
        modrm |= left.encode(); // set source register
        self.writer.write_u8(modrm);
    }

    pub(crate) fn ret(&mut self) {
        self.writer.write_u8(0xC3);
    }

    pub(crate) fn bytes(&self) -> &Vec<u8> {
        self.writer.bytes()
    }
}