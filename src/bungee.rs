use crate::VarIntWrite;
use std::io::Write;
use byteorder::WriteBytesExt;

pub struct BungeeVarIntWrite;

impl VarIntWrite for BungeeVarIntWrite {
    #[inline]
    fn write<W: Write>(mut writer: W, value: i32) -> std::io::Result<()> {
        let mut value = value as u32;
        let mut part;
        loop {
            part = (value & 0x7F) as u8;
            value >>= 7;
            if value != 0 {
                part |= 0x80;
            }
            writer.write_u8(part)?;
            if value == 0 {
                return Ok(());
            }
        }
    }
}