use std::io::{self, Write};

use byteorder::{WriteBytesExt, BigEndian};

use crate::VarIntWrite;

pub struct BlendedVarIntWrite;

impl VarIntWrite for BlendedVarIntWrite {
    #[inline]
    fn write<W: Write>(mut writer: W, value: i32) -> io::Result<()> {
        const MASK: u32 = 0xFF_FF_FF_FF;

        let value = value as u32;

        if (value & (MASK << 7)) == 0 {
            writer.write_u8(value as u8)?;
        } else if (value & (MASK << 14)) == 0 {
            let w = (value & 0x7F | 0x80) << 8
                | (value >> 7);
            writer.write_u16::<BigEndian>(w as u16)?;
        } else if (value & (MASK << 21)) == 0 {
            let w = (value & 0x7F | 0x80) << 16
                | ((value >> 7) & 0x7F | 0x80) << 8
                | (value >> 14);
            writer.write_u24::<BigEndian>(w)?;
        } else if (value & (MASK << 28)) == 0 {
            let w = (value & 0x7F | 0x80) << 24
                | ((value >> 7) & 0x7F | 0x80) << 16
                | ((value >> 14) & 0x7F | 0x80) << 8
                | (value >> 21);
            writer.write_u32::<BigEndian>(w)?;
        } else {
            let w = (value & 0x7F | 0x80) << 24
                | ((value >> 7) & 0x7F | 0x80) << 16
                | ((value >> 14) & 0x7F | 0x80) << 8
                | ((value >> 21) & 0x7F | 0x80);
            writer.write_u32::<BigEndian>(w)?;
            writer.write_u8((value >> 28) as u8)?;
        }

        Ok(())
    }
}
