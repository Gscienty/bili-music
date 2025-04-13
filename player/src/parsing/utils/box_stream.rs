use crate::errors;

use super::box_type::BoxType;

pub struct BoxStream<T>(pub T)
where
    T: tokio::io::AsyncReadExt + Unpin;

impl<T> BoxStream<T>
where
    T: tokio::io::AsyncReadExt + Unpin,
{
    pub async fn read_box_type(&mut self) -> Result<BoxType, errors::Error> {
        let result = BoxType(self.0.read_u32_le().await?);

        Ok(result)
    }

    pub async fn read_u8(&mut self) -> Result<u8, errors::Error> {
        let result = self.0.read_u8().await?;

        Ok(result)
    }

    pub async fn read_u16(&mut self) -> Result<u16, errors::Error> {
        let result = self.0.read_u16().await?;

        Ok(result)
    }

    pub async fn read_u24(&mut self) -> Result<u32, errors::Error> {
        let b0 = self.0.read_u8().await?;
        let b1 = self.0.read_u8().await?;
        let b2 = self.0.read_u8().await?;

        Ok(((b0 as u32) << 16) | ((b1 as u32) << 8) | (b2 as u32))
    }

    pub async fn read_u32(&mut self) -> Result<u32, errors::Error> {
        let result = self.0.read_u32().await?;

        Ok(result)
    }

    pub async fn read_u64(&mut self) -> Result<u64, errors::Error> {
        let result = self.0.read_u64().await?;

        Ok(result)
    }

    pub async fn read_exact(&mut self, buf: &mut [u8]) -> Result<(), errors::Error> {
        self.0.read_exact(buf).await?;

        Ok(())
    }

    pub async fn read_box_common_header(
        &mut self,
        parent_size: usize,
    ) -> Result<(BoxType, usize, usize), errors::Error> {
        let mut size = self.read_u32().await? as usize;
        let typ = self.read_box_type().await?;
        let mut hdr_size = 8;

        if size == 1 {
            size = self.read_u64().await? as usize;
            hdr_size += 8;
        } else if size == 0 {
            size = parent_size;
        }

        Ok((typ, size, hdr_size))
    }

    pub async fn read_box_version_flag_header(&mut self) -> Result<(u8, [u8; 3]), errors::Error> {
        let version = self.read_u8().await?;
        let mut flag = [0u8; 3];
        self.read_exact(&mut flag).await?;

        Ok((version, flag))
    }

    pub async fn read_sample_header(&mut self) -> Result<u16, errors::Error> {
        for _ in 0..6 {
            let _ = self.read_u8().await?;
        }
        let data_reference_index = self.read_u16().await?;
        Ok(data_reference_index)
    }

    pub async fn skip_bytes(&mut self, bytes: usize) -> Result<(), errors::Error> {
        for _ in 0..bytes {
            self.0.read_u8().await?;
        }

        Ok(())
    }
}
