use std::io::Write;

use byteorder::{BigEndian, WriteBytesExt};

use crate::VarIntWrite;

pub struct Lucky5VarIntWrite;

impl VarIntWrite for Lucky5VarIntWrite {
    #[inline]
    fn write<W: Write>(mut writer: W, value: i32) -> std::io::Result<()> {
        let value = value as u32;

        let w = (value & 0x7F | 0x80) << 24
            | ((value >> 7) & 0x7F | 0x80) << 16
            | ((value >> 14) & 0x7F | 0x80)
            | ((value >> 21) & 0x7F | 0x80);
        writer.write_u32::<BigEndian>(w)?;
        writer.write_u8((value >> 28) as u8)?;

        Ok(())
    }
}