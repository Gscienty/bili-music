use crate::{
    errors,
    parsing::{ParseBox, utils},
};

pub const STCO: u32 = utils::box_type_u32(['s', 't', 'c', 'o']);
#[derive(Debug)]
pub struct ChunkOffsetBox {
    chunk_offsets: Vec<u32>,
}

impl ParseBox for ChunkOffsetBox {
    async fn parse(
        stream: &mut utils::BoxStream<impl tokio::io::AsyncReadExt + Unpin>,
        typ: utils::BoxType,
        mut size: usize,
    ) -> Result<Self, errors::Error> {
        let (version, _) = stream.read_box_version_flag_header().await?;

        let entry_count = stream.read_u32().await?;
        let mut chunk_offsets = Vec::new();
        if version == 0 {
            for _ in 0..entry_count {
                chunk_offsets.push(stream.read_u32().await?);
            }
        }

        Ok(Self { chunk_offsets })
    }
}
