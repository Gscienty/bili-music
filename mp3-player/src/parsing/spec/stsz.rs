use crate::{
    errors,
    parsing::{ParseBox, utils},
};

pub const STSZ: u32 = utils::box_type_u32(['s', 't', 's', 'z']);
#[derive(Debug)]
pub struct SampleSizeBox {
    sample_sizes: Vec<u32>,
}

impl ParseBox for SampleSizeBox {
    async fn parse(
        stream: &mut utils::BoxStream<impl tokio::io::AsyncReadExt + Unpin>,
        typ: utils::BoxType,
        mut size: usize,
    ) -> Result<Self, errors::Error> {
        let (version, _) = stream.read_box_version_flag_header().await?;

        let mut sample_sizes = Vec::new();
        if version == 0 {
            let sample_size = stream.read_u32().await?;
            let sample_count = stream.read_u32().await?;

            if sample_size != 0 {
                sample_sizes.resize(sample_count as usize, sample_size);
            } else {
                for _ in 0..sample_count {
                    sample_sizes.push(stream.read_u32().await?);
                }
            }
        }

        Ok(Self { sample_sizes })
    }
}
