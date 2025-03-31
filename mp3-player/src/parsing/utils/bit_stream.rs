use std::collections::VecDeque;

use crate::errors;

use super::BoxStream;

pub struct BitStream<'s, T>
where
    T: tokio::io::AsyncReadExt + Unpin,
{
    bits: VecDeque<u8>,
    stream: &'s mut BoxStream<T>,
    consumed_bytes: usize,
}

impl<'s, T> BitStream<'s, T>
where
    T: tokio::io::AsyncReadExt + Unpin,
{
    pub fn new(stream: &'s mut BoxStream<T>) -> Self {
        Self {
            stream,
            bits: VecDeque::with_capacity(8),
            consumed_bytes: 0,
        }
    }

    async fn fill(&mut self) -> Result<(), errors::Error> {
        let byte = self.stream.read_u8().await?;
        self.consumed_bytes += 1;

        self.bits.push_front((byte & 0b1000_0000) >> 7);
        self.bits.push_front((byte & 0b0100_0000) >> 6);
        self.bits.push_front((byte & 0b0010_0000) >> 5);
        self.bits.push_front((byte & 0b0001_0000) >> 4);
        self.bits.push_front((byte & 0b0000_1000) >> 3);
        self.bits.push_front((byte & 0b0000_0100) >> 2);
        self.bits.push_front((byte & 0b0000_0010) >> 1);
        self.bits.push_front(byte & 0b0000_0001);

        Ok(())
    }

    async fn peek(&mut self) -> Result<u8, errors::Error> {
        if self.bits.is_empty() {
            self.fill().await?;
        }
        Ok(self.bits.pop_back().unwrap_or_default())
    }

    pub const fn get_consumed_bytes(&self) -> usize {
        self.consumed_bytes
    }

    pub async fn read_u8(&mut self, length: usize) -> Result<u8, errors::Error> {
        let mut bits = 0u8;
        for _ in 0..length {
            bits <<= 1;
            bits |= self.peek().await?;
        }

        Ok(bits)
    }

    pub async fn read_u16(&mut self, length: usize) -> Result<u16, errors::Error> {
        let mut bits = 0u16;
        for _ in 0..length {
            bits <<= 1;
            bits |= self.peek().await? as u16;
        }

        Ok(bits)
    }

    pub async fn read_u32(&mut self, length: usize) -> Result<u32, errors::Error> {
        let mut bits = 0u32;
        for _ in 0..length {
            bits <<= 1;
            bits |= self.peek().await? as u32;
        }

        Ok(bits)
    }
}
