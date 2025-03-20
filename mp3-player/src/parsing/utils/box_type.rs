use std::fmt::{Display, Write};

#[derive(Debug, Clone, Copy)]
pub struct BoxType(pub u32);

impl Display for BoxType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char((self.0 & 0xFF) as u8 as char)?;
        f.write_char(((self.0 >> 8) & 0xFF) as u8 as char)?;
        f.write_char(((self.0 >> 16) & 0xFF) as u8 as char)?;
        f.write_char(((self.0 >> 24) & 0xFF) as u8 as char)?;

        Ok(())
    }
}

pub const fn box_type_u32(v: [char; 4]) -> u32 {
    (v[0] as u32) | ((v[1] as u32) << 8) | ((v[2] as u32) << 16) | ((v[3] as u32) << 24)
}
