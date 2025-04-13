use crate::{
    errors,
    parsing::{ParseBox, utils},
};

pub const STSC: u32 = utils::box_type_u32(['s', 't', 's', 'c']);
#[derive(Debug)]
pub struct SampleToChunkBox {
    first_chunk: Vec<u32>,
    samples_per_chunk: Vec<u32>,
    sample_description_index: Vec<u32>,
}

impl ParseBox for SampleToChunkBox {
    async fn parse(
        stream: &mut utils::BoxStream<impl tokio::io::AsyncReadExt + Unpin>,
        typ: utils::BoxType,
        _size: usize,
    ) -> Result<Self, errors::Error> {
        let (version, _) = stream.read_box_version_flag_header().await?;

        let mut first_chunk = Vec::new();
        let mut samples_per_chunk = Vec::new();
        let mut sample_description_index = Vec::new();
        let entry_count = stream.read_u32().await?;
        if version == 0 {
            for _ in 0..entry_count {
                first_chunk.push(stream.read_u32().await?);
                samples_per_chunk.push(stream.read_u32().await?);
                sample_description_index.push(stream.read_u32().await?);
            }
        }

        Ok(Self {
            first_chunk,
            samples_per_chunk,
            sample_description_index,
        })
    }
}
