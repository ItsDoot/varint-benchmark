use crate::VarIntWrite;
use std::io::Write;
use byteorder::WriteBytesExt;

pub struct VelocityVarIntWrite;

impl VarIntWrite for VelocityVarIntWrite {
    #[inline]
    fn write<W: Write>(mut writer: W, value: i32) -> std::io::Result<()> {
        let mut value = value as u32;

        loop {
            if value & 0xFF_FF_FF_80 == 0 {
                writer.write_u8(value as u8)?;
                return Ok(());
            }

            writer.write_u8((value & 0x7F | 0x80) as u8)?;
            value >>= 7;
        }
    }
}